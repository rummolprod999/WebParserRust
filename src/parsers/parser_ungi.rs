extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_ungi::TenderUngi;
use crate::tenders::tenders::WebTender;
use crate::toolslib::httptools;
use crate::toolslib::{datetimetools, regextools};
use std::error;

pub struct ParserUngi<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserUngi<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserUngi<'a> {
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
        let urls = [
            "http://ungi.uz/category/tendery/obyavlennie-konkursi/page/",
            "http://ungi.uz/category/tendery/konkursy-v-ramkah-investitsionnyh-proektov/page/",
            "http://ungi.uz/category/tendery/tendery-tendery/page/",
        ];
        for url in urls.iter() {
            for i in (1..=5).rev() {
                let url_n = format!("{}{}/", url, i);
                let page = httptools::HttpTools::get_page_text(&url_n);
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
    }
    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for ten in document.find(
            Name("div")
                .and(Class("thumbnail"))
                .child(Name("div").and(Class("caption"))),
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
            .find(Name("h4"))
            .nth(0)
            .ok_or(format!("{} {}", "cannot find  pur_name on tender", ""))?
            .text()
            .trim()
            .to_string();
        let full_text = tender
            .find(Name("p"))
            .nth(0)
            .ok_or(format!("{} {}", "cannot find  full_text on tender", ""))?
            .html()
            .trim()
            .to_string();
        let mut pur_num = regextools::RegexTools::get_one_group(&full_text, r"№ отбора —(.+?)<br")
            .ok_or(format!("{} {}", "cannot find pur_num on tender", pur_name))?;
        pur_num = pur_num
            .replace("<strong>", "")
            .replace("</strong>", "")
            .replace("&nbsp;", "")
            .trim()
            .to_string();
        let mut date_pub_text = regextools::RegexTools::get_one_group(
            &full_text,
            r"Дата публикации \(GMT \+5\).+?—(.+?)<br",
        )
        .ok_or(format!(
            "{} {}",
            "cannot find date_pub_text on tender", pur_name
        ))?;
        date_pub_text = date_pub_text
            .replace("<strong>", "")
            .replace("</strong>", "")
            .replace("&nbsp;", "")
            .trim()
            .to_string();
        let date_pub =
            datetimetools::DateTimeTools::get_date_from_string(&date_pub_text, "%d.%m.%Y").ok_or(
                format!("{} {}", "cannot find date_pub on tender", date_pub_text),
            )?;
        let mut date_end_text = regextools::RegexTools::get_one_group(
            &full_text,
            r"Дата окончания приема предложений \(GMT \+5\).+?—(.+?)<br",
        )
        .ok_or(format!(
            "{} {}",
            "cannot find date_end_text on tender", pur_name
        ))?;
        date_end_text = date_end_text
            .replace("<strong>", "")
            .replace("</strong>", "")
            .replace("&nbsp;", "")
            .trim()
            .to_string();
        let date_end = datetimetools::DateTimeTools::get_datetime_from_string(
            &date_end_text,
            "%d.%m.%Y  до %H:%M",
        )
        .ok_or(format!(
            "{} {}",
            "cannot find date_end on tender", date_end_text
        ))?;
        let mut cus_name =
            regextools::RegexTools::get_one_group(&full_text, r"Заказчик.+?—(.+?)<br")
                .ok_or(format!("{} {}", "cannot find cus_name on tender", pur_name))?;
        cus_name = cus_name
            .replace("<strong>", "")
            .replace("</strong>", "")
            .replace("&nbsp;", "")
            .trim()
            .to_string();
        let href = tender
            .find(Name("a").and(|node: &Node| node.text().contains("Скачать файл")))
            .next()
            .ok_or("cannot find href_t on tender")?
            .attr("href")
            .ok_or("cannot find href attr on href")?
            .to_string();
        let tn = TenderUngi {
            type_fz: 224,
            etp_name: "ООО «Нефтегазинвест»".to_string(),
            etp_url: "http://ungi.uz".to_string(),
            href: &href,
            pur_num,
            cus_name,
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
