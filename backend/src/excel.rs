//! Helper stuff for talking to Office 365 Excel over HTTP
//!
//! It'S a DaTaBaSe

use std::env;

/// Add a timestamp
pub fn add_timestamp() -> Result<(), ()> {
    let auth_token =
        env::var("MS_AUTH_TOKEN").map_err(|e| error!("Couldn't get MS API token: {}", e))?;

    unimplemented!()
}
