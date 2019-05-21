extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_lada::TenderLada;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools::DateTimeTools;
use crate::toolslib::httptools;
use crate::toolslib::toolslib;
use chrono::Datelike;
use std::error;

pub struct ParserLada<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserLada<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserLada<'a> {
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
        let url = "https://lada-image.ru/about/tender_committee/";
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
        for ten in document.find(Name("div").and(Class("tender_item"))) {
            match self.parser_tender(ten) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<error::Error>> {
        let a_t = tender
            .find(Name("a"))
            .nth(0)
            .ok_or(format!("{}", "can not find td tag a_t on tender"))?;
        let href_t = a_t.attr("href").ok_or("can not find href attr on tender")?;
        let href = format!("https://lada-image.ru{}", href_t);
        let pur_num = toolslib::create_md5_str(&*href);
        let pur_name = tender
            .find(Name("div").and(Class("news_item_text")))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "can not find div tag pur_name on tender", pur_num
            ))?
            .text();
        let number_d = tender
            .find(Name("div").and(Class("date_news")))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "can not find div tag number_d on tender", pur_num
            ))?
            .find(Name("span"))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "can not find span tag number_d on tender", pur_num
            ))?
            .text();
        let month_text = tender
            .find(Name("div").and(Class("date_news")))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "can not find div tag month_text on tender", pur_num
            ))?
            .find(Name("span"))
            .nth(1)
            .ok_or(format!(
                "{} {}",
                "can not find span tag month_text on tender", pur_num
            ))?
            .text();
        let curr_year = format!("{}", DateTimeTools::return_datetime_now().date().year());
        let month = toolslib::month_to_number(&month_text);
        let full_date = format!("{}.{}.{}", number_d, month, curr_year);
        let date_pub = DateTimeTools::get_date_from_string(&full_date, "%d.%m.%Y")
            .ok_or(format!("{} {}", "can not find date_pub on tender", pur_num))?;
        let tn = TenderLada {
            type_fz: 191,
            etp_name: "АО «Лада-Имидж»".to_string(),
            etp_url: "https://lada-image.ru/".to_string(),
            href,
            pur_num,
            pur_name,
            date_pub,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
