extern crate clap;
extern crate chrono;
extern crate serde_json;

use self::clap::{Arg, App};
use std::process;
use std::path::PathBuf;
use std::fmt;
use std::clone::Clone;
use std::option::Option;
use std::fs;
use self::chrono::Local;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;
use std::fs::File;

#[derive(Clone)]
pub enum Args {
    None,
    Mts,
    Beeline,
}

#[derive(Deserialize, Debug)]
struct SettingsParser {
    database: String,
    userdb: String,
    passdb: String,
    server: String,
    port: String,
}

pub struct FullSettingsParser {
    pub database: String,
    pub userdb: String,
    pub passdb: String,
    pub server: String,
    pub port: String,
    pub temp: PathBuf,
    pub log: PathBuf,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Args::None => write!(f, "none"),
            Args::Beeline => write!(f, "beeline"),
            Args::Mts => write!(f, "mts"),
        }
    }
}

const SETTINGS_FILE: &str = "settings.json";
static mut ARGUMENT: Option<Args> = Some(Args::None);

pub fn create_settings() -> FullSettingsParser {
    create_argument();
    let execute_path = get_execute_path();
    let (log, temp) = create_dirs(&execute_path);
    create_log_file(&log);
    let set = get_file_settings(&execute_path);
    let s = FullSettingsParser {
        database: set.database,
        userdb: set.userdb,
        passdb: set.passdb,
        server: set.server,
        port: set.port,
        temp,
        log,
    };
    s
}

fn create_log_file(pb: &PathBuf) {
    let local_date = Local::now().date().format("%Y-%m-%d").to_string();
    let arg = get_argument().unwrap();
    let log_file_name = format!("log_parsing_{}_{}.log", arg, local_date);
    let file_log = pb.join(log_file_name);
    if !&file_log.exists() {
        fs::File::create(&file_log.as_path()).unwrap();
    }
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build(&file_log.as_path()).unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("logfile")
            .build(LevelFilter::Info)).unwrap();
    log4rs::init_config(config).unwrap();
}

fn get_file_settings(pb: &PathBuf) -> SettingsParser {
    let fb = pb.parent().unwrap();
    let filepb = fb.join(SETTINGS_FILE);
    let path = filepb.as_path();
    let file = File::open(path).unwrap();
    let v: SettingsParser = serde_json::from_reader(file).unwrap();
    v
}

fn create_dirs(pb: &PathBuf) -> (PathBuf, PathBuf) {
    let arg = get_argument().unwrap();
    let dir_log = format!("logdir_{}", arg);
    let path = pb.parent().unwrap();
    let log_path_buf = &path.join(&dir_log);
    if !log_path_buf.exists() {
        fs::create_dir_all(&path.join(log_path_buf.as_path())).unwrap();
    }
    let dir_temp = format!("tempdir_{}", arg);
    let temp_path_buf = &path.join(&dir_temp);
    if !temp_path_buf.exists() {
        fs::create_dir_all(&path.join(temp_path_buf.as_path())).unwrap();
    } else {
        fs::remove_dir_all(&path.join(temp_path_buf.as_path())).unwrap();
        fs::create_dir_all(&path.join(temp_path_buf.as_path())).unwrap();
    }
    let log = path.join(log_path_buf.as_path());
    let temp = path.join(temp_path_buf.as_path());
    (log, temp)
}

fn get_execute_path() -> PathBuf {
    let path: PathBuf = std::env::current_exe().unwrap();
    path
}

fn create_argument() -> () {
    let a = check_args();
    unsafe {
        ARGUMENT = Some(a);
    }
}

pub fn get_argument() -> Option<Args> {
    unsafe {
        let k = ARGUMENT.clone();
        k
    }
}

pub fn check_args() -> Args {
    let arguments = "Please, use this arguments: mts, beeline";
    let matches = App::new("web_parser_rust")
        .version("1.0.0")
        .author("rummolprod999")
        .about("web_parser_rust")
        .arg(Arg::with_name("argument")
            .short("-a")
            .long("argument")
            .takes_value(true)
            .help(arguments))
        .get_matches();
    let a = match matches.value_of("argument") {
        None => {
            println!("empty argument, use -h for help");
            process::exit(0x0100);
        }
        Some(s) => match s {
            "mts" => Args::Mts,
            "beeline" => Args::Beeline,
            _ => {
                println!("bad argument, use -h for help");
                process::exit(0x0100);
            }
        }
    };
    a
}