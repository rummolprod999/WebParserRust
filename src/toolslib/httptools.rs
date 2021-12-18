use reqwest::header::{COOKIE, USER_AGENT};
use reqwest::ClientBuilder;
use std::error::Error;
use std::io::Read;
use std::option::Option;
use std::process::Command;
use std::process::Stdio;

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
                    warn!("{} {}", e, e.to_string());
                }
            }
        }
        s
    }

    pub fn get_page_text_ua(url: &str) -> Option<String> {
        let mut s: Option<String> = None;
        let mut i = 5;
        while i >= 0 {
            let res = HttpTools::try_get_page_ua(url);
            match res {
                Ok(r) => {
                    s = Some(r);
                    break;
                }
                Err(e) => {
                    i -= 1;
                    warn!("{} {}", e, e.to_string());
                }
            }
        }
        s
    }

    pub fn get_page_text_no_ssl(url: &str) -> Option<String> {
        let mut s: Option<String> = None;
        let mut i = 5;
        while i >= 0 {
            let res = HttpTools::try_get_page_no_ssl(url);
            match res {
                Ok(r) => {
                    s = Some(r);
                    break;
                }
                Err(e) => {
                    i -= 1;
                    warn!("{} {}", e, e.to_string());
                }
            }
        }
        s
    }

    pub fn try_get_page(url: &str) -> Result<String, Box<dyn Error>> {
        let mut res = reqwest::get(url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok(body)
    }

    pub fn try_get_page_ua(url: &str) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let mut res = client
            .get(url)
            .header(COOKIE, "ds_phpsid=a6fcm1mgr5mnqmkdifjja3bes0; adspire_uid=AS.1009301133.1616942617; search_enable=0; ds_ssid=qbh6ahqits1j8m9hsmu7hqffc0; DS_SM_SALE_UID=9751638678; Front2021=vetus90; sprf=AAAAAGG9cBVCCSmOBG4UAg==; spid=1639804949278_94f4b694192f6d0b8c9f5a7c445919b9_kq85l3q3ol6a6r9l; spsc=1639804949278_3e9db0e873b317507445f434e8a8785c_a5476469b72f558bb72e6aae99c6a060; rrpvid=595705449592927")
            .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.56 Safari/537.36")
            .send()?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok(body)
    }

    pub fn try_get_page_no_ssl(url: &str) -> Result<String, Box<dyn Error>> {
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()?;
        let mut res = client.get(url).send()?;
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
                    warn!("{} {}", e, e.to_string());
                }
            }
        }
        s
    }

    pub fn try_get_page1251(url: &str) -> Result<String, Box<dyn Error>> {
        let mut res = reqwest::get(url)?;
        let x = res.text()?;
        Ok(x)
    }

    pub fn get_page_from_wget_1251(url: &str) -> Result<String, Box<dyn Error>> {
        let output = Command::new("wget")
            .args(&[
                "--header='Accept-Charset: windows-1251'",
                "-q",
                "-O",
                "-",
                url,
            ])
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or("error in child process wget")?;
        let out = Command::new("iconv")
            .args(&["-f", "cp1251"])
            .stdin(output)
            .output()?
            .stdout;
        let s = String::from_utf8(out)?;
        Ok(s)
    }
}
