//! Miscellaneous utilities.

use std::sync::atomic::{AtomicUsize, Ordering};

use failure;

/// Generates a new number.
pub fn gensym() -> usize {
    lazy_static! {
        static ref N: AtomicUsize = AtomicUsize::new(0);
    }
    N.fetch_add(1, Ordering::SeqCst)
}

/// Logs an error, including its causes and backtrace (if possible).
pub fn log_err(err: failure::Error) {
    let mut first = true;
    let num_errs = err.iter_chain().count();
    if num_errs <= 1 {
        error!("{}", err);
    } else {
        for cause in err.iter_chain() {
            if first {
                first = false;
                error!("           {}", cause);
            } else {
                error!("caused by: {}", cause);
            }
        }
    }
    let bt = err.backtrace().to_string();
    if bt != "" {
        error!("{}", bt);
    }
}
