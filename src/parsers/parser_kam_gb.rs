extern crate select;
extern crate chrono;

use super::parsers::WebParserTenders;
use ::settings::settings::FullSettingsParser;
use ::toolslib::httptools;
use std::error;
use ::toolslib::datetimetools;
use ::toolslib::toolslib;
use ::toolslib::toolslib::FindExt;
use self::select::document::Document;
use self::select::predicate::{Class, Name, Predicate};
use self::select::node::Node;
use tenders::tender_kamgb::TenderKamgb;
use tenders::tenders::WebTender;
use parsers::parsers::Attachment;

pub struct ParserKamgb<'a> {
    pub add_tender: i32,
    pub upd_tender: i32,
    pub settings: &'a FullSettingsParser,
    pub connect_string: String,
}

impl<'a> WebParserTenders for ParserKamgb<'a> {
    fn parser(&mut self)
    {
        self.try_parsing();
        self.end_parsing(&self.add_tender, &self.upd_tender);
    }
}

impl<'a> ParserKamgb<'a> {
    pub fn try_parsing(&mut self) {
        let c_s = format!("mysql://{}:{}@{}:{}/{}", self.settings.userdb, self.settings.passdb, self.settings.server, self.settings.port, self.settings.database);
        self.connect_string = c_s;
        let url = "http://www.kamgb.ru/postavka/";
        let page = httptools::HttpTools::get_page_text(url);
        match page {
            Some(p) => {
                self.get_tenders_from_page(p);
            }
            None => {
                warn!("can not get start page {}", url);
                return;
            }
        }
    }

    fn get_tenders_from_page(&mut self, page_text: String) {
        let document = Document::from(&*page_text);
        for ten in document.find(Class("tenders").child(Class("tender"))) {
            match self.parser_tender(ten) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    fn parser_tender(&mut self, tender: Node) -> Result<(), Box<error::Error>> {
        let a_t = tender.find(Class("tender__name").and(Name("div")).child(Name("a"))).next().ok_or("can not find a tag on tender")?;
        let href_t = a_t.attr("href").ok_or("can not find href attr on tender")?;
        let href = format!("http://www.kamgb.ru{}", href_t);
        let pur_num = toolslib::create_md5_str(&href);
        let pur_name = a_t.text().trim().to_string();
        let mut attachments: Vec<Attachment> = Vec::new();
        let attach_s = tender.find(Class("download").child(Name("a").and(Class("dashed"))));
        for at in attach_s {
            let name_att = at.text().trim().to_string();
            let url_att_t = at.attr("href").ok_or("can not find href attr on attachment")?;
            let url_att = format!("http://www.kamgb.ru{}", url_att_t);
            let att = Attachment { name_file: name_att, url_file: url_att };
            attachments.push(att);
        }
        let mut ten_info = tender.find(Class("tender__info").and(Name("div")));
        let cus_name_tt = toolslib::find_from_child_text(&mut ten_info, "can not find cus_name_t", "Инициатор закупки")?;
        let cus_name = cus_name_tt.text().replace("Инициатор закупки", "").trim().to_string();

        let mut ten_info = tender.find(Class("tender__info").and(Name("div")));
        let pub_date_tt = toolslib::find_from_child_text(&mut ten_info, "can not find pub_date_tt", "Дата размещения")?;
        let pub_date_t = pub_date_tt.text().replace("Дата размещения", "").trim().to_string();

        let end_date_tt = tender.find_text_in_node(Class("tender__info").and(Name("div")), "can not find end_date_tt", "Срок приёма предложения")?;
        let end_date_t = end_date_tt.text().replace("Срок приёма предложения, до", "").trim().to_string();
        let pub_date_str = datetimetools::DateTimeTools::replace_str_months(&pub_date_t).ok_or("can not replace pub_date_str")?;
        let end_date_str = datetimetools::DateTimeTools::replace_str_months(&end_date_t).ok_or("can not replace end_date_str")?;
        let date_pub = datetimetools::DateTimeTools::get_date_from_string(&pub_date_str, "%d.%m.%Y").ok_or("can not find date_pub on tender")?;
        let date_end = datetimetools::DateTimeTools::get_date_from_string(&end_date_str, "%d.%m.%Y").ok_or("can not find date_end on tender")?;
        let tn: TenderKamgb = TenderKamgb { type_fz: 133, etp_name: "OOO «КАМАЗжилбыт»".to_string(), etp_url: "http://www.kamgb.ru/".to_string(), href, pur_num, pur_name: pur_name.to_string(), cus_name, date_pub, date_end, attachments, connect_string: &self.connect_string };
        let (addt, updt) = tn.parser();
        self.add_tender += addt;
        self.upd_tender += updt;
        Ok(())
    }
}
