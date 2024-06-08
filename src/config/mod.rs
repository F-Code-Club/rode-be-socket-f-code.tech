mod cors;
mod google_api;
mod jwt;
mod misc;
mod server;

pub use cors::*;
pub use google_api::*;
pub use jwt::*;
pub use misc::*;
pub use server::*;

use std::{env, str::FromStr};

fn env_or_default<T: FromStr>(env_name: &'static str, default: T) -> T {
    match env::var(env_name) {
        Err(_) => default,
        Ok(raw) => match raw.parse() {
            Ok(value) => value,
            Err(_) => default,
        },
    }
}
