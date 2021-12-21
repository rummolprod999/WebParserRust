extern crate chrono;
extern crate select;

use self::select::document::Document;
use self::select::node::Node;
use self::select::predicate::{Class, Name, Predicate};
use super::parsers::WebParserTenders;
use crate::parsers::parsers::Attachment;
use crate::settings::settings::FullSettingsParser;
use crate::tenders::tender_quadra::TenderQuadra;
use crate::tenders::tenders::WebTender;
use crate::toolslib::datetimetools;
use crate::toolslib::httptools;
use crate::toolslib::regextools;
use std::error;

pub struct ParserQuadra<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserQuadra<'a> {
    fn parser(&mut self) {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserQuadra<'a> {
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
        let url = "https://trade.quadra.ru/purchase/purch_prod.php";
        let page = httptools::HttpTools::get_page_text1251(url);
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
        let mut inc = 0;
        for ten in document.find(Name("div").and(Class("news")).child(Name("dt"))) {
            let ten2 = match document
                .find(Name("div").and(Class("news")).child(Name("dd")))
                .nth(inc)
            {
                Some(t) => t,
                None => return,
            };
            match self.parser_tender(ten, ten2) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
            inc += 1;
        }
    }

    fn parser_tender(&mut self, tender1: Node, tender2: Node) -> Result<(), Box<dyn error::Error>> {
        let pur_name = tender2
            .find(Name("font"))
            .nth(0)
            .ok_or("cannot find pur_name on tender")?
            .text();
        let href_t = tender2
            .find(Name("a").and(|_x: &Node| {
                return true;
            }))
            .next()
            .ok_or("cannot find href_t on tender")?
            .attr("href")
            .ok_or("cannot find href attr on href_t")?;
        let href = format!("https://trade.quadra.ru{}", href_t);
        let mut tmp_cus_and_pur_name = tender1
            .find(Name("div").child(Name("b")).child(Name("font")))
            .next()
            .ok_or("cannot find tmp_cus_and_pur_name on tender")?
            .text()
            .to_string();
        tmp_cus_and_pur_name = tmp_cus_and_pur_name.trim().to_string();
        let pur_num = regextools::RegexTools::get_one_group(&tmp_cus_and_pur_name, r"№(\d+)")
            .ok_or(format!("{} {}", "cannot find pur_num on tender", pur_name))?;
        let cus_name = regextools::RegexTools::get_one_group(&tmp_cus_and_pur_name, r"/\s+(.+)")
            .ok_or(format!("{} {}", "cannot find cus_name on tender", pur_name))?;
        let pw_name = tender1
            .find(Name("b"))
            .nth(0)
            .ok_or("cannot find pw_name on tender")?
            .attr("title")
            .ok_or("cannot find title attr on pw_name")?
            .to_string();
        let date_pub_t = tender1
            .find(Name("a"))
            .nth(0)
            .ok_or("cannot find date_pub_t on tender")?
            .text();
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&date_pub_t, "%d.%m.%Y")
            .ok_or(format!("{} {}", "cannot find date_pub on tender", pur_num))?;
        let date_end_t = tender1
            .find(Name("a"))
            .nth(1)
            .ok_or("cannot find date_end_t on tender")?
            .text();
        let date_end =
            datetimetools::DateTimeTools::get_datetime_from_string(&date_end_t, "%d.%m.%Y (%H:%M)")
                .ok_or(format!("{} {}", "cannot find date_pub on tender", pur_num))?;
        let mut attachments: Vec<Attachment> = Vec::new();
        let attach_s = tender2.find(Name("a"));
        for at in attach_s {
            let name_att = at.text().trim().to_string();
            let url_att_t = at
                .attr("href")
                .ok_or("cannot find href attr on attachment")?;
            let url_att = format!("https://trade.quadra.ru{}", url_att_t);
            let att = Attachment {
                name_file: name_att,
                url_file: url_att,
            };
            attachments.push(att);
        }
        let tn = TenderQuadra {
            type_fz: 184,
            etp_name: "ПАО «Квадра»".to_string(),
            etp_url: "https://trade.quadra.ru/".to_string(),
            href,
            pur_num,
            pur_name,
            pw_name,
            cus_name,
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
