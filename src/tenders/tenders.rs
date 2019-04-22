extern crate chrono;
extern crate mysql;
extern crate mysql_common;

use self::chrono::FixedOffset;
use self::chrono::{DateTime, NaiveDateTime};
use self::my::Value;
use self::mysql as my;
use crate::toolslib::regextools;
use std::cmp::Ordering;
use std::error;

pub trait WebTender {
    fn parser(&self) -> (i32, i32);
    fn parser_unwrap(&self) -> Result<(i32, i32), Box<error::Error>>;

    fn ret_cancel_status(
        &self,
        pool: &my::Pool,
        type_fz: i32,
        pur_num: &String,
        date_upd: &DateTime<FixedOffset>,
    ) -> Result<(i32, bool), Box<error::Error>> {
        let d_upd = &date_upd.naive_local();
        let mut cancel = 0;
        let mut upd = false;
        let res = (pool.prep_exec("SELECT id_tender, date_version FROM tender WHERE purchase_number = :purchase_number AND cancel=0 AND type_fz = :type_fz", params! {"purchase_number" => pur_num, "type_fz" => type_fz}))?;
        for r in res.into_iter() {
            match r {
                Err(e) => {
                    warn!("{}", e);
                    return Err(::std::convert::From::from(e));
                }
                Ok(mut res) => {
                    upd = true;
                    let id_tender: i64 = match res.get_opt::<Value, usize>(0) {
                        None => return Err(::std::convert::From::from("no result id_tender")),
                        Some(idt) => {
                            let idr = (idt)?;
                            let tt = my::from_value_opt::<i64>(idr);
                            let m = (tt)?;
                            m
                        }
                    };
                    let date_version: NaiveDateTime = match res.get_opt::<Value, usize>(1) {
                        None => return Err(::std::convert::From::from("no result date_version")),
                        Some(idt) => {
                            let idr = (idt)?;
                            let tt = my::from_value_opt::<NaiveDateTime>(idr);
                            let m = (tt)?;
                            m
                        }
                    };
                    match d_upd.cmp(&date_version) {
                        Ordering::Greater | Ordering::Equal => {
                            let _res_upd = (pool.prep_exec(
                                "UPDATE tender SET cancel=1 WHERE id_tender = :id_tender",
                                params! {"id_tender" => &id_tender},
                            ))?;
                        }
                        Ordering::Less => {
                            cancel = 1;
                        }
                    };
                }
            }
        }
        Ok((cancel, upd))
    }

    fn get_etp_id(
        &self,
        pool: &my::Pool,
        etp_name: &str,
        etp_url: &str,
    ) -> Result<u64, Box<error::Error>> {
        let mut res = (pool.prep_exec(
            "SELECT id_etp FROM etp WHERE name = :name AND url = :url LIMIT 1",
            params! {"name" => etp_name, "url" => etp_url},
        ))?;
        if let Some(id_etp_row) = res.next() {
            let id_etp = (id_etp_row)?
                .get_opt::<Value, usize>(0)
                .ok_or("None id_etp")?
                .and_then(|x| my::from_value_opt::<u64>(x))?;
            return Ok(id_etp);
        } else {
            let res_insert = (pool.prep_exec(
                "INSERT INTO etp SET name = :name, url = :url, conf=0",
                params! {"name" => etp_name, "url" => etp_url},
            ))?;
            return Ok(res_insert.last_insert_id());
        }
    }

    fn get_placing_way_id(&self, pool: &my::Pool, pw_name: &str) -> Result<u64, Box<error::Error>> {
        let mut res = (pool.prep_exec(
            "SELECT id_placing_way FROM placing_way WHERE name = :name LIMIT 1",
            params! {"name" => pw_name},
        ))?;
        if let Some(id_pw_row) = res.next() {
            let id_pw = (id_pw_row)?
                .get_opt::<Value, usize>(0)
                .ok_or("None id_pw")?
                .and_then(|x| my::from_value_opt::<u64>(x))?;
            return Ok(id_pw);
        } else {
            let conf = match pw_name.to_lowercase() {
                ref x if x.contains("открыт") => 5,
                ref x if x.contains("аукцион") => 1,
                ref x if x.contains("котиров") => 2,
                ref x if x.contains("предложен") => 3,
                ref x if x.contains("единств") => 4,
                _ => 6,
            };
            let res_insert = (pool.prep_exec(
                "INSERT INTO placing_way SET name = :name, conformity = :conformity",
                params! {"name" => pw_name, "conformity" => &conf},
            ))?;
            return Ok(res_insert.last_insert_id());
        }
    }

    fn add_version_num(
        &self,
        pool: &my::Pool,
        type_fz: i32,
        pur_num: &String,
    ) -> Result<(), Box<error::Error>> {
        let mut ver_num = 1;
        let res = (pool.prep_exec("SELECT id_tender FROM tender WHERE purchase_number = :purchase_number AND type_fz = :type_fz ORDER BY UNIX_TIMESTAMP(date_version) ASC", params! {"purchase_number" => pur_num, "type_fz" => type_fz}))?;
        for r in res.into_iter() {
            let id_tender = (r)?
                .get_opt::<Value, usize>(0)
                .ok_or("None id_tender in add_ver_number")?
                .and_then(|x| my::from_value_opt::<u64>(x))?;
            (pool.prep_exec("UPDATE tender SET num_version = :num_version WHERE id_tender = :id_tender AND type_fz = :type_fz", params! {"num_version" => &ver_num, "id_tender" => &id_tender, "type_fz" => &type_fz}))?;
            ver_num += 1;
        }
        Ok(())
    }

