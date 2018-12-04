extern crate select;
extern crate chrono;

use super::parsers::WebParserTenders;
use ::settings::settings::FullSettingsParser;
use ::toolslib::httptools;
use ::toolslib::regextools;
use ::toolslib::datetimetools;
use ::toolslib::toolslib;
use self::select::document::Document;
use self::select::predicate::{Class, Name, Predicate};
use self::select::node::Node;
use tenders::tender_nefaz::TenderNefaz;
use tenders::tenders::WebTender;

pub struct ParserNefaz<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserNefaz<'a> {
    fn parser(&mut self)
    {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserNefaz<'a> {
    pub fn try_parsing(&mut self) {
        let c_s = format!("mysql://{}:{}@{}:{}/{}", self.settings.userdb, self.settings.passdb, self.settings.server, self.settings.port, self.settings.database);
        self.connect_string = c_s;
        let url = "http://www.nefaz.ru/supply/announcements/";
        let page = httptools::HttpTools::get_page_text(url);
        match page {
            Some(p) => self.get_tenders_from_page(p),
            None => {
                warn!("can not get start page {}", url);
                return;
            }
        }
    }

    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for ten in document.find(Class("tablecat").child(Name("tbody")).child(Name("tr"))) {
            self.parser_tender(ten);
        }
    }

    fn parser_tender(&mut self, tender: Node) {
        let a_t = match tender.find(Name("a")).next() {
            Some(n) => { n }
            None => {
                warn!("{}", "can not find a tag on tender");
                return;
            }
        };
        let href_t = match a_t.attr("href") {
            Some(hr) => hr,
            None => {
                warn!("{}", "can not find href attr on tender");
                return;
            }
        };
        let href = format!("http://www.nefaz.ru{}", href_t);
        let pur_num = toolslib::create_md5_str(&href);
        let pur_name = match tender.find(Name("td")).nth(1) {
            Some(n) => { n.text() }
            None => {
                warn!("{}", "can not find pur_name on tender");
                return;
            }
        };
        let date_pb = match tender.find(Name("td")).nth(0) {
            Some(n) => { n.text() }
            None => {
                warn!("{}", "can not find date_pb on tender");
                return;
            }
        };
        let datepb_t = match regextools::RegexTools::get_one_group(&date_pb.trim(), r"(\d{2}\.\d{2}\.\d{4})") {
            Some(pn) => pn,
            None => {
                warn!("{} {}", "can not find date_pub_t on tender", href_t);
                return;
            }
        };
        let date_pub_t = datetimetools::DateTimeTools::get_date_from_string(&datepb_t, "%d.%m.%Y");
        let date_pub = match date_pub_t {
            Some(d) => d,
            None => {
                warn!("{} {}", "can not find date_pub on tender", href_t);
                return;
            }
        };
        let date_end = datetimetools::DateTimeTools::return_min_datetime();
        let org_name = match tender.find(Name("td")).nth(2) {
            Some(n) => { n.text() }
            None => {
                warn!("{}", "can not find org_name on tender");
                return;
            }
        };
        let tn: TenderNefaz = TenderNefaz { type_fz: 132, etp_name: "ПАО «НЕФАЗ»".to_string(), etp_url: "http://www.nefaz.ru/".to_string(), href, pur_num, pur_name: pur_name.to_string(), org_name, date_pub, date_end, connect_string: &self.connect_string };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
    }
}