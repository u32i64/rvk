#![doc(html_logo_url = "https://raw.githubusercontent.com/u32i64/rvk/master/logo.png")]

//! # Overview
//! This is a crate for accessing VK API (synchronously).
//!
//! It consists of:
//!
//! - [`api`](api/index.html) **module**, which works with the API;
//! - [`error`](error/index.html) **module**, which handles errors that may occur during an API call;
//! - [`methods`](methods/index.html) **module**, which contains **API [methods](https://vk.com/dev/methods)**;
//! - [`objects`](objects/index.html) **module**, which contains **API [objects](https://vk.com/dev/objects)**,
//!
//! # Example
//! ```no_run
//! use rvk::{methods::users, objects::user::User, APIClient, Params};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut api = APIClient::new("your_access_token"); // Create an API Client
//!
//!     let mut params = Params::new(); // Create a HashMap to store parameters
//!     params.insert("user_ids".into(), "1".into());
//!
//!     let res = users::get::<Vec<User>>(&api, params).await;
//!
//!     match res {
//!         Ok(users) => {
//!             let user: &User = &users[0];
//!
//!             println!(
//!                 "User #{} is {} {}.",
//!                 user.id, user.first_name, user.last_name
//!             );
//!         }
//!         Err(e) => println!("{}", e),
//!     };
//! }
//! ```

pub mod api;
pub mod error;
pub mod methods;
pub mod objects;

pub use crate::api::{APIClient, Params};

/// Defines the version of VK API that is used.
pub const API_VERSION: &str = "5.103";
