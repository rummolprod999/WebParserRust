extern crate chrono;
extern crate mysql;
extern crate select;
extern crate uuid;

use self::my::Value;
use self::mysql as my;
use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use crate::parsers::parsers::Attachment;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools::DateTimeTools;
use crate::toolslib::httptools;
use crate::toolslib::regextools;
use chrono::prelude::*;
use chrono::DateTime;
use std::error;

pub struct TenderNornic<'a> {
    pub type_fz: i32,
    pub etp_name: String,
    pub etp_url: String,
    pub href: String,
    pub pur_num: String,
    pub pur_name: String,
    pub cus_name: String,
    pub date_pub: DateTime<FixedOffset>,
    pub connect_string: &'a String,
}

impl<'a> WebTender for TenderNornic<'a> {
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

    fn parser_unwrap(&self) -> Result<(i32, i32), Box<error::Error>> {
        let date_upd = DateTimeTools::return_datetime_now();
        let mut add_t = 0;
        let mut upd_t = 0;
        let pool = (my::Pool::new(self.connect_string))?;
        let mut query_res = (pool.prep_exec(r"SELECT id_tender FROM tender WHERE purchase_number = :purchase_number AND type_fz = :type_fz AND doc_publish_date = :doc_publish_date", params! {"purchase_number" => &self.pur_num, "type_fz" => &self.type_fz, "doc_publish_date" => &self.date_pub.naive_local()}))?;
        if let Some(_) = query_res.next() {
            //info!("this tender exist in base, pur_num {}", &self.pur_num);
            return Ok((0, 0));
        };
        let page = httptools::HttpTools::get_page_text1251(&self.href)
            .ok_or(format!("can not download page {}", &self.href))?;
        let document = Document::from(&*page);
        let (cancel_status, update) =
            (self.ret_cancel_status(&pool, self.type_fz, &self.pur_num, &date_upd))?;
        let id_organizer = (self.get_org_id(&pool))?;
        let id_placing_way = 0u64;
        let id_etp = (self.get_etp_id(&pool, &self.etp_name, &self.etp_url))?;
        Ok((add_t, upd_t))
    }
}

impl<'a> TenderNornic<'a> {
    fn get_org_id(&self, pool: &my::Pool) -> Result<u64, Box<error::Error>> {
        let mut res = (pool.prep_exec(
            "SELECT id_organizer FROM organizer WHERE full_name = :full_name",
            params! {"full_name" => &self.etp_name},
        ))?;
        if let Some(org_row_t) = res.next() {
            let id_org: u64 = (org_row_t)?
                .get_opt::<Value, usize>(0)
                .ok_or("bad id_organizer")?
                .and_then(|x| my::from_value_opt::<u64>(x))?;
            return Ok(id_org);
        } else {
            let phone = "+7 (3919) 258001";
            let email = "nord@nk.nornik.ru";
            let inn = "8401005730";
            let post_address = "";
            let cont_person = "";
            let res_insert = (pool.prep_exec("INSERT INTO organizer SET full_name = :full_name, contact_person = :contact_person, contact_phone = :contact_phone, contact_email = :contact_email, inn = :inn, post_address = :post_address", params! {"full_name" => &self.etp_name, "contact_person" => cont_person, "contact_phone" => phone, "contact_email" => email, "inn" => inn, "post_address" => post_address}))?;
            return Ok(res_insert.last_insert_id());
        }
    }
}
