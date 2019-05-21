extern crate md5;
extern crate select;

use self::select::node::Find;
use self::select::node::Node;
use self::select::predicate::Predicate;
use std::error;

pub fn create_md5_str(inp: &str) -> String {
    let res = md5::compute(inp);
    let res_str = format!("{:?}", res);
    res_str
}

pub fn month_to_number(inp: &str) -> String {
    let lover = inp.to_lowercase();
    if lover.contains("янв") {
        return "01".to_string();
    } else if lover.contains("февр") {
        return "02".to_string();
    } else if lover.contains("март") {
        return "03".to_string();
    } else if lover.contains("апр") {
        return "04".to_string();
    } else if lover.contains("мая") {
        return "05".to_string();
    } else if lover.contains("июн") {
        return "06".to_string();
    } else if lover.contains("июль") {
        return "07".to_string();
    } else if lover.contains("авг") {
        return "08".to_string();
    } else if lover.contains("сент") {
        return "09".to_string();
    } else if lover.contains("окт") {
        return "10".to_string();
    } else if lover.contains("нояб") {
        return "11".to_string();
    } else if lover.contains("дек") {
        return "12".to_string();
    }
    return "".to_string();
}

pub fn replace_date_in_string(s: &str) -> String {
    if s.contains("января") {
        return s.replace("января", "01").replace(" ", ".");
    } else if s.contains("февраля") {
        return s.replace("февраля", "02").replace(" ", ".");
    } else if s.contains("марта") {
        return s.replace("марта", "03").replace(" ", ".");
    } else if s.contains("апреля") {
        return s.replace("апреля", "04").replace(" ", ".");
    } else if s.contains("мая") {
        return s.replace("мая", "05").replace(" ", ".");
    } else if s.contains("июня") {
        return s.replace("июня", "06").replace(" ", ".");
    } else if s.contains("июля") {
        return s.replace("июля", "07").replace(" ", ".");
    } else if s.contains("августа") {
        return s.replace("августа", "08").replace(" ", ".");
    } else if s.contains("сентября") {
        return s.replace("сентября", "09").replace(" ", ".");
    } else if s.contains("октября") {
        return s.replace("октября", "10").replace(" ", ".");
    } else if s.contains("ноября") {
        return s.replace("ноября", "11").replace(" ", ".");
    } else if s.contains("декабря") {
        return s.replace("декабря", "12").replace(" ", ".");
    }
    return "".to_string();
}
pub fn find_from_child_text<'a, 'b, T: Predicate>(
    find: &mut Find<'a, T>,
    err_str: &'a str,
    find_str: &'a str,
) -> Result<Node<'a>, Box<error::Error>> {
    while let Some(n) = find.next() {
        if n.text().contains(find_str) {
            return Ok(n);
        }
    }
    Err(::std::convert::From::from(err_str))
}

pub trait FindExt<P> {
    fn find_text_in_node(
        &self,
        p: P,
        err_str: &str,
        find_str: &str,
    ) -> Result<Node, Box<error::Error>>;
}

impl<'a, P: Predicate> FindExt<P> for Node<'a> {
    fn find_text_in_node(
        &self,
        p: P,
        err_str: &str,
        find_str: &str,
    ) -> Result<Node, Box<error::Error>> {
        let mut res = self.find(p);
        while let Some(n) = res.next() {
            if n.text().contains(find_str) {
                return Ok(n);
            }
        }
        Err(::std::convert::From::from(err_str))
    }
}
