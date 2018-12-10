extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_uds::TenderUds;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use std::error;

pub struct ParserUds<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserUds<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserUds<'a> {
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
        let url = "http://uds-group.ru/tenders";
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
            Class("table-responsive")
                .and(Name("div"))
                .child(Name("table").and(Class("table")))
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

    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<error::Error>> {
        let a_t = tender
            .find(Name("td").child(Name("a")))
            .next()
            .ok_or("can not find a tag on tender")?;
        let href_t = a_t.attr("href").ok_or("can not find href attr on tender")?;
        let href = format!("http://uds-group.ru{}", href_t);
        let pur_name = tender
            .find(Name("td"))
            .nth(1)
            .ok_or("can not find pur_name on tender")?
            .text()
            .trim()
            .to_string();
        let mut pur_obj = tender
            .find(Name("td"))
            .nth(2)
            .ok_or("can not find pur_obj on tender")?
            .text()
            .trim()
            .to_string();
        if pur_obj == "" {
            pur_obj = pur_name.clone();
        }
        let pur_num = tender
            .find(Name("td"))
            .nth(0)
            .ok_or("can not find pur_num on tender")?
            .text()
            .trim()
            .to_string();
        let pub_date_t = tender
            .find(Name("td"))
            .nth(3)
            .ok_or("can not find pur_num on tender")?
            .text()
            .trim()
            .to_string();
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&pub_date_t, "%d.%m.%Y")
            .ok_or("can not find date_pub on tender")?;
        let end_date_t = tender
            .find(Name("td"))
            .nth(4)
            .map(|x| x.text().trim().to_string())
            .and_then(|x| {
                if x == "" {
                    Some("01.01.1970".to_string())
                } else {
                    Some(x)
                }
            })
            .unwrap_or("01.01.1970".to_string());
        let date_end = datetimetools::DateTimeTools::get_date_from_string(&end_date_t, "%d.%m.%Y")
            .ok_or("can not find date_end on tender")?;
        let tn: TenderUds = TenderUds {
            type_fz: 134,
            etp_name: "Холдинг UDS group".to_string(),
            etp_url: "http://uds-group.ru/".to_string(),
            href,
            pur_num,
            pur_name: pur_name.to_string(),
            pur_obj,
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
