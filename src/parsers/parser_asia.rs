extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_asia::TenderAsia;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools::DateTimeTools;
use crate::toolslib::toolslib;
use crate::toolslib::{httptools, regextools};
use std::error;

pub struct ParserAsia<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserAsia<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserAsia<'a> {
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
        let url = "https://asiacement.ru/tendery-i-zakupki";
        let page = httptools::HttpTools::get_page_text(&url);
        match page {
            Some(p) => {
                self.get_tenders_from_page(p);
            }
            None => {
                warn!("cannot get start page {}", url);
                return;
            }
        }
    }

    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for ten in document.find(Name("div").and(Class("tenders-line__item_box"))) {
            match self.parser_tender(ten) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<dyn error::Error>> {
        let href = "https://asiacement.ru/tendery-i-zakupki".to_string();
        let pur_name = tender
            .find(Name("div").and(Class("tenders-line__item_title")))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "cannot find div tag pur_name on tender", href
            ))?
            .text()
            .trim()
            .to_string();
        let pur_num = toolslib::create_md5_str(&pur_name);
        let notice_ver = tender
            .find(Name("div").and(Class("tenders-line__item_desc")))
            .nth(0)
            .and_then(|x| Some(x.text().trim().to_string()))
            .unwrap_or_default();
        let dates = tender
            .find(Name("div").and(Class("tenders-line__item_date-num")))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "cannot find div tag dates on tender", href
            ))?
            .text()
            .trim()
            .to_string();
        let date_pub_t = regextools::RegexTools::get_one_group(&dates, r"с\s*(\d{2}.\d{2}.\d{4})")
            .ok_or(format!("{} {}", "cannot find date_pub_t on tender", href))?;
        let date_end_t = regextools::RegexTools::get_one_group(&dates, r"до\s*(\d{2}.\d{2}.\d{4})")
            .ok_or(format!("{} {}", "cannot find date_end_t on tender", href))?;
        let date_pub = DateTimeTools::get_date_from_string(&date_pub_t, "%d.%m.%Y")
            .ok_or(format!("{} {}", "cannot find date_pub on tender", href))?;
        let date_end = DateTimeTools::get_date_from_string(&date_end_t, "%d.%m.%Y")
            .ok_or(format!("{} {}", "cannot find date_end on tender", href))?;
        let attach_url = "https://asiacement.ru/tendery-i-zakupki".to_string();
        let tn = TenderAsia {
            type_fz: 195,
            etp_name: "ООО \"Азия Цемент\"".to_string(),
            etp_url: "https://asiacement.ru/".to_string(),
            href,
            pur_num,
            pur_name,
            date_pub,
            date_end,
            attach_url,
            notice_ver,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
