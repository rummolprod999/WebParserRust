extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_mosobl::TenderMosobl;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use std::error;

pub struct ParserMosobl<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserMosobl<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserMosobl<'a> {
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
        let url = "https://mosoblbank.ru/about/tenders/";
        let page = httptools::HttpTools::get_page_text_no_ssl(&url);
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
            Name("table")
                .and(Class("tableCommon"))
                .child(Name("tbody"))
                .child(Name("tr").and(|x: &Node| {
                    if x.text().contains("Начало")
                        && x.text().contains("Окончание")
                        && x.text().contains("Название")
                    {
                        false
                    } else {
                        true
                    }
                })),
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
        let pur_name = tender
            .find(Name("td"))
            .nth(3)
            .ok_or(format!("{} {}", "cannot find  pur_name on tender", ""))?
            .text()
            .trim()
            .to_string();
        let pur_num = tender
            .find(Name("td"))
            .nth(2)
            .ok_or(format!("{} {}", "cannot find  pur_num on tender", pur_name))?
            .text()
            .trim()
            .to_string();
        let href_t = tender
            .find(Name("a"))
            .next()
            .ok_or("cannot find href_t on tender")?
            .attr("href")
            .ok_or("cannot find href attr on href_t")?;
        let href = format!("https://mosoblbank.ru{}", href_t);
        let date_pub_t = tender
            .find(Name("td"))
            .nth(0)
            .ok_or("cannot find date_pub_t on tender")?
            .text()
            .replace("0019", "2019");
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&date_pub_t, "%d.%m.%Y")
            .ok_or(format!(
                "{} {} {}",
                "cannot find date_pub on tender", pur_num, date_pub_t
            ))?;
        let date_end_t = tender
            .find(Name("td"))
            .nth(1)
            .ok_or("cannot find date_end_t on tender")?
            .text()
            .replace("0019", "2019");
        let date_end =
            datetimetools::DateTimeTools::get_datetime_from_string(&date_end_t, "%d.%m.%Y %H:%M")
                .or_else(|| {
                    datetimetools::DateTimeTools::get_date_from_string(&date_end_t, "%d.%m.%Y")
                })
                .ok_or(format!(
                    "{} {} {}",
                    "cannot find date_end on tender", pur_num, date_pub_t
                ))?;
        let tn = TenderMosobl {
            type_fz: 199,
            etp_name: "ПАО МОСОБЛБАНК".to_string(),
            etp_url: "https://mosoblbank.ru/".to_string(),
            href: &href,
            pur_num,
            pur_name,
            date_pub,
            date_end,
            attach_url: &href,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
