mod traits;
mod r#type;

pub use r#type::{not, Input, Input::*, Type, Type::*};
pub use traits::{AsRegex, Condition, Error, Result};
