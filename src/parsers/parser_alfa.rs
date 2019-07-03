extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_alfa::TenderAlfa;
use crate::tenders::tenders::WebTender;
use crate::toolslib::httptools;
use crate::toolslib::{datetimetools, toolslib};
use std::error;

pub struct ParserAlfa<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserAlfa<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserAlfa<'a> {
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
        let url = "https://alfabank.ru/tenders/current/";
        let page = httptools::HttpTools::get_page_text1251(&url);
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
        for ten in document.find(
            Name("div")
                .and(Class("tender-list"))
                .child(Name("div").and(Class("tender-item"))),
        ) {
            match self.parser_tender(ten) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<error::Error>> {
        let pur_name = tender
            .find(Name("div").and(Class("title")))
            .nth(0)
            .ok_or(format!("{} {}", "can not find  pur_name on tender", ""))?
            .text()
            .trim()
            .to_string();
        let pur_num = toolslib::create_md5_str(&pur_name);
        let href_t = tender
            .find(Name("a"))
            .next()
            .ok_or("can not find href_t on tender")?
            .attr("href")
            .ok_or("can not find href attr on href_t")?;
        let href = format!("https://alfabank.ru{}", href_t);
        let date_pub_t = tender
            .find(Name("div").and(Class("date")))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "can not find date_pub_t on tender", pur_name
            ))?
            .text()
            .trim()
            .to_string();
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&date_pub_t, "%d.%m.%Y")
            .ok_or(format!(
                "{} {} {}",
                "can not find date_pub on tender", pur_num, date_pub_t
            ))?;
        let date_end = date_pub.clone();
        let tn = TenderAlfa {
            type_fz: 201,
            etp_name: "АО «Альфа-Банк»".to_string(),
            etp_url: "https://alfabank.ru/".to_string(),
            href: &href,
            pur_num,
            pur_name,
            date_pub,
            date_end,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
