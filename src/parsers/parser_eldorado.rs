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
        println!("{}", page_text);
    }
}
