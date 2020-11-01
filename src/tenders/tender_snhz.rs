extern crate chrono;
extern crate mysql;
extern crate select;
extern crate uuid;

use self::my::Value;
use self::mysql as my;
use self::uuid::Uuid;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools::DateTimeTools;
use chrono::prelude::*;
use chrono::DateTime;
use std::error;

pub struct TenderSnHz<'a> {
    pub type_fz: i32,
    pub etp_name: String,
    pub etp_url: String,
    pub href: String,
    pub pur_num: String,
    pub pur_name: String,
    pub org_name: String,
    pub pw_name: String,
    pub date_pub: DateTime<FixedOffset>,
    pub date_end: DateTime<FixedOffset>,
    pub connect_string: &'a String,
}

impl<'a> WebTender for TenderSnHz<'a> {
    fn parser(&self) -> (i32, i32) {
        let res = match self.parser_unwrap() {
            Ok(v) => v,
            Err(e) => {
                warn!("{} {}", e, e.to_string());
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
        let mut query_res = (pool.prep_exec(r"SELECT id_tender FROM tender WHERE purchase_number = :purchase_number AND type_fz = :type_fz AND doc_publish_date = :doc_publish_date AND end_date = :end_date", params! {"purchase_number" => &self.pur_num, "type_fz" => &self.type_fz, "doc_publish_date" => &self.date_pub.naive_local(), "end_date" => &self.date_end.naive_local()}))?;
        if let Some(_) = query_res.next() {
            //info!("this tender exist in base, pur_num {}", &self.pur_num);
            return Ok((0, 0));
        };
        let (cancel_status, update) =
            (self.ret_cancel_status(&pool, self.type_fz, &self.pur_num, &date_upd))?;
        Ok((add_t, upd_t))
    }
}
