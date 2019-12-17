extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::parsers::parsers::Attachment;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_ruscoal::TenderRuscoal;
use crate::tenders::tenders::WebTender;
use crate::toolslib::httptools;
use crate::toolslib::{datetimetools, regextools};
use select::predicate::Not;
use std::error;

pub struct ParserRuscoal<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

struct Ruscoal {
    pub url: String,
    pub cus_name: String,
}

impl<'a> WebParserTenders for ParserRuscoal<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserRuscoal<'a> {
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
            Ruscoal {
                url: "https://www.ruscoal.ru/mtr/".to_string(),
                cus_name: "".to_string(),
            },
            Ruscoal {
                url: "https://www.ruscoal.ru/tenders/services/ao-amurugol/".to_string(),
                cus_name: "АО «Амуруголь»".to_string(),
            },
            Ruscoal {
                url: "https://www.ruscoal.ru/tenders/services/oao-krasnoyarskkrajugol/".to_string(),
                cus_name: "АО «Красноярсккрайуголь»".to_string(),
            },
            Ruscoal {
                url: "https://www.ruscoal.ru/tenders/services/ruscoal/".to_string(),
                cus_name: "АО «Русский Уголь»".to_string(),
            },
            Ruscoal {
                url: "https://www.ruscoal.ru/tenders/services/ao-uk-razrez-stepnoj/".to_string(),
                cus_name: "АО «УК «Разрез Степной»".to_string(),
            },
        ];
        for (c, url) in urls.iter().enumerate() {
            let page = httptools::HttpTools::get_page_text(&url.url);
            match page {
                Some(p) => {
                    self.get_tenders_from_page(p, url, c);
                }
                None => {
                    warn!("can not get start page {}", url.url);
                    return;
                }
            }
        }
    }

    fn get_tenders_from_page(&mut self, page_text: String, place: &Ruscoal, count: usize) {
        let document = Document::from(&*page_text);
        for ten in document.find(
            Name("table")
                .and(Class("tender-table"))
                .child(Name("tbody"))
                .child(Name("tr").and(Not(Class("tender-table-head")))),
        ) {
            match self.parser_tender(ten, place, count) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    fn parser_tender(
        &mut self,
        tender: Node,
        place: &Ruscoal,
        count: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        let pur_num = tender
            .find(Name("td"))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "can not find  pur_num on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let pur_num = format!("{}_{}", pur_num, count);
        let pur_name = tender
            .find(Name("td"))
            .nth(4)
            .ok_or(format!(
                "{} {}",
                "can not find  pur_name on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let pub_date_t = tender
            .find(Name("td"))
            .nth(1)
            .ok_or(format!(
                "{} {}",
                "can not find  pub_date_t on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let date_pub =
            datetimetools::DateTimeTools::get_date_from_string(&pub_date_t, "%d.%m.%Y").ok_or(
                format!("{} {}", "can not find date_pub on tender", pub_date_t),
            )?;
        let end_date_text = tender
            .find(Name("td"))
            .nth(2)
            .ok_or(format!(
                "{} {}",
                "can not find  end_date_text on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let date_end_t =
            regextools::RegexTools::get_one_group(&end_date_text, r"(\d{2}.\d{2}.\d{4})").ok_or(
                format!("{} {}", "can not find date_end_t on tender", end_date_text),
            )?;
        let time_end_t = regextools::RegexTools::get_one_group(&end_date_text, r"(\d{2}[:-]\d{2})")
            .unwrap_or("".to_string());
        let mut date_end_temp = format!("{0} {1}", date_end_t, time_end_t);
        date_end_temp = date_end_temp.trim().replace("-", ":").to_string();
        let date_end = datetimetools::DateTimeTools::get_datetime_from_string(
            &date_end_temp,
            "%d.%m.%Y %H:%M",
        )
        .or_else(|| datetimetools::DateTimeTools::get_date_from_string(&date_end_temp, "%d.%m.%Y"))
        .ok_or(format!(
            "{} {}",
            "can not find date_end on tender", date_end_temp
        ))?;
        let cus_name = if place.cus_name == "" {
            tender
                .find(Name("td"))
                .nth(5)
                .ok_or(format!(
                    "{} {}",
                    "can not find  cus_name on tender",
                    tender.text()
                ))?
                .text()
                .trim()
                .to_string()
        } else {
            place.cus_name.to_string()
        };
        let mut attachments: Vec<Attachment> = Vec::new();
        let attach_s = tender
            .find(Name("td"))
            .nth(6)
            .or_else(|| tender.find(Name("td")).nth(5))
            .ok_or("can not find attachements")?
            .find(Name("a"));
        for at in attach_s {
            let name_att = at.text().trim().to_string();
            if name_att.contains("Принять") {
                continue;
            }
            let url_att_t = at
                .attr("href")
                .ok_or("can not find href attr on attachment")?;
            let url_att = format!("https://www.ruscoal.ru{}", url_att_t.to_string());
            let att = Attachment {
                name_file: name_att,
                url_file: url_att,
            };
            attachments.push(att);
        }
        let href = &place.url;
        let tn = TenderRuscoal {
            type_fz: 228,
            etp_name: "АО «Русский Уголь»".to_string(),
            etp_url: "https://www.ruscoal.ru/".to_string(),
            href: &href,
            pur_num,
            cus_name,
            pur_name,
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