    fn add_tender_keywords(
        &self,
        pool: &my::Pool,
        id_tender: &u64,
    ) -> Result<(), Box<error::Error>> {
        let mut s = "".to_string();
        let res_po = (pool.prep_exec("SELECT DISTINCT po.name, po.okpd_name FROM purchase_object AS po LEFT JOIN lot AS l ON l.id_lot = po.id_lot WHERE l.id_tender = :id_tender", params! {"id_tender" => id_tender}))?;
        for r_po in res_po.into_iter() {
            let mut r_p = (r_po)?;
            let name = r_p
                .get_opt::<Value, usize>(0)
                .ok_or("None name in po.name")?
                .and_then(|x| my::from_value_opt::<String>(x))
                .unwrap_or_default();
            let okpd_name = r_p
                .get_opt::<Value, usize>(1)
                .ok_or("None name in po.okpd_name")?
                .and_then(|x| my::from_value_opt::<String>(x))
                .unwrap_or_default();
            if name != String::new() {
                s.push_str(" ");
                s.push_str(&name);
            }
            if okpd_name != String::new() {
                s.push_str(" ");
                s.push_str(&okpd_name);
            }
        }
        let res_att = (pool.prep_exec(
            "SELECT DISTINCT file_name FROM attachment WHERE id_tender = :id_tender",
            params! {"id_tender" => id_tender},
        ))?;
        for r_att in res_att.into_iter() {
            let name_att = (r_att)?
                .get_opt::<Value, usize>(0)
                .ok_or("None name in attachment")?
                .and_then(|x| my::from_value_opt::<String>(x))
                .unwrap_or_default();
            if name_att != String::new() {
                s.push_str(" ");
                s.push_str(&name_att);
            }
        }
        let res_ten = (pool.prep_exec(
            "SELECT purchase_object_info, id_organizer FROM tender WHERE id_tender = :id_tender",
            params! {"id_tender" => id_tender},
        ))?;
        for r_ten in res_ten.into_iter() {
            let mut r_p = (r_ten)?;
            let t_name = r_p
                .get_opt::<Value, usize>(0)
                .ok_or("None purchase_object_info in tender")?
                .and_then(|x| my::from_value_opt::<String>(x))
                .unwrap_or_default();
            if t_name != String::new() {
                s.push_str(" ");
                s.push_str(&t_name);
            }
            let id_org = r_p
                .get_opt::<Value, usize>(1)
                .ok_or("None id_org in tender")?
                .and_then(|x| my::from_value_opt::<u64>(x))
                .unwrap_or_default();
            if id_org != 0 {
                let res_org = (pool.prep_exec(
                    "SELECT full_name, inn FROM organizer WHERE id_organizer = :id_organizer",
                    params! {"id_organizer" => &id_org},
                ))?;
                for r_org in res_org.into_iter() {
                    let mut r_o = (r_org)?;
                    let org_name = r_o
                        .get_opt::<Value, usize>(0)
                        .ok_or("None org_name in organizer")?
                        .and_then(|x| my::from_value_opt::<String>(x))
                        .unwrap_or_default();
                    let org_inn = r_o
                        .get_opt::<Value, usize>(1)
                        .ok_or("None org_inn in organizer")?
                        .and_then(|x| my::from_value_opt::<String>(x))
                        .unwrap_or_default();
                    if org_name != String::new() {
                        s.push_str(" ");
                        s.push_str(&org_name);
                    }
                    if org_inn != String::new() {
                        s.push_str(" ");
                        s.push_str(&org_inn);
                    }
                }
            }
        }
        let res_cus = (pool.prep_exec("SELECT DISTINCT cus.inn, cus.full_name FROM customer AS cus LEFT JOIN purchase_object AS po ON cus.id_customer = po.id_customer LEFT JOIN lot AS l ON l.id_lot = po.id_lot WHERE l.id_tender = :id_tender", params! {"id_tender" => &id_tender}))?;
        for r_cus in res_cus.into_iter() {
            let mut r_c = (r_cus)?;
            let cus_inn = r_c
                .get_opt::<Value, usize>(0)
                .ok_or("None cus_inn in customer")?
                .and_then(|x| my::from_value_opt::<String>(x))
                .unwrap_or_default();
            let cus_name = r_c
                .get_opt::<Value, usize>(1)
                .ok_or("None cus_name in customer")?
                .and_then(|x| my::from_value_opt::<String>(x))
                .unwrap_or_default();
            if cus_inn != String::new() {
                s.push_str(" ");
                s.push_str(&cus_inn);
            }
            if cus_name != String::new() {
                s.push_str(" ");
                s.push_str(&cus_name);
            }
        }
        let res_string = regextools::RegexTools::del_double_ws(&s).unwrap_or(s);
        (pool.prep_exec(
            "UPDATE tender SET tender_kwords = :tender_kwords WHERE id_tender = :id_tender",
            params! {"tender_kwords" => &res_string.trim(), "id_tender" => &id_tender},
        ))?;
        Ok(())
    }
}
