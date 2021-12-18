extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_dochki::TenderDochki;
use crate::tenders::tenders::WebTender;
use crate::toolslib::httptools;
use crate::toolslib::{datetimetools, regextools, toolslib};
use std::error;
use std::iter::Iterator;

pub struct ParserDochki<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserDochki<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserDochki<'a> {
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
        let url_b = "https://www.dochkisinochki.ru/shops/tender/?PAGEN_1=";

        for d in (1..=3).rev() {
            let url = format!("{}{}", url_b, d);
            let page = httptools::HttpTools::get_page_text_ua(&url);
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
    }

    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for ten in document.find(Name("div").and(Class("tender_element"))) {
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
            .find(Name("h2"))
            .nth(0)
            .ok_or(format!("{} {}", "cannot find  pur_name on tender", ""))?
            .text()
            .trim()
            .to_string();
        let pur_num = toolslib::create_md5_str(&pur_name);
        let pub_date_t =
            regextools::RegexTools::get_one_group(&pur_name.trim(), r"\((\d{2}\.\d{2}\.\d{2})")
                .ok_or(format!(
                    "{} {}",
                    "cannot find pub_date_t on tender", pur_num
                ))?;
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&pub_date_t, "%d.%m.%y")
            .ok_or(format!("{} {}", "cannot find date_pub on tender", pur_num))?;
        let pub_end_t =
            regextools::RegexTools::get_one_group(&pur_name.trim(), r"-\s*(\d{2}\.\d{2}\.\d{2})")
                .ok_or(format!(
                "{} {}",
                "cannot find pub_end_t on tender", pur_name
            ))?;
        let date_end = datetimetools::DateTimeTools::get_date_from_string(&pub_end_t, "%d.%m.%y")
            .ok_or(format!("{} {}", "cannot find date_end on tender", pur_name))?;
        let href = tender
            .find(
                Name("div")
                    .and(Class("tender_element_files"))
                    .child(Name("div"))
                    .child(Name("a")),
            )
            .next()
            .ok_or("cannot find href_t on tender")?
            .attr("href")
            .ok_or("cannot find href attr on href")?;
        let tn = TenderDochki {
            type_fz: 215,
            etp_name: "Дочки & сыночки".to_string(),
            etp_url: "https://www.dochkisinochki.ru/".to_string(),
            href: &href.to_string(),
            pur_num,
            pur_name,
            date_pub,
            date_end,
            attach_url: &href.to_string(),
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
