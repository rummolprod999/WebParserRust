extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_mts::TenderMts;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use crate::toolslib::regextools;

pub struct ParserMts<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserMts<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserMts<'a> {
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
        let url = "https://tenders.mts.ru/default.aspx";
        let page = httptools::HttpTools::get_page_text(url);
        match page {
            Some(p) => self.get_tenders_from_page(p),
            None => {
                warn!("cannot get start page {}", url);
                return;
            }
        }
    }
    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for ten in document.find(Class("m-purchases-list-row")) {
            self.parser_tender(ten);
        }
    }

    fn parser_tender(&mut self, tender: Node) {
        let a_t = match tender.find(Name("a")).next() {
            Some(n) => n,
            None => {
                warn!("{}", "cannot find a tag on tender");
                return;
            }
        };
        let href_t = match a_t.attr("href") {
            Some(hr) => hr,
            None => {
                warn!("{}", "cannot find href attr on tender");
                return;
            }
        };
        let pur_name_t = a_t.text();
        let pur_name = pur_name_t.trim();
        let pur_num = match regextools::RegexTools::get_one_group(href_t, r"tender_id=(\d+)") {
            Some(pn) => pn,
            None => {
                warn!("{} {}", "cannot find pur_num on tender", href_t);
                return;
            }
        };
        let href = format!("https://tenders.mts.ru/{}", href_t);
        let date_pb = match tender.find(Class("m-purchases-list-date")).next() {
            Some(n) => n,
            None => {
                warn!("{} {}", "cannot find date_pb on tender", href);
                return;
            }
        };
        let datepb_t = match regextools::RegexTools::get_one_group(
            &date_pb.text().trim(),
            r"(\d{2}\.\d{2}\.\d{4})",
        ) {
            Some(pn) => pn,
            None => {
                warn!("{} {}", "cannot find date_pub_t on tender", href_t);
                return;
            }
        };
        let date_pub_t = datetimetools::DateTimeTools::get_date_from_string(&datepb_t, "%d.%m.%Y");
        let date_pub = match date_pub_t {
            Some(d) => d,
            None => {
                warn!("{} {}", "cannot find date_pub on tender", href_t);
                return;
            }
        };
        let date_end = datetimetools::DateTimeTools::return_min_datetime();
        let tn: TenderMts = TenderMts {
            type_fz: 131,
            etp_name: "ПАО «Мобильные ТелеСистемы»".to_string(),
            etp_url: "https://tenders.mts.ru/".to_string(),
            href,
            pur_num,
            pur_name: pur_name.to_string(),
            date_pub,
            date_end,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
    }
}
