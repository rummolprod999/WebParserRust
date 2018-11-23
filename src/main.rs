extern crate WebParserRust;
#[macro_use]
extern crate log;
extern crate log4rs;
use WebParserRust::settings::settings::create_settings;

fn main() {
    create_settings();
    info!("Start parsing");
}
