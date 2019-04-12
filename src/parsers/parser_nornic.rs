extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Name, Predicate, Class};
use super::parsers::WebParserTenders;
use crate::parsers::parsers::Attachment;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_salavat::TenderSalavat;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use crate::toolslib::regextools;
use std::error;

pub struct ParserNornic<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserNornic<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserNornic<'a> {
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
        let url = "http://www.zf.norilsknickel.ru/guest_listsets.aspx?subtype=1&category=-1";
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
        for (i, ten) in document
            .find(Name("table").and(Class("GuestListSet")).child(Name("tr")).child(Name("td")).child(Name("table")))
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

    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<error::Error>> {
        println!("{}", tender.text());
        Ok(())
    }
}