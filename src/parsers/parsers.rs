pub trait WebParserTenders {
    fn parser(&mut self) -> () {}
    fn end_parsing(&self, add: &i32, upd: &i32) {
        let add_count = format!("Добавили Tender {}", add);
        let upd_count = format!("Обновили Tender {}", upd);
        info!("{}", add_count);
        info!("{}", upd_count);
    }
}

pub struct Attachment {
    pub name_file: String,
    pub url_file: String,
}
