extern crate chrono;
extern crate mysql;
extern crate select;
extern crate uuid;

use self::my::Value;
use self::mysql as my;
use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Name, Predicate};
use self::uuid::Uuid;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools::DateTimeTools;
use crate::toolslib::httptools;
use chrono::prelude::*;
use chrono::DateTime;
use std::{error, thread, time};

pub struct TenderTgk14<'a> {
    pub type_fz: i32,
    pub etp_name: String,
    pub etp_url: String,
    pub href: String,
    pub pur_num: String,
    pub pur_name: String,
    pub pw_name: String,
    pub nmck: String,
    pub date_pub: DateTime<FixedOffset>,
    pub date_end: DateTime<FixedOffset>,
    pub status: String,
    pub connect_string: &'a String,
}

impl<'a> WebTender for TenderTgk14<'a> {
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
        let mut query_res = (pool.prep_exec(r"SELECT id_tender FROM tender WHERE purchase_number = :purchase_number AND type_fz = :type_fz AND doc_publish_date = :doc_publish_date AND end_date = :end_date AND notice_version = :notice_version", params! {"purchase_number" => &self.pur_num, "type_fz" => &self.type_fz, "doc_publish_date" => &self.date_pub.naive_local(), "end_date" => &self.date_end.naive_local(), "notice_version" => &self.status}))?;
        if let Some(_) = query_res.next() {
            //info!("this tender exist in base, pur_num {}", &self.pur_num);
            return Ok((0, 0));
        };
        let ten_millis = time::Duration::from_millis(5 * 1000);
        thread::sleep(ten_millis);

