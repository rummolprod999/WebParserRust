extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_ingrad::TenderIngrad;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use std::error;

pub struct ParserIngrad<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserIngrad<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserIngrad<'a> {
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
        let url = "https://tenders.ingrad.ru/";
        let page = httptools::HttpTools::get_page_text(url);
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
        for ten in document.find(
            Class("tbl_tenders_list")
                .and(Name("table"))
                .child(Name("tbody"))
                .child(Name("tr")),
        ) {
            match self.parser_tender(ten) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }
    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<dyn error::Error>> {
        let href = "https://tenders.ingrad.ru/";
        let pur_name = tender
            .find(Name("td"))
            .nth(1)
            .ok_or("cannot find pur_name on tender")?
            .text()
            .trim()
            .to_string();
        let pur_num = tender
            .find(Name("td"))
            .nth(0)
            .ok_or("cannot find pur_num on tender")?
            .text()
            .trim()
            .to_string();
        let pub_date_t = tender
            .find(Name("td"))
            .nth(2)
            .ok_or("cannot find pub_date_t on tender")?
            .text()
            .trim()
            .to_string();
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&pub_date_t, "%d.%m.%Y")
            .ok_or("cannot find date_pub on tender")?;
        let end_date_t = tender
            .find(Name("td"))
            .nth(3)
            .ok_or("cannot find end_date_t on tender")?
            .text()
            .trim()
            .to_string();
        let date_end = datetimetools::DateTimeTools::get_date_from_string(&end_date_t, "%d.%m.%Y")
            .ok_or("cannot find date_end on tender")?;
        let tn: TenderIngrad = TenderIngrad {
            type_fz: 247,
            etp_name: "ГК \"INGRAD\"".to_string(),
            etp_url: "https://tenders.ingrad.ru/".to_string(),
            href: &href.to_string(),
            pur_num,
            pur_name: pur_name.to_string(),
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
