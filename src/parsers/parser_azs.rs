extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::parsers::parsers::Attachment;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_azs::TenderAzs;
use crate::tenders::tenders::WebTender;
use crate::toolslib::httptools;
use crate::toolslib::{datetimetools, regextools};
use std::error;

pub struct ParserAzs<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserAzs<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserAzs<'a> {
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
        let url = "https://azsgazprom.ru/?id=41&tender=1";
        let page = httptools::HttpTools::get_page_text(url);
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
            Class("table-ges")
                .and(Name("table"))
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
    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<dyn error::Error>> {
        let pur_name = tender
            .find(
                Name("td")
                    .child(Name("b"))
                    .child(Name("a").and(Class("dot-link"))),
            )
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "cannot find  pur_name on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let pur_name2 = tender
            .find(Name("div").and(Class("mt-1")))
            .nth(1)
            .ok_or(format!(
                "{} {}",
                "cannot find  pur_name2 on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let pur_num = regextools::RegexTools::get_one_group(&pur_name, r"№(\d+)")
            .ok_or(format!("{} {}", "cannot find pur_num on tender", pur_name))?;
        let pub_date_t =
            regextools::RegexTools::get_one_group(&pur_name, r"/(\d{2}\.\d{2}\.\d{4})/").ok_or(
                format!("{} {}", "cannot find pub_date_t on tender", pur_name),
            )?;
        let date_pub =
            datetimetools::DateTimeTools::get_date_from_string(&pub_date_t, "%d.%m.%Y").ok_or(
                format!("{} {}", "cannot find date_pub on tender", pub_date_t),
            )?;
        let pw_name = tender
            .find(Name("td"))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "cannot find  pw_name on tender",
                tender.text()
            ))?
            .first_child()
            .ok_or(format!(
                "{} {}",
                "cannot find  pw_name on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let end_date_full = tender
            .find(Name("td"))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "cannot find  pw_name on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let end_date_t = regextools::RegexTools::get_one_group(
            &end_date_full,
            r"до\s*(\d{2}\.\d{2}\.\d{4}, \d{2}:\d{2})",
        )
        .ok_or(format!(
            "{} {}",
            "cannot find end_date_t on tender", end_date_full
        ))?;
        let date_end =
            datetimetools::DateTimeTools::get_datetime_from_string(&end_date_t, "%d.%m.%Y, %H:%M")
                .or_else(|| {
                    datetimetools::DateTimeTools::get_date_from_string(&end_date_t, "%d.%m.%Y")
                })
                .ok_or(format!(
                    "{} {}",
                    "cannot find date_end on tender", end_date_t
                ))?;
        let cus_name = tender
            .find(Name("td"))
            .nth(2)
            .ok_or(format!(
                "{} {}",
                "cannot find  cus_name on tender",
                tender.text()
            ))?
            .first_child()
            .ok_or(format!(
                "{} {}",
                "cannot find  cus_name on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let org_name = tender
            .find(Name("td"))
            .nth(2)
            .ok_or(format!(
                "{} {}",
                "cannot find  cus_name on tender",
                tender.text()
            ))?
            .last_child()
            .ok_or(format!(
                "{} {}",
                "cannot find  cus_name on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let mut attachments: Vec<Attachment> = Vec::new();
        let attach_s = tender.find(
            Name("td")
                .child(Name("div"))
                .child(Name("p"))
                .child(Name("a")),
        );
        for at in attach_s {
            let name_att = at.text().trim().to_string();
            if name_att.contains("Принять") {
                continue;
            }
            let url_att_t = at
                .attr("href")
                .ok_or("cannot find href attr on attachment")?;
            let url_att = format!("https://azsgazprom.ru/{}", url_att_t.to_string());
            let att = Attachment {
                name_file: name_att,
                url_file: url_att,
            };
            attachments.push(att);
        }
        let href = "https://azsgazprom.ru/?id=41&tender=1".to_string();
        let tn = TenderAzs {
            type_fz: 229,
            etp_name: "ООО «ГЭС розница»".to_string(),
            etp_url: "https://azsgazprom.ru/".to_string(),
            href: &href,
            pur_num,
            pur_name: pur_name2,
            cus_name,
            org_name,
            pw_name,
            date_pub,
            date_end,
            attachments,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
