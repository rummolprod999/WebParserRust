extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_nordstar::TenderNordstar;
use crate::tenders::tender_snhz::TenderSnHz;
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
        let documents: Vec<Node> = document
            .find(
                Name("table")
                    .and(Class("tab_z"))
                    .child(Name("tbody"))
                    .child(Name("tr")),
            )
            .collect();
        for tender in (0..documents.len()).step_by(2) {
            match self.parser_tender(documents[tender], documents[tender + 1], &url) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    fn parser_tender(
        &mut self,
        tender1: Node,
        tender2: Node,
        _url: &String,
    ) -> Result<(), Box<dyn error::Error>> {
        let href = tender1
            .find(Name("td").child(Name("a")))
            .next()
            .ok_or("cannot find href_t on tender")?
            .attr("href")
            .ok_or("cannot find href attr on href")?
            .to_string();
        let href = format!("https://snhz.ru{}", href.to_string());
        let pur_num = tender1
            .find(Name("td"))
            .nth(0)
            .ok_or("cannot find pur_num on tender")?
            .text()
            .trim()
            .to_string();
        let org_name = tender1
            .find(Name("td").child(Name("a")))
            .next()
            .ok_or("cannot find org_name on tender")?
            .text()
            .trim()
            .to_string();
        let pw_name = tender1
            .find(Name("td"))
            .nth(2)
            .ok_or("cannot find pw_name on tender")?
            .text()
            .trim()
            .to_string();
        let pur_name = tender2
            .find(Name("td").child(Name("a")))
            .nth(0)
            .ok_or("cannot find pur_name on tender")?
            .text()
            .trim()
            .to_string();
        let pub_date_t = tender1
            .find(Name("td"))
            .nth(1)
            .ok_or("cannot find pub_date_t")?
            .text()
            .trim()
            .to_string();
        let date_pub =
            datetimetools::DateTimeTools::get_datetime_from_string(&pub_date_t, "%Y-%m-%d %H:%M")
                .ok_or(format!(
                "{} {}",
                "cannot find date_pub on tender", pub_date_t
            ))?;
        let end_date_t = tender2
            .find(Name("td"))
            .nth(0)
            .ok_or("cannot find end_date_t")?
            .text()
            .trim()
            .to_string();
        let date_end =
            datetimetools::DateTimeTools::get_datetime_from_string(&end_date_t, "%Y-%m-%d %H:%M")
                .ok_or(format!(
                "{} {}",
                "cannot find date_end on tender", pub_date_t
            ))?;
        let tn: TenderSnHz = TenderSnHz {
            type_fz: 281,
            etp_name: "ОАО «СНХЗ»".to_string(),
            etp_url: "https://snhz.ru".to_string(),
            href: href.to_string(),
            pur_num,
            pur_name,
            org_name,
            pw_name,
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
