extern crate md5;
extern crate select;

use std::error;
use self::select::predicate::Predicate;
use self::select::node::Find;
use self::select::node::Node;

pub fn create_md5_str(inp: &str) -> String {
    let res = md5::compute(inp);
    let res_str = format!("{:?}", res);
    res_str
}

pub fn find_from_child_text<'a, 'b, T: Predicate>(find: &mut Find<'a, T>, err_str: &'a str, find_str: &'a str) -> Result<Node<'a>, Box<error::Error>> {
    while let Some(n) = find.next() {
        if n.text().contains(find_str) {
            return Ok(n);
        }
    }
    Err(::std::convert::From::from(err_str))
}

pub trait FindExt<P> {
    fn find_text_in_node(&self, p: P, err_str: &str, find_str: &str) -> Result<Node, Box<error::Error>>;
}

impl<'a, P: Predicate> FindExt<P> for Node<'a> {
    fn find_text_in_node(&self, p: P, err_str: &str, find_str: &str) -> Result<Node, Box<error::Error>> {
        let mut res = self.find(p);
        while let Some(n) = res.next() {
            if n.text().contains(find_str) {
                return Ok(n);
            }
        }
        Err(::std::convert::From::from(err_str))
    }
}