        let page = httptools::HttpTools::get_page_text_ua(&self.href)
            .ok_or(format!("cannot download page {}", &self.href))?;
        let document = Document::from(&*page);
        let (cancel_status, update) =
            (self.ret_cancel_status(&pool, self.type_fz, &self.pur_num, &date_upd))?;
        let id_organizer = (self.get_org_id(&pool))?;
        let id_placing_way = (self.get_placing_way_id(&pool, &self.pw_name))?;
        let id_etp = (self.get_etp_id(&pool, &self.etp_name, &self.etp_url))?;
        let id_tender = (self.get_tender_id(
            &pool,
            &id_organizer,
            &id_placing_way,
            &id_etp,
            &date_upd,
            &cancel_status,
        ))?;
        if update {
            upd_t = 1;
        } else {
            add_t = 1;
        }
        let id_customer = (self.get_cus_id(&pool))?;
        let id_lot = (self.get_lot_id(&pool, &id_tender))?;
        (self.insert_attachment(&pool, &id_tender, &document))?;
        (self.insert_purchase_object(&pool, &id_lot, &id_customer))?;
        (self.add_version_num(&pool, self.type_fz, &self.pur_num))?;
        (self.add_tender_keywords(&pool, &id_tender))?;
        Ok((add_t, upd_t))
    }
}
impl<'a> TenderTgk14<'a> {
    fn get_org_id(&self, pool: &my::Pool) -> Result<u64, Box<dyn error::Error>> {
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
            let phone = "(3022) 38-45-50";
            let email = "office@chita.tgk-14.com";
            let inn = "7534018889";
            let post_address = "672000, г. Чита, ул. Профсоюзная, д. 23.";
            let cont_person = "";
            let res_insert = (pool.prep_exec("INSERT INTO organizer SET full_name = :full_name, contact_person = :contact_person, contact_phone = :contact_phone, contact_email = :contact_email, inn = :inn, post_address = :post_address", params! {"full_name" => &self.etp_name, "contact_person" => cont_person, "contact_phone" => phone, "contact_email" => email, "inn" => inn, "post_address" => post_address}))?;
            return Ok(res_insert.last_insert_id());
        }
    }

    fn get_tender_id(
        &self,
        pool: &my::Pool,
        id_organizer: &u64,
        id_placing_way: &u64,
        id_etp: &u64,
        date_upd: &DateTime<FixedOffset>,
        cancel_status: &i32,
    ) -> Result<u64, Box<dyn error::Error>> {
        let res_insert = (pool.prep_exec("INSERT INTO tender SET id_xml = :id_xml, purchase_number = :purchase_number, doc_publish_date = :doc_publish_date, href = :href, purchase_object_info = :purchase_object_info, type_fz = :type_fz, id_organizer = :id_organizer, id_placing_way = :id_placing_way, id_etp = :id_etp, end_date = :end_date, cancel = :cancel, date_version = :date_version, num_version = :num_version, xml = :xml, print_form = :print_form, id_region = :id_region, notice_version = :notice_version", params! {"id_xml" => &self.pur_num, "purchase_number" => &self.pur_num, "doc_publish_date" => &self.date_pub.naive_local(), "href" => &self.href, "purchase_object_info" => &self.pur_name, "type_fz" => &self.type_fz, "id_organizer" => id_organizer, "id_placing_way" => id_placing_way, "id_etp" => id_etp, "end_date" => &self.date_end.naive_local(), "cancel" => cancel_status, "date_version" => &date_upd.naive_local(), "num_version" => 1, "xml" => &self.href, "print_form" => &self.href, "id_region" => 0, "notice_version" => &self.status}))?;
        return Ok(res_insert.last_insert_id());
    }

    fn get_cus_id(&self, pool: &my::Pool) -> Result<u64, Box<dyn error::Error>> {
        let mut res = (pool.prep_exec(
            "SELECT id_customer FROM customer WHERE full_name = :full_name",
            params! {"full_name" => &self.etp_name},
        ))?;
        if let Some(id_cus_row) = res.next() {
            let id_cus = (id_cus_row)?
                .get_opt::<Value, usize>(0)
                .ok_or("None id_customer")?
                .and_then(|x| my::from_value_opt::<u64>(x))?;
            return Ok(id_cus);
        } else {
            let inn = "7534018889";
            let reg_num = Uuid::new_v4().hyphenated().to_string();
            let res_insert = (pool.prep_exec("INSERT INTO customer SET full_name = :full_name, reg_num = :reg_num, is223=1, inn = :inn", params! {"full_name" => &self.etp_name, "reg_num" => reg_num, "inn" => inn}))?;
            return Ok(res_insert.last_insert_id());
        }
    }
    fn get_lot_id(&self, pool: &my::Pool, id_tender: &u64) -> Result<u64, Box<dyn error::Error>> {
        let res_insert = (pool.prep_exec("INSERT INTO lot SET id_tender = :id_tender, lot_number = :lot_number, currency = :currency, max_price = :max_price", params! {"id_tender" => id_tender, "lot_number" => 1, "currency" => "", "max_price" => &self.nmck}))?;
        return Ok(res_insert.last_insert_id());
    }
    fn insert_purchase_object(
        &self,
        pool: &my::Pool,
        id_lot: &u64,
        id_customer: &u64,
    ) -> Result<(), Box<dyn error::Error>> {
        (pool.prep_exec("INSERT INTO purchase_object SET id_lot = :id_lot, id_customer = :id_customer, name = :name", params! {"id_lot" => id_lot, "id_customer" => id_customer, "name" => &self.pur_name}))?;
        Ok(())
    }

    fn insert_attachment(
        &self,
        pool: &my::Pool,
        id_tender: &u64,
        doc: &Document,
    ) -> Result<(), Box<dyn error::Error>> {
        let attachments = doc.find(Name("a").and(|x: &Node| {
            if x.attr("href")
                .map_or("".to_string(), |x| x.to_string())
                .contains("/upload/")
            {
                true
            } else {
                false
            }
        }));
        for att in attachments {
            let url_att_t = att.attr("href").map_or("".to_string(), |x| x.to_string());
            let url_at = format!("https://zakupki.tgk-14.com{}", url_att_t);
            let att_name = att.text();
            let att_name = att_name.trim();
            (pool.prep_exec("INSERT INTO attachment SET id_tender = :id_tender, file_name = :file_name, url = :url", params! {"id_tender" => id_tender, "file_name" => &att_name, "url" => &url_at}))?;
        }
        Ok(())
    }
}
