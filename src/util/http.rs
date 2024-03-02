use crate::built_info;
use const_format::formatcp;
use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    pub static ref CLIENT: Client = Client::new();
}

pub const USER_AGENT: &str = formatcp!(
    "{}/{} ({})",
    built_info::PKG_NAME,
    built_info::PKG_VERSION,
    built_info::PKG_HOMEPAGE
);
