extern crate web_parser_rust;
#[macro_use]
extern crate log;
extern crate log4rs;

use std::process;
use web_parser_rust::parsers::parsers::WebParserTenders;
use web_parser_rust::parsers::{
    parser_ahstep::ParserAhstep, parser_alfa::ParserAlfa, parser_am::ParserAm,
    parser_asia::ParserAsia, parser_azer::ParserAzer, parser_azs::ParserAzs,
    parser_baltika::ParserBaltika, parser_beeline::ParserBeeline, parser_dochki::ParserDochki,
    parser_eldorado::ParserEldorado, parser_ingrad::ParserIngrad, parser_kam_gb::ParserKamgb,
    parser_kaprem::ParserKaprem, parser_lada::ParserLada, parser_medsi::ParserMedsi,
    parser_megafon::ParserMegafon, parser_mosobl::ParserMosobl, parser_mts::ParserMts,
    parser_nefaz::ParserNefaz, parser_nordstar::ParserNordstar, parser_nornic::ParserNornic,
    parser_pewete::ParserPewete, parser_quadra::ParserQuadra, parser_ruscoal::ParserRuscoal,
    parser_salavat::ParserSalavat, parser_smp::ParserSmp, parser_snhz::ParserSnHz,
    parser_tgk14::ParserTgk14, parser_uds::ParserUds, parser_ungi::ParserUngi,
};
use web_parser_rust::settings::settings::{
    create_settings, get_argument, Args, FullSettingsParser,
};

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
        Args::Uds => {
            parser_uds(set);
        }
        Args::Beeline => {
            parser_beeline(set);
        }
        Args::Megafon => {
            parser_megafon(set);
        }
        Args::Ahstep => {
            parser_ahstep(set);
        }
        Args::Salavat => {
            parser_salavat(set);
        }
        Args::Nornic => {
            parser_nornic(set);
        }
        Args::Pewete => {
            parser_pewete(set);
        }
        Args::Quadra => {
            parser_quadra(set);
        }
        Args::Tgk14 => {
            parser_tgk14(set);
        }
        Args::Medsi => {
            parser_medsi(set);
        }
        Args::Lada => {
            parser_lada(set);
        }
        Args::Asia => {
            parser_asia(set);
        }
        Args::Eldorado => {
            parser_eldorado(set);
        }
        Args::Mosobl => {
            parser_mosobl(set);
        }
        Args::Baltika => {
            parser_baltika(set);
        }
        Args::Alfa => {
            parser_alfa(set);
        }
        Args::Smp => {
            parser_smp(set);
        }
        Args::Am => {
            parser_am(set);
        }
        Args::Azer => {
            parser_azer(set);
        }
        Args::Dochki => {
            parser_dochki(set);
        }
        Args::Ungi => {
            parser_ungi(set);
        }
        Args::Ruscoal => {
            parser_ruscoal(set);
        }
        Args::Azs => {
            parser_azs(set);
        }
        Args::Nordstar => {
            parser_nordstar(set);
        }
        Args::Ingrad => {
            parser_ingrad(set);
        }
        Args::Kaprem => {
            parser_kaprem(set);
        }
        Args::Snhz => {
            parser_snhz(set);
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

fn parser_uds(set: &FullSettingsParser) {
    let mut p = ParserUds {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_beeline(set: &FullSettingsParser) {
    let mut p = ParserBeeline {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_megafon(set: &FullSettingsParser) {
    let mut p = ParserMegafon {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_ahstep(set: &FullSettingsParser) {
    let mut p = ParserAhstep {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_salavat(set: &FullSettingsParser) {
    let mut p = ParserSalavat {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_nornic(set: &FullSettingsParser) {
    let mut p = ParserNornic {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_pewete(set: &FullSettingsParser) {
    let mut p = ParserPewete {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_quadra(set: &FullSettingsParser) {
    let mut p = ParserQuadra {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_tgk14(set: &FullSettingsParser) {
    let mut p = ParserTgk14 {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_medsi(set: &FullSettingsParser) {
    let mut p = ParserMedsi {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_lada(set: &FullSettingsParser) {
    let mut p = ParserLada {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_asia(set: &FullSettingsParser) {
    let mut p = ParserAsia {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_eldorado(set: &FullSettingsParser) {
    let mut p = ParserEldorado {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_mosobl(set: &FullSettingsParser) {
    let mut p = ParserMosobl {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_baltika(set: &FullSettingsParser) {
    let mut p = ParserBaltika {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_alfa(set: &FullSettingsParser) {
    let mut p = ParserAlfa {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_smp(set: &FullSettingsParser) {
    let mut p = ParserSmp {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_am(set: &FullSettingsParser) {
    let mut p = ParserAm {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_azer(set: &FullSettingsParser) {
    let mut p = ParserAzer {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_dochki(set: &FullSettingsParser) {
    let mut p = ParserDochki {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_ungi(set: &FullSettingsParser) {
    let mut p = ParserUngi {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_ruscoal(set: &FullSettingsParser) {
    let mut p = ParserRuscoal {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_azs(set: &FullSettingsParser) {
    let mut p = ParserAzs {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_nordstar(set: &FullSettingsParser) {
    let mut p = ParserNordstar {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_ingrad(set: &FullSettingsParser) {
    let mut p = ParserIngrad {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_kaprem(set: &FullSettingsParser) {
    let mut p = ParserKaprem {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}

fn parser_snhz(set: &FullSettingsParser) {
    let mut p = ParserSnHz {
        add_tender: 0,
        upd_tender: 0,
        settings: set,
        connect_string: String::new(),
    };
    p.parser();
}
