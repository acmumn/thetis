use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use failure::{Error, SyncFailure};
use tera::Tera;
use url::Url;

use auth::capabilities::Rules;
use DB;

/// The "globalish" values used by the server. Cheaply clonable.
#[derive(Clone)]
pub struct Context {
    /// The base URL of the site.
    pub(crate) base_url: Arc<Url>,

    /// The currently loaded rules.
    pub(crate) capabilities: Arc<RwLock<Rules>>,

    /// The file that contains rules.
    capabilities_file: Arc<Path>,

    /// A pool of connections to the database.
    pub db: DB,

    /// The secret to use for signing JWTs.
    pub(crate) jwt_secret: Arc<str>,

    /// The web templates.
    pub templates: Arc<RwLock<Tera>>,
}

impl Context {
    /// Creates a new Context.
    pub fn new(
        base_url: Url,
        database_url: &str,
        jwt_secret: String,
        capabilities_file: PathBuf,
        mut template_dir: String,
    ) -> Result<Context, Error> {
        let base_url = Arc::new(base_url);
        let capabilities_file = Arc::from(capabilities_file);
        let jwt_secret = Arc::from(jwt_secret);

        let capabilities = Rules::load_from(&capabilities_file)?;
        let capabilities = Arc::new(RwLock::new(capabilities));

        template_dir.push_str("/**/*");
        let templates = Tera::new(&template_dir).map_err(SyncFailure::new)?;
        let templates = Arc::new(RwLock::new(templates));

        let db = DB::connect(database_url)?;

        Ok(Context {
            base_url,
            capabilities,
            capabilities_file,
            db,
            jwt_secret,
            templates,
        })
    }
}
