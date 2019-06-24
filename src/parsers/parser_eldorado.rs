extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Not, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_eldorado::TenderEldorado;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools::DateTimeTools;
use crate::toolslib::httptools;
use crate::toolslib::toolslib;
use std::error;

pub struct ParserEldorado<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserEldorado<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserEldorado<'a> {
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
        let url = "http://www.eldorado.ru/company/tenders/";
        let page = httptools::HttpTools::get_page_from_wget_1251(&url);
        match page {
            Ok(p) => {
                self.get_tenders_from_page(p);
            }
            Err(e) => {
                warn!("can not get start page {} {}", url, e);
                return;
            }
        }
    }

    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for ten in document.find(
            Name("table")
                .and(Class("tender"))
                .child(Name("tbody"))
                .child(Name("tr").and(Not(Class("common_tender_head")))),
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
        let href = "https://www.eldorado.ru/company/tenders/".to_string();
        let pur_name = tender
            .find(Name("td"))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "can not find td 0 tag pur_name on tender", href
            ))?
            .text()
            .trim()
            .to_string();
        let pur_num = toolslib::create_md5_str(&pur_name);
        let date_pub_t = tender
            .find(Name("td"))
            .nth(2)
            .ok_or(format!(
                "{} {}",
                "can not find td 2 tag pur_name on tender", pur_name
            ))?
            .text()
            .trim()
            .to_string();
        let date_pub = DateTimeTools::get_date_from_string(&date_pub_t, "%d.%m.%Y").ok_or(
            format!("{} {}", "can not find date_pub on tender", pur_name),
        )?;
        let date_end_t = tender
            .find(Name("td"))
            .nth(3)
            .ok_or(format!(
                "{} {}",
                "can not find td 3 tag pur_name on tender", pur_name
            ))?
            .text()
            .trim()
            .to_string();
        let date_end = DateTimeTools::get_date_from_string(&date_end_t, "%d.%m.%Y").ok_or(
            format!("{} {}", "can not find date_end on tender", pur_name),
        )?;
        let tn = TenderEldorado {
            type_fz: 198,
            etp_name: "ПАО «М.Видео-Эльдорадо»".to_string(),
            etp_url: "https://www.eldorado.ru/".to_string(),
            href,
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
