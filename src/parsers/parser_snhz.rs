extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_nordstar::TenderNordstar;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use crate::toolslib::regextools;
use select::predicate::Class;
use std::error;

pub struct ParserSnHz<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserSnHz<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserSnHz<'a> {
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
        let urls = ["https://snhz.ru/?event=zakupki"];
        for url in urls.iter() {
            let page = httptools::HttpTools::get_page_text1251(url);
            match page {
                Some(p) => {
                    self.get_tenders_from_page(p, url.to_string());
                }
                None => {
                    warn!("can not get start page {}", url);
                    return;
                }
            }
        }
    }
    fn get_tenders_from_page(&mut self, page_text: String, url: String) {
        let document = Document::from(&*page_text);
        for (_i, ten) in document
            .find(
                Name("table")
                    .and(Class("tab_z"))
                    .child(Name("tbody"))
                    .child(Name("tr")),
            )
            .enumerate()
        {
            match self.parser_tender(ten, &url) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    fn parser_tender(&mut self, tender: Node, _url: &String) -> Result<(), Box<dyn error::Error>> {
        let href = tender
            .find(Name("div").and(Class("al-dl-doc-name")).child(Name("a")))
            .next()
            .ok_or("cannot find href_t on tender")?
            .attr("href")
            .ok_or("cannot find href attr on href")?
            .to_string();
        let href = format!("https://nordstar.ru/partners/purchase/{}", href.to_string());
        println!("{:?}", tender.as_text());
        Ok(())
    }
}
