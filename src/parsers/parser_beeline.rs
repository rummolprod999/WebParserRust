extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_beeline::TenderBeeline;
use crate::tenders::tenders::WebTender;
use crate::toolslib::httptools;
use crate::toolslib::toolslib;
use std::error;

pub struct ParserBeeline<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserBeeline<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserBeeline<'a> {
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
        let url = "https://moskva.beeline.ru/about/partners/tender/tenders/";
        let page = httptools::HttpTools::get_page_text(url);
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
                .and(Class("purchase-list"))
                .descendant(Name("h3")),
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
        let a_t = tender
            .find(Name("a"))
            .next()
            .ok_or("can not find a tag on tender")?;
        let href_t = a_t.attr("href").ok_or("can not find href attr on tender")?;
        let href = format!("https://moskva.beeline.ru{}", href_t);
        let pur_name = tender
            .find(Name("a"))
            .next()
            .ok_or("can not find pur_name on tender")?
            .text()
            .trim()
            .to_string();
        let pur_num = toolslib::create_md5_str(&href);
        let tn: TenderBeeline = TenderBeeline {
            type_fz: 135,
            etp_name: "ПАО «ВымпелКом»".to_string(),
            etp_url: "https://beeline.ru".to_string(),
            href,
            pur_num,
            pur_name,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
