use std::vec::IntoIter;

use crate::BundlerEnvoirment;

pub mod skip;

pub mod attributes {
    pub use crate::attributes::skip::Skip;
}

pub fn parse_attr_line(line: &str) -> Option<(String, String)> {
    let mut line = line.clone();
    if line.starts_with("--- @") {
        line = line.strip_prefix("--- @").unwrap();
    } else {
        return None;
    }

    let parts = line.split(" ").collect::<Vec<&str>>();

    if parts.len() == 2 {
        return Some((parts[0].to_string(), parts[1].to_string()));
    } else if parts.len() == 1 {
        return Some((parts[0].to_string(), String::new()));
    } else {
        return None;
    }
}

pub trait MarAttribute {
    fn execute(
        &self,
        contents: &mut IntoIter<String>,
        env: &mut BundlerEnvoirment,
    ) -> Result<(), String>;
}
