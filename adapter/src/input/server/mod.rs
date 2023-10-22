mod api;
mod error;
mod middleware;
mod response;
mod system;

use self::error::Error;
use self::error::Result;
pub use self::system::Status;
pub use crate::input::server::api::server as api_server;
pub use crate::input::server::system::server as system_server;
