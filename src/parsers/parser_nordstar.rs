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

pub struct ParserNordstar<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserNordstar<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserNordstar<'a> {
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
        let urls = ["https://nordstar.ru/partners/purchase/?SID=8"];
        for url in urls.iter() {
            let page = httptools::HttpTools::get_page_text_no_ssl(url);
            match page {
                Some(p) => {
                    self.get_tenders_from_page(p, url.to_string());
                }
                None => {
                    warn!("cannot get start page {}", url);
                    return;
                }
            }
        }
    }
    fn get_tenders_from_page(&mut self, page_text: String, url: String) {
        let document = Document::from(&*page_text);
        for (_i, ten) in document
            .find(
                Name("div")
                    .and(Class("al-dl-docs-item").or(Class("al-dl-docs-item al-dl-docs-item-bg"))),
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
        let pur_num = tender
            .find(Name("div").and(Class("al-dl-doc-name")).child(Name("a")))
            .next()
            .ok_or("cannot find pur_name on tender")?
            .text()
            .trim()
            .to_string();
        let href = tender
            .find(Name("div").and(Class("al-dl-doc-name")).child(Name("a")))
            .next()
            .ok_or("cannot find href_t on tender")?
            .attr("href")
            .ok_or("cannot find href attr on href")?
            .to_string();
        let href = format!("https://nordstar.ru/partners/purchase/{}", href.to_string());
        let pur_name = tender
            .find(Name("div").and(Class("al-dl-discribe")).child(Name("p")))
            .nth(0)
            .and_then(|x| Some(x.text().trim().to_string()))
            .unwrap_or(pur_num.to_string());
        let pub_date_t = tender
            .find(
                Name("div")
                    .and(Class("docs-date-time"))
                    .and(|x: &Node| x.text().contains("Документ обновлен:")),
            )
            .nth(0)
            .ok_or("cannot find pub_date_t")?
            .text()
            .trim()
            .to_string();
        let pub_date_t = regextools::RegexTools::get_one_group(
            &pub_date_t,
            r"(\d{2}\.\d{2}\.\d{4} \d{2}:\d{2})",
        )
        .ok_or(format!(
            "{} {}",
            "cannot find pub_date_t on tender", pub_date_t
        ))?;
        let date_pub =
            datetimetools::DateTimeTools::get_datetime_from_string(&pub_date_t, "%d.%m.%Y %H:%M")
                .ok_or(format!(
                "{} {}",
                "cannot find date_pub on tender", pub_date_t
            ))?;
        let end_date_t = tender
            .find(
                Name("div")
                    .and(Class("docs-date-time"))
                    .and(|x: &Node| x.text().contains("Срок подачи предложения до")),
            )
            .nth(0)
            .and_then(|x| Some(x.text().trim().to_string()))
            .unwrap_or("".to_string());
        let end_date_t = regextools::RegexTools::get_one_group(
            &end_date_t,
            r"(\d{2}\.\d{2}\.\d{4} \d{2}:\d{2}:\d{2})",
        )
        .ok_or(format!(
            "{} {}",
            "cannot find end_date_t on tender", end_date_t
        ))?;
        let date_end = datetimetools::DateTimeTools::get_datetime_from_string(
            &end_date_t,
            "%d.%m.%Y %H:%M:%S",
        )
        .ok_or(format!(
            "{} {}",
            "cannot find end_date_t on tender", end_date_t
        ))?;
        let tn: TenderNordstar = TenderNordstar {
            type_fz: 237,
            etp_name: "NordStar (АО «АК «НордСтар»)".to_string(),
            etp_url: "https://nordstar.ru/".to_string(),
            href: href.to_string(),
            pur_num,
            pur_name,
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
