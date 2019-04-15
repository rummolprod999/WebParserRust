extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Name, Predicate};
use super::parsers::WebParserTenders;
use crate::parsers::parsers::Attachment;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_salavat::TenderSalavat;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use crate::toolslib::regextools;
use std::error;

pub struct ParserSalavat<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserSalavat<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserSalavat<'a> {
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
            "http://gazpromss.ru/zakupki/izveshenya/zakiz18/",
            "http://gazpromss.ru/zakupki/izveshenya/zizv2019/",
        ];
        for url in urls.iter() {
            let page = httptools::HttpTools::get_page_text(url);
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
        for (i, ten) in document
            .find(Name("table").child(Name("tbody")).child(Name("tr")))
            .enumerate()
        {
            if i > 0 {
                match self.parser_tender(ten, &url) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            }
        }
    }

    fn parser_tender(&mut self, tender: Node, url: &String) -> Result<(), Box<error::Error>> {
        let pur_name = tender
            .find(Name("td"))
            .next()
            .ok_or("can not find pur_name on tender")?
            .text()
            .trim()
            .to_string();
        let pur_num_t =
            match regextools::RegexTools::get_one_group_fix(&pur_name, r"(?:№(.+?)|(ПК.+?))\s+")
            {
                Some(pn) => {
                    let tmp = pn.trim();
                    tmp.to_string()
                }
                None => {
                    warn!("{} {}", "can not find pur_num on tender", &pur_name);
                    return Ok(());
                }
            };
        let pur_num = if pur_num_t.ends_with("для") {
            let m = pur_num_t.replace("для", "");
            m
        } else {
            pur_num_t
        };
        let mut attachments: Vec<Attachment> = Vec::new();
        let attach_s = tender
            .find(Name("td"))
            .nth(3)
            .ok_or("can not find attachements")?
            .find(Name("a"));
        for at in attach_s {
            let name_att_t = at.text().trim().to_string();
            let name_att = if &name_att_t == "" {
                "Подробнее".to_string()
            } else {
                name_att_t
            };
            let url_att_t = at
                .attr("href")
                .ok_or("can not find href attr on attachment")?;
            let url_att = url_att_t.to_string();
            let att = Attachment {
                name_file: name_att,
                url_file: url_att,
            };
            attachments.push(att);
        }
        let pub_date_t = tender
            .find(Name("td"))
            .nth(1)
            .ok_or("can not find pub_date_t")?
            .text()
            .replace("208", "2018")
            .trim()
            .to_string();
        let end_date_t = tender
            .find(Name("td"))
            .nth(2)
            .ok_or("can not find end_date_t")?
            .text()
            .replace("208", "2018")
            .trim()
            .to_string();
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&pub_date_t, "%d.%m.%Y")
            .ok_or("can not find date_pub on tender")?;
        let date_end = datetimetools::DateTimeTools::get_date_from_string(&end_date_t, "%d.%m.%Y")
            .ok_or("can not find date_end on tender")?;
        let tn: TenderSalavat = TenderSalavat {
            type_fz: 142,
            etp_name: "АО «Газпром СтройТЭК Салават»".to_string(),
            etp_url: "http://gazpromss.ru/".to_string(),
            href: url.to_string(),
            pur_num,
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
