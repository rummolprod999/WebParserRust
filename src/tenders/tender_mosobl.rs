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

pub struct TenderMosobl<'a> {
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

impl<'a> WebTender for TenderMosobl<'a> {
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
        let mut query_res = (pool.prep_exec(r"SELECT id_tender FROM tender WHERE purchase_number = :purchase_number AND type_fz = :type_fz AND doc_publish_date = :doc_publish_date AND end_date = :end_date", params! {"purchase_number" => &self.pur_num, "type_fz" => &self.type_fz, "doc_publish_date" => &self.date_pub.naive_local(), "end_date" => &self.date_end.naive_local()}))?;
        if let Some(_) = query_res.next() {
            //info!("this tender exist in base, pur_num {}", &self.pur_num);
            return Ok((0, 0));
        };
        let (cancel_status, update) =
            (self.ret_cancel_status(&pool, self.type_fz, &self.pur_num, &date_upd))?;
        let id_organizer = (self.get_org_id(&pool))?;
        let id_placing_way = 0u64;
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
        let id_customer = 0u64;
        let id_lot = (self.get_lot_id(&pool, &id_tender))?;
        (self.insert_attachment(&pool, &id_tender))?;
        (self.insert_purchase_object(&pool, &id_lot, &id_customer))?;
        (self.add_version_num(&pool, self.type_fz, &self.pur_num))?;
        (self.add_tender_keywords(&pool, &id_tender))?;
        Ok((add_t, upd_t))
    }
}

impl<'a> TenderMosobl<'a> {
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
            let phone = "+7 (495) 909-81-91";
            let email = "";
            let inn = "7750005588";
            let post_address = "Российская Федерация, 107023, город Москва, улица Большая Семёновская, дом 32, строение 1";
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
        let res_insert = (pool.prep_exec("INSERT INTO tender SET id_xml = :id_xml, purchase_number = :purchase_number, doc_publish_date = :doc_publish_date, href = :href, purchase_object_info = :purchase_object_info, type_fz = :type_fz, id_organizer = :id_organizer, id_placing_way = :id_placing_way, id_etp = :id_etp, end_date = :end_date, cancel = :cancel, date_version = :date_version, num_version = :num_version, xml = :xml, print_form = :print_form, id_region = :id_region, notice_version = :notice_version", params! {"id_xml" => &self.pur_num, "purchase_number" => &self.pur_num, "doc_publish_date" => &self.date_pub.naive_local(), "href" => &self.href, "purchase_object_info" => &self.pur_name, "type_fz" => &self.type_fz, "id_organizer" => id_organizer, "id_placing_way" => id_placing_way, "id_etp" => id_etp, "end_date" => &self.date_end.naive_local(), "cancel" => cancel_status, "date_version" => &date_upd.naive_local(), "num_version" => 1, "xml" => &self.href, "print_form" => &self.href, "id_region" => 0, "notice_version" => ""}))?;
        return Ok(res_insert.last_insert_id());
    }

    fn get_lot_id(&self, pool: &my::Pool, id_tender: &u64) -> Result<u64, Box<dyn error::Error>> {
        let res_insert = (pool.prep_exec("INSERT INTO lot SET id_tender = :id_tender, lot_number = :lot_number, currency = :currency", params! {"id_tender" => id_tender, "lot_number" => 1, "currency" => ""}))?;
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
    fn insert_attachment(&self, pool: &my::Pool, id_tender: &u64) -> Result<(), Box<dyn error::Error>> {
        (pool.prep_exec("INSERT INTO attachment SET id_tender = :id_tender, file_name = :file_name, url = :url", params! {"id_tender" => id_tender, "file_name" => "Документация", "url" => &self.attach_url}))?;
        Ok(())
    }
}
