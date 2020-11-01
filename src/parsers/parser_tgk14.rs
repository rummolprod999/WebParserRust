extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_tgk14::TenderTgk14;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use std::error;

pub struct ParserTgk14<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserTgk14<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserTgk14<'a> {
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
        let url_b = "https://old.tgk-14.com/trade/vesti.sections.php?&num_page=";

        for d in (1..=3).rev() {
            let url = format!("{}{}", url_b, d);
            let page = httptools::HttpTools::get_page_text(&url);
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
    }

    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for ten in document.find(
            Name("table")
                .child(Name("tbody"))
                .child(Name("tr").and(Class("line-odd").or(Class("line-even")))),
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
        let pur_num = tender
            .find(Name("td"))
            .nth(0)
            .ok_or("cannot find pur_num on tender")?
            .text();
        let pur_name = tender
            .find(Name("td"))
            .nth(2)
            .ok_or(format!(
                "{} {}",
                "cannot find td tag pur_name on tender", pur_num
            ))?
            .find(Name("a"))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "cannot find a tag pur_name on tender", pur_num
            ))?
            .text();
        let a_t = tender
            .find(Name("td"))
            .nth(2)
            .ok_or(format!(
                "{} {}",
                "cannot find td tag a_t on tender", pur_num
            ))?
            .find(Name("a"))
            .nth(0)
            .ok_or(format!("{} {}", "cannot find a tag a_t on tender", pur_num))?;
        let href_t = a_t.attr("href").ok_or("cannot find href attr on tender")?;
        let href = format!("https://old.tgk-14.com{}", href_t);
        let pw_name = tender
            .find(Name("td"))
            .nth(3)
            .map_or("".to_string(), |x| x.text());
        let date_pub_t = tender
            .find(Name("td"))
            .nth(4)
            .ok_or("cannot find date_pub_t on tender")?
            .text();
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&date_pub_t, "%d.%m.%Y")
            .ok_or(format!("{} {}", "cannot find date_pub on tender", pur_num))?;
        let date_end_t = tender
            .find(Name("td"))
            .nth(5)
            .ok_or("cannot find date_end_t on tender")?
            .text();
        let date_end = datetimetools::DateTimeTools::get_datetime_from_string(
            &date_end_t,
            "%d.%m.%Y %H:%M:%S",
        )
        .ok_or(format!("{} {}", "cannot find date_end on tender", pur_num))?;
        let status = tender
            .find(Name("td"))
            .nth(6)
            .map_or("".to_string(), |x| x.text());
        let tn = TenderTgk14 {
            type_fz: 187,
            etp_name: "ПАО «Территориальная Генерирующая Компания № 14»".to_string(),
            etp_url: "https://www.tgk-14.com/".to_string(),
            href,
            pur_num,
            pur_name,
            pw_name,
            date_pub,
            date_end,
            status,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
