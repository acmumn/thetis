//! Miscellaneous utilities.

use std::sync::atomic::{AtomicUsize, Ordering};

use failure;
use futures::Stream;

/// Boxes a stream, since `.boxed()` is apparently deprecated. (It makes sense to deprecate the
/// futures version, since `Either` exists, but since there's no Stream `Either`...) This is mainly
/// a way to give a hint to type inference that we want a trait object.
pub fn box_stream<E, S, T>(stream: S) -> Box<dyn Stream<Item = T, Error = E> + Send>
where
    S: 'static + Stream<Item = T, Error = E> + Send,
{
    Box::new(stream)
}

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
