#![deny(unused_features)]
#![deny(deprecated)]
#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]
#![warn(missing_copy_implementations)]

pub use crate::requests::{Request, RequestReceiver};
pub use crate::server::{new, TestServer};
pub use actix_web::{HttpRequest, HttpResponse};
use crossbeam_channel as channel;

pub mod helper;
mod middleware;
mod requests;
mod server;
