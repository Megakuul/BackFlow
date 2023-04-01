use std::error::Error;

use ini::{Ini};

pub struct Configuration {
    pub redirects: Redirects,
    pub certificates: Certificates,
}

pub struct Redirects {
    pub path_enabled: String,
}

pub struct Certificates {
    pub path_certificates: String,
}

impl Configuration {

    pub fn new(config_path: &str) -> Self {

        let conf: Option<Ini> = match Ini::load_from_file(config_path) {
            Ok(config) => Some(config),
            Err(_) => None,
        };

        let redirects = Redirects {
            path_enabled: get_property(&conf, "path_enabled", "redirects", "/etc/backflow/se"),
        };

        let certificates: Certificates = Certificates { 
            path_certificates: get_property(&conf, "path_certificates", "certificates", "/etc/backflow/cert"),
        };

        return Self {redirects, certificates}
    }
}

/// Gets a property from .ini config
/// 
/// This function will not throw any errors, instead it will just use the default value
/// 
/// example:
/// ```
/// let conf: Option<Ini> = match Ini::load_from_file("/home/root/config.ini") {
///     Ok(config) => Some(config),
///     Err(_) => None,
/// };
/// 
/// String property = get_property(&conf, "property", "propertysection", "defaultvalue");
/// ```
fn get_property(config: &Option<Ini>, name: &str, section: &str, def: &str) -> String {
    let tmp_sec = config
        .as_ref()
        .and_then(|c| c.section(Some(section)));

    return tmp_sec
        .map_or(def.to_owned(), |c| c.get(name).unwrap_or(def).to_owned());
}