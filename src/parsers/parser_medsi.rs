extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_medsi::TenderMedsi;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use std::error;

pub struct ParserMedsi<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserMedsi<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserMedsi<'a> {
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
        let url = "https://medsi.ru/about/purchases/";
        let page = httptools::HttpTools::get_page_text(&url);
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
        for ten in document.find(Name("tr").and(Class("active_tender"))) {
            match self.parser_tender(ten) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<dyn error::Error>> {
        let pur_num = tender
            .find(Name("td"))
            .nth(0)
            .ok_or("can not find pur_num on tender")?
            .text();
        let pur_name = tender
            .find(Name("td"))
            .nth(2)
            .ok_or(format!(
                "{} {}",
                "can not find td tag pur_name on tender", pur_num
            ))?
            .find(Name("a"))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "can not find a tag pur_name on tender", pur_num
            ))?
            .text();
        let a_t = tender
            .find(Name("td"))
            .nth(2)
            .ok_or(format!(
                "{} {}",
                "can not find td tag a_t on tender", pur_num
            ))?
            .find(Name("a"))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "can not find a tag a_t on tender", pur_num
            ))?;
        let href_t = a_t.attr("href").ok_or("can not find href attr on tender")?;
        let href = format!("https://medsi.ru{}", href_t);
        let date_pub_t = tender
            .find(Name("td"))
            .nth(3)
            .ok_or("can not find date_pub_t on tender")?
            .text();
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&date_pub_t, "%d.%m.%Y")
            .ok_or(format!("{} {}", "can not find date_pub on tender", pur_num))?;
        let date_end_t = tender
            .find(Name("td"))
            .nth(4)
            .ok_or("can not find date_end_t on tender")?
            .text();
        let date_end = datetimetools::DateTimeTools::get_datetime_from_string(
            &date_end_t,
            "%d.%m.%Y %H:%M:%S",
        )
        .ok_or(format!("{} {}", "can not find date_end on tender", pur_num))?;
        let tn = TenderMedsi {
            type_fz: 190,
            etp_name: "Медицинская корпорация МЕДСИ".to_string(),
            etp_url: "https://medsi.ru/".to_string(),
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
