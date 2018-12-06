extern crate web_parser_rust;
#[macro_use]
extern crate log;
extern crate log4rs;

use web_parser_rust::settings::settings::{create_settings, FullSettingsParser, get_argument, Args};
use web_parser_rust::parsers::{parser_mts::ParserMts, parser_nefaz::ParserNefaz, parser_kam_gb::ParserKamgb};
use std::process;
use web_parser_rust::parsers::parsers::WebParserTenders;

fn main() {
    let set = parser_initialise();
    parser_executor(&set);
    parser_end();
}

fn parser_executor(set: &FullSettingsParser) {
    let arg = get_argument().unwrap();
    match arg {
        Args::Mts => {
            parser_mts(set);
        }
        Args::Nefaz => {
            parser_nefaz(set);
        }
        Args::Kamgb => {
            parser_kamgb(set);
        }
        _ => {
            warn!("Bad enum type!");
            process::exit(0x0100);
        }
    }
}

fn parser_initialise() -> FullSettingsParser {
    let set = create_settings();
    info!("Start parsing");
    set
}

fn parser_end() {
    info!("End parsing");
}

fn parser_mts(set: &FullSettingsParser) {
    let mut p = ParserMts {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_nefaz(set: &FullSettingsParser) {
    let mut p = ParserNefaz {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_kamgb(set: &FullSettingsParser) {
    let mut p = ParserKamgb {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}