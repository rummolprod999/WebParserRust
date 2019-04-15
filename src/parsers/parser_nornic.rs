extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_nornic::TenderNornic;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use crate::toolslib::regextools;
use std::error;

pub struct ParserNornic<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserNornic<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserNornic<'a> {
    pub fn try_parsing(&mut self) {
        let c_s = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.settings.userdb,
            self.settings.passdb,
            self.settings.server,
            self.settings.port,
            self.settings.database
        );
        self.connect_string = c_s;
        let url = "http://www.zf.norilsknickel.ru/guest_listsets.aspx?subtype=1&category=-1";
        let page = httptools::HttpTools::get_page_text1251(url);
        match page {
            Some(p) => {
                self.get_tenders_from_page(p);
            }
            None => {
                warn!("can not get start page {}", url);
                return;
            }
        }
    }

    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for (_i, ten) in document
            .find(
                Name("table")
                    .and(Class("GuestListSet"))
                    .child(Name("tbody"))
                    .child(Name("tr"))
                    .child(Name("td"))
                    .child(Name("table"))
                    .child(Name("tbody")),
            )
            .enumerate()
        {
            match self.parser_tender(ten) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<error::Error>> {
        let pur_num_t = tender
            .find(Name("tr"))
            .next()
            .ok_or("can not find pur_num_t on tender")?
            .find(Name("td"))
            .next()
            .ok_or("can not find pur_num_t on tender")?
            .attr("id")
            .ok_or("can not find attr id pur_num_t on tender")?
            .to_string();
        let pur_num = pur_num_t.replace("l", "");
        let tmp = tender
            .find(
                Name("tr")
                    .child(Name("td"))
                    .child(Name("table"))
                    .child(Name("tbody"))
                    .child(Name("tr"))
                    .child(Name("td")),
            )
            .nth(0)
            .ok_or("can not find tmp on tender")?
            .text()
            .to_string();
        let cus_name_t = regextools::RegexTools::get_one_group(&tmp.trim(), r":(.+)№")
            .ok_or(format!("{} {}", "can not find cus_name on tender", pur_num))?;
        let cus_name =
            regextools::RegexTools::del_double_ws(&cus_name_t).ok_or("del double space error")?;
        let pub_date_t =
            regextools::RegexTools::get_one_group(&tmp.trim(), r"(\d{2}\.\d{2}\.\d{4})").ok_or(
                format!("{} {}", "can not find pub_date_t on tender", pur_num),
            )?;
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&pub_date_t, "%d.%m.%Y")
            .ok_or(format!("{} {}", "can not find date_pub on tender", pur_num))?;
        let pur_name = tender
            .find(Name("td").and(Class("Name")))
            .next()
            .ok_or("can not find attr pur_name on tender")?
            .text();
        let href_t = tender
            .find(Name("a").and(|x: &Node| {
                if x.text().contains("подробнее") {
                    true
                } else {
                    false
                }
            }))
            .next()
            .ok_or(format!("{} {}", "can not find href_t on tender", pur_num))?
            .attr("href")
            .ok_or(format!(
                "{} {}",
                "can not find href_t attr on tender", pur_num
            ))?;
        let href = format!("http://www.zf.norilsknickel.ru/{}", href_t);
        let tn: TenderNornic = TenderNornic {
            type_fz: 181,
            etp_name: "ЗФ ПАО \"ГМК \"Норильский никель\"".to_string(),
            etp_url: "http://www.zf.norilsknickel.ru/".to_string(),
            href,
            pur_num,
            pur_name,
            cus_name,
            date_pub,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
