extern crate regex;

use self::regex::Regex;

pub struct RegexTools {}

impl RegexTools {
    pub fn get_one_group(s: &str, pattern: &str) -> Option<String> {
        let re = Regex::new(pattern);
        let result = match re {
            Ok(r) => r,
            Err(e) => {
                warn!("{}", e);
                return None;
            }
        };
        let cap = match result.captures(s) {
            Some(t) => t,
            None => return None
        };
        let rz = if cap.len() > 1 {
            let r = &cap[1];
            Some(r.to_string())
        } else {
            None
        };
        rz
    }

    pub fn del_double_ws(s: &String) -> Option<String> {
        let pat = r"\s+";
        let re = Regex::new(pat);
        let result = match re {
            Ok(r) => r,
            Err(e) => {
                warn!("{}", e);
                return None;
            }
        };
        let res_string = result.replace_all(s, " ");
        Some(res_string.to_string())
    }
}