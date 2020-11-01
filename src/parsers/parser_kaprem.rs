extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Attr, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::parsers::parsers::Attachment;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_kaprem::TenderKaprem;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::{httptools, toolslib};
use std::error;

pub struct ParserKaprem<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

struct Kaprem {
    pub url: String,
}

impl<'a> WebParserTenders for ParserKaprem<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserKaprem<'a> {
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
            Kaprem {
                url: "http://kapremont02.ru/purchase/external-procurement/topical/".to_string(),
            },
            Kaprem {
                url: "http://kapremont02.ru/purchase/internal-procurement/topical/".to_string(),
            },
        ];
        for (c, url) in urls.iter().enumerate() {
            let page = httptools::HttpTools::get_page_text(&url.url);
            match page {
                Some(p) => {
                    self.get_tenders_from_page(p, url, c);
                }
                None => {
                    warn!("cannot get start page {}", url.url);
                    return;
                }
            }
        }
    }

    fn get_tenders_from_page(&mut self, page_text: String, place: &Kaprem, count: usize) {
        let document = Document::from(&*page_text);
        for ten in document.find(Name("div").and(Attr {
            0: "style",
            1: "margin:10px 0 10px 0px; color:#1066b3;",
        })) {
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
        place: &Kaprem,
        _count: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        let pur_name = tender
            .find(Name("span"))
            .nth(0)
            .ok_or(format!(
                "{} {}",
                "cannot find  pur_name on tender",
                tender.text()
            ))?
            .text()
            .trim()
            .to_string();
        let pur_num = toolslib::create_md5_str(&pur_name);
        let date_pub = datetimetools::DateTimeTools::return_datetime_now();
        let mut attachments: Vec<Attachment> = Vec::new();
        let attach_s = tender.find(Name("a"));
        for at in attach_s {
            let name_att = at.text().trim().to_string();
            if !name_att.contains("Скачать") {
                continue;
            }
            let url_att_t = at
                .attr("href")
                .ok_or("cannot find href attr on attachment")?;
            let url_att = format!("http://kapremont02.ru{}", url_att_t.to_string());
            let att = Attachment {
                name_file: name_att,
                url_file: url_att,
            };
            attachments.push(att);
        }
        let href = &place.url;
        let tn = TenderKaprem {
            type_fz: 250,
            etp_name: "НЕКОММЕРЧЕСКАЯ ОРГАНИЗАЦИЯ ФОНД «РЕГИОНАЛЬНЫЙ ОПЕРАТОР КАПИТАЛЬНОГО РЕМОНТА ОБЩЕГО ИМУЩЕСТВА В МНОГОКВАРТИРНЫХ ДОМАХ, РАСПОЛОЖЕННЫХ НА ТЕРРИТОРИИ РЕСПУБЛИКИ БАШКОРТОСТАН»	".to_string(),
            etp_url: "http://kapremont02.ru/".to_string(),
            href: &href,
            pur_num,
            pur_name,
            date_pub,
            attachments,
            connect_string: &self.connect_string,
        };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
