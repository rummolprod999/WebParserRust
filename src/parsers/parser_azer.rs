extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_azer::TenderAzer;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use std::error;

pub struct ParserAzer<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserAzer<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserAzer<'a> {
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
        let url = "http://ru.azerbaijan.tenderinfo.org/";
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
        for ten in document.find(Name("div").and(Class("country_list_single"))) {
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
            .find(
                Name("div")
                    .and(Class("country_list_txt"))
                    .child(Name("a"))
                    .child(Name("b")),
            )
            .nth(0)
            .ok_or(format!("{} {}", "cannot find  pur_name on tender", ""))?
            .text()
            .trim()
            .to_string();
        let href = tender
            .find(Name("div").and(Class("country_list_txt")).child(Name("a")))
            .next()
            .ok_or("cannot find href on tender")?
            .attr("href")
            .ok_or("cannot find href attr on href")?;
        let date_pub = datetimetools::DateTimeTools::return_datetime_now();
        let pur_num = tender
            .find(
                Name("div")
                    .and(Class("right"))
                    .child(Name("p"))
                    .child(Name("span").and(Class("right"))),
            )
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "cannot find div tag pur_num on tender", href
            ))?
            .text()
            .trim()
            .to_string();
        let org_name = tender
            .find(Name("span").and(Class("s14")))
            .nth(1)
            .ok_or(format!("{} {}", "cannot find  org_name on tender", ""))?
            .text()
            .trim()
            .to_string();
        let tn = TenderAzer {
            type_fz: 211,
            etp_name: "TenderInfo.Org".to_string(),
            etp_url: "http://ru.azerbaijan.tenderinfo.org/".to_string(),
            href: &href.to_string(),
            pur_num,
            pur_name,
            date_pub,
            org_name,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
