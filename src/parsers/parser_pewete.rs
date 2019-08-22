extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_pewete::TenderPewete;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use std::error;

pub struct ParserPewete<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserPewete<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserPewete<'a> {
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
        let url = "http://tender.pewete.com/auctions/?q=&pageSize=100";
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
                    .and(Class("dataTable"))
                    .child(Name("tbody"))
                    .child(Name("tr")),
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

    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<dyn error::Error>> {
        let mut pur_num = tender
            .find(Name("td"))
            .nth(0)
            .ok_or("can not find tmp on tender")?
            .text();
        pur_num = pur_num.trim().to_string();
        let mut pur_name = tender
            .find(Name("td"))
            .nth(1)
            .ok_or("can not find pur_name on tender")?
            .text();
        pur_name = pur_name.trim().to_string();
        let date_pub_t = tender
            .find(Name("td"))
            .nth(2)
            .ok_or("can not find date_pub_t on tender")?
            .text();
        let date_end_t = tender
            .find(Name("td"))
            .nth(3)
            .ok_or("can not find date_end_t on tender")?
            .text();
        let date_scoring_t = tender
            .find(Name("td"))
            .nth(4)
            .ok_or("can not find date_scoring_t on tender")?
            .text();
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&date_pub_t, "%d.%m.%Y")
            .ok_or(format!("{} {}", "can not find date_pub on tender", pur_num))?;
        let date_end = datetimetools::DateTimeTools::get_date_from_string(&date_end_t, "%d.%m.%Y")
            .ok_or(format!("{} {}", "can not find date_end on tender", pur_num))?;
        let date_scoring =
            datetimetools::DateTimeTools::get_date_from_string(&date_scoring_t, "%d.%m.%Y").ok_or(
                format!("{} {}", "can not find date_scoring_t on tender", pur_num),
            )?;
        let tn = TenderPewete {
            type_fz: 183,
            etp_name: "ООО «Петро Велт Технолоджис»".to_string(),
            etp_url: "http://tender.pewete.com/".to_string(),
            href: "http://tender.pewete.com/auctions/?q=&pageSize=100".to_string(),
            pur_num,
            pur_name,
            date_pub,
            date_end,
            date_scoring,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
