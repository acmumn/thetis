extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate fern;
extern crate hostname;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;
extern crate syslog;
extern crate thetis;
extern crate tokio;
extern crate tokio_threadpool;
extern crate url;

use std::net::{SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::process::exit;

use failure::Error;
use structopt::StructOpt;
use thetis::{util::log_err, web::serve_on, Context};
use tokio_threadpool::ThreadPool;
use url::Url;

fn main() {
    dotenv::dotenv().ok();
    let options = Options::from_args();
    if let Err(err) = options.start_logger() {
        error!("Warning: logging couldn't start: {}", err);
    }

    if let Err(err) = run(options) {
        log_err(err);
        exit(1);
    }
}

fn run(options: Options) -> Result<(), Error> {
    let serve_addr = options.serve_addr()?;

    /*
    let mailer = Mailer::new(
        options.smtp_addr,
        options.smtp_from,
        options.smtp_user,
        options.smtp_pass,
        options.smtp_reply_to,
    )?;
    */

    let context = Context::new(
        options.base_url,
        &options.database_url,
        options.jwt_secret,
        options.capabilities_file,
        options.template_dir,
    )?;
    let server = serve_on(serve_addr, context.clone());
    let thread_pool = ThreadPool::new();
    thread_pool.spawn(server);

    /*
    let sweeper = Interval::new(Instant::now(), Duration::from_secs(5 * 60))
        .map_err(Error::from)
        .for_each(move |_| {
            let fut = sweep(db.clone(), mailer.clone(), base_url.clone());
            Ok(thread_pool.spawn(fut.map_err(|e| log_err(e.into()))))
        })
        .map_err(log_err);

    tokio::run(sweeper);
    */
    tokio::run(thread_pool.shutdown_on_idle());
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "::structopt::clap::AppSettings::ColoredHelp"))]
struct Options {
    /// Turns off message output. Passing once prevents logging to syslog. Passing twice or more
    /// disables all logging.
    #[structopt(short = "q", long = "quiet", parse(from_occurrences))]
    quiet: usize,

    /// Increases the verbosity. Default verbosity is warnings and higher to syslog, info and
    /// higher to the console.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,

    /// The base URL for unsubscribe links and template examples.
    #[structopt(long = "base-url", env = "BASE_URL")]
    base_url: Url,

    /// The file to load capability rules from.
    #[structopt(
        short = "c",
        long = "caps",
        env = "CAPABILITIES_FILE",
        default_value = "capabilities.pl",
        parse(from_os_str)
    )]
    capabilities_file: PathBuf,

    /// The URL of the MySQL database.
    #[structopt(long = "db", env = "DATABASE_URL")]
    database_url: String,

    /// The host to serve on.
    #[structopt(
        short = "H",
        long = "host",
        env = "HOST",
        default_value = "::"
    )]
    host: String,

    /// The JWT HS512 secret.
    #[structopt(long = "jwt-secret", env = "JWT_SECRET")]
    jwt_secret: String,

    /// The port to serve on.
    #[structopt(
        short = "P",
        long = "port",
        env = "PORT",
        default_value = "8001"
    )]
    port: u16,

    /// The SMTP server to use.
    #[structopt(
        long = "smtp-addr",
        env = "SMTP_ADDR",
        default_value = "smtp.gmail.com"
    )]
    smtp_addr: String,

    /// The SMTP From header to use.
    #[structopt(long = "smtp-from", env = "SMTP_FROM")]
    smtp_from: String,

    /// The SMTP username to use.
    #[structopt(long = "smtp-user", env = "SMTP_USER")]
    smtp_user: String,

    /// The SMTP password to use.
    #[structopt(long = "smtp-pass", env = "SMTP_PASS")]
    smtp_pass: String,

    /// The SMTP Reply-To header to use.
    #[structopt(long = "smtp-reply-to", env = "SMTP_REPLY_TO")]
    smtp_reply_to: Option<String>,

    /// The syslog server to send logs to.
    #[structopt(short = "s", long = "syslog-server", env = "SYSLOG_SERVER")]
    syslog_server: Option<String>,

    /// The directory to load web templates from.
    #[structopt(
        short = "t",
        long = "templates",
        env = "TEMPLATE_DIR",
        default_value = "templates"
    )]
    template_dir: String,
}

impl Options {
    /// Get the address to serve on.
    fn serve_addr(&self) -> Result<SocketAddr, Error> {
        let addrs = (&self.host as &str, self.port)
            .to_socket_addrs()?
            .collect::<Vec<_>>();
        if addrs.is_empty() {
            bail!("No matching address exists")
        } else {
            Ok(addrs[0])
        }
    }

    /// Sets up logging as specified by the `-q`, `-s`, and `-v` flags.
    fn start_logger(&self) -> Result<(), Error> {
        use fern::Dispatch;
        use log::LevelFilter;

        if self.quiet >= 2 {
            return Ok(());
        }

        let (console_ll, syslog_ll) = match self.verbose {
            0 => (LevelFilter::Info, LevelFilter::Warn),
            1 => (LevelFilter::Debug, LevelFilter::Info),
            2 => (LevelFilter::Trace, LevelFilter::Debug),
            _ => (LevelFilter::Trace, LevelFilter::Trace),
        };

        let fern = Dispatch::new().chain(
            Dispatch::new()
                .level(console_ll)
                .format(move |out, message, record| {
                    out.finish(format_args!("[{}] {}", record.level(), message))
                }).chain(std::io::stderr()),
        );

        let fern = if self.quiet == 0 {
            let formatter = syslog::Formatter3164 {
                facility: syslog::Facility::LOG_DAEMON,
                hostname: hostname::get_hostname(),
                process: "thetis".to_owned(),
                pid: ::std::process::id() as i32,
            };

            let syslog = if let Some(ref server) = self.syslog_server {
                syslog::tcp(formatter, server).map_err(failure::SyncFailure::new)?
            } else {
                syslog::unix(formatter.clone())
                    .or_else(|_| syslog::tcp(formatter.clone(), ("127.0.0.1", 601)))
                    .or_else(|_| {
                        syslog::udp(formatter.clone(), ("127.0.0.1", 0), ("127.0.0.1", 514))
                    }).map_err(failure::SyncFailure::new)?
            };

            fern.chain(Dispatch::new().level(syslog_ll).chain(syslog))
        } else {
            fern
        };

        fern.apply()?;
        Ok(())
    }
}
