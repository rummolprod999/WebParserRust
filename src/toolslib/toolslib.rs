extern crate md5;

pub fn create_md5_str(inp: &str) -> String {
    let res = md5::compute(inp);
    let res_str = format!("{:?}", res);
    res_str
}