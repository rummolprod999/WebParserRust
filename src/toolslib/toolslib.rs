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
