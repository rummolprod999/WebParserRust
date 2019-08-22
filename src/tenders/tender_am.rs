extern crate chrono;
extern crate mysql;
extern crate select;
extern crate uuid;

use self::my::Value;
use self::mysql as my;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools::DateTimeTools;
use chrono::prelude::*;
use chrono::DateTime;
use std::error;

pub struct TenderAm<'a> {
    pub type_fz: i32,
    pub etp_name: String,
    pub etp_url: String,
    pub href: &'a String,
    pub pur_num: String,
    pub pur_name: String,
    pub date_pub: DateTime<FixedOffset>,
    pub date_end: DateTime<FixedOffset>,
    pub attach_url: &'a String,
    pub connect_string: &'a String,
}

impl<'a> WebTender for TenderAm<'a> {
    fn parser(&self) -> (i32, i32) {
        let res = match self.parser_unwrap() {
            Ok(v) => v,
            Err(e) => {
                warn!("{} {}", e, e.description());
                (0, 0)
            }
        };
        res
    }
    fn parser_unwrap(&self) -> Result<(i32, i32), Box<dyn error::Error>> {
        let date_upd = DateTimeTools::return_datetime_now();
        let mut add_t = 0;
        let mut upd_t = 0;
        let pool = (my::Pool::new(self.connect_string))?;
        Ok((add_t, upd_t))
    }
}

impl<'a> TenderAm<'a> {}
