extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_ahstep::TenderAhstep;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::datetimetools::DateTimeTools;
use crate::toolslib::httptools;
use crate::toolslib::regextools;
use crate::toolslib::toolslib;
use std::error;

pub struct ParserAhstep<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserAhstep<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserAhstep<'a> {
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
        let url = "https://ahstep.ru/tenders";
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
                .and(Class("articles-tender"))
                .child(Class("row")),
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
            .find(Name("p").and(Class("tender-title")).child(Name("a")))
            .next()
            .ok_or("can not find a tag on tender")?;
        let href_t = a_t.attr("href").ok_or("can not find href attr on tender")?;
        let href = format!("https://ahstep.ru/tenders{}", href_t);
        let pur_name = tender
            .find(Name("p").and(Class("tender-title")).child(Name("a")))
            .next()
            .ok_or("can not find pur_name on tender")?
            .text()
            .trim()
            .to_string();
        let pur_num = toolslib::create_md5_str(&*href);
        let date_pub = DateTimeTools::return_datetime_now();
        let mut end_date_t = tender
            .find(Name("p").and(Class("text-center")))
            .nth(0)
            .ok_or("can not find end_date_t on tender")?
            .text()
            .trim()
            .to_string();
        end_date_t = regextools::RegexTools::del_double_ws(&end_date_t)
            .ok_or("bad delete double whitespace")?;
        let date_end =
            datetimetools::DateTimeTools::get_datetime_from_string(&end_date_t, "%d.%m.%Y %H:%M")
                .ok_or("can not find date_end on tender")?;
        let tn: TenderAhstep = TenderAhstep {
            type_fz: 141,
            etp_name: "АО Агрохолдинг «СТЕПЬ»".to_string(),
            etp_url: "https://ahstep.ru/".to_string(),
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
        Ok(())
    }
}
