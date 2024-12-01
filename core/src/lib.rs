pub use config::Config;
pub use function::{Arg, ArgError, ArgParser, Args, Function};
pub use ssg::{InitError, LinSsg,BuildError};

mod function;
mod markdown;
mod config;
mod ssg;
