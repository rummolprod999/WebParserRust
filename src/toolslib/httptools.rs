use std::error::Error;
use std::io::Read;
use std::option::Option;

pub struct HttpTools {}

impl HttpTools {
    pub fn get_page_text(url: &str) -> Option<String> {
        let mut s: Option<String> = None;
        let mut i = 5;
        while i >= 0 {
            let res = HttpTools::try_get_page(url);
            match res {
                Ok(r) => {
                    s = Some(r);
                    break;
                }
                Err(e) => {
                    i -= 1;
                    warn!("{} {}", e, e.description());
                }
            }
        }
        s
    }

    pub fn try_get_page(url: &str) -> Result<String, Box<Error>> {
        let mut res = reqwest::get(url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok(body)
    }

    pub fn get_page_text1251(url: &str) -> Option<String> {
        let mut s: Option<String> = None;
        let mut i = 5;
        while i >= 0 {
            let res = HttpTools::try_get_page1251(url);
            match res {
                Ok(r) => {
                    s = Some(r);
                    break;
                }
                Err(e) => {
                    i -= 1;
                    warn!("{} {}", e, e.description());
                }
            }
        }
        s
    }

    pub fn try_get_page1251(url: &str) -> Result<String, Box<Error>> {
        let mut res = reqwest::get(url)?;
        let x = res.text()?;
        Ok(x)
    }
}
