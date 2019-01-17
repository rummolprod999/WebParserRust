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
            None => return None,
        };
        let rz = if cap.len() > 1 {
            let r = &cap[1];
            Some(r.to_string())
        } else {
            None
        };
        rz
    }
    pub fn get_one_group_fix(s: &str, pattern: &str) -> Option<String> {
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
            None => return None,
        };
        let rz = if cap.len() > 2 {
            let r = &cap.get(1).map(|m| m.as_str()).unwrap_or("");
            if *r == "" {
                let d = &cap.get(2).map(|m| m.as_str()).unwrap_or("");
                Some(d.to_string())
            } else {
                Some(r.to_string())
            }
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

    pub fn del_all_ws(s: &String) -> Option<String> {
        let pat = r"\s+";
        let re = Regex::new(pat);
        let result = match re {
            Ok(r) => r,
            Err(e) => {
                warn!("{}", e);
                return None;
            }
        };
        let res_string = result.replace_all(s, "");
        Some(res_string.to_string())
    }
}
