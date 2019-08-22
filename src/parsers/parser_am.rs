extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_am::TenderAm;
use crate::tenders::tenders::WebTender;
use crate::toolslib::httptools;
use crate::toolslib::{datetimetools, regextools, toolslib};
use std::error;

pub struct ParserAm<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserAm<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserAm<'a> {
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
        let urls = ["http://procurement.am/ru/page/obyavleniya_o_zakupkakh_/"];
        for url in urls.iter() {
            for i in 1..=5 {
                let url_n = format!("{}{}", url, i);
                let page = httptools::HttpTools::get_page_text(&url_n);
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
        }
    }
    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for ten in document.find(Name("div").and(Class("tender"))) {
            match self.parser_tender(ten) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }
    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<dyn error::Error>> {
        let pur_name = tender
            .find(Name("div").and(Class("tender_title")))
            .nth(0)
            .ok_or(format!("{} {}", "can not find  pur_name on tender", ""))?
            .text()
            .trim()
            .to_string();
        let href = tender
            .find(
                Name("div")
                    .and(Class("tender_title"))
                    .child(Name("p"))
                    .child(Name("a")),
            )
            .next()
            .ok_or("can not find href_t on tender")?
            .attr("href")
            .ok_or("can not find href attr on href")?
            .to_string();
        let pur_num = toolslib::create_md5_str(&href);
        let date_pub_t = tender
            .find(Name("div").child(Name("p").and(Class("tender_time"))))
            .nth(0)
            .ok_or("can not find date_pub_t on tender")?
            .text();
        let date_pub_tt = regextools::RegexTools::get_one_group(
            &date_pub_t,
            r"(\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2})",
        )
        .ok_or(format!(
            "{} {}",
            "can not find date_pub_tt on tender", pur_num
        ))?;
        let date_pub = datetimetools::DateTimeTools::get_datetime_from_string(
            &date_pub_tt,
            "%Y-%m-%d %H:%M:%S",
        )
        .ok_or(format!("{} {}", "can not find date_pub on tender", pur_num))?;
        let date_end = date_pub.clone();
        let tn = TenderAm {
            type_fz: 209,
            etp_name: "МИНИСТЕРСТВО ФИНАНСОВ РЕСПУБЛИКИ АРМЕНИЯ"
                .to_string(),
            etp_url: "http://procurement.am/".to_string(),
            href: &href,
            pur_num,
            pur_name,
            date_pub,
            date_end,
            attach_url: &href,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
