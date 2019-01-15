extern crate chrono;

use crate::toolslib::regextools::RegexTools;
use chrono::prelude::*;
use chrono::Duration;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, UTC};

pub struct DateTimeTools {}

impl DateTimeTools {
    pub fn get_date_from_string(s: &str, pattern: &str) -> Option<DateTime<FixedOffset>> {
        let res = NaiveDate::parse_from_str(s, pattern);
        let dt = match res {
            Ok(v) => {
                let t = NaiveTime::from_hms(0, 0, 0);
                let d = NaiveDateTime::new(v, t);
                let dtt = DateTime::from_utc(d, FixedOffset::east(3 * 3600));
                let m = match dtt.checked_sub_signed(Duration::hours(3)) {
                    Some(r) => Some(r),
                    None => None,
                };
                m
            }
            Err(e) => {
                warn!("{}", e);
                None
            }
        };

        dt
    }

    pub fn get_datetime_from_string(s: &str, pattern: &str) -> Option<DateTime<FixedOffset>> {
        let res = NaiveDateTime::parse_from_str(s, pattern);
        let dt = match res {
            Ok(v) => {
                let dtt = DateTime::from_utc(v, FixedOffset::east(3 * 3600));
                let m = match dtt.checked_sub_signed(Duration::hours(3)) {
                    Some(r) => Some(r),
                    None => None,
                };
                m
            }
            Err(e) => {
                warn!("{}", e);
                None
            }
        };

        dt
    }

    pub fn return_min_datetime() -> DateTime<FixedOffset> {
        let d = DateTime::parse_from_rfc3339("1970-01-01T00:00:00+03:00").unwrap();
        d
    }

    pub fn return_datetime_now() -> DateTime<FixedOffset> {
        let d = UTC::now();
        let dt = d.with_timezone(&FixedOffset::east(3 * 3600));
        dt
    }

    pub fn replace_str_months(st: &str) -> Option<String> {
        let res = match st {
            x if x.contains("января") => x.replace("января", ".01."),
            x if x.contains("февраля") => x.replace("февраля", ".02."),
            x if x.contains("марта") => x.replace("марта", ".03."),
            x if x.contains("апреля") => x.replace("апреля", ".04."),
            x if x.contains("мая") => x.replace("мая", ".05."),
            x if x.contains("июня") => x.replace("июня", ".06."),
            x if x.contains("июля") => x.replace("июля", ".07."),
            x if x.contains("августа") => x.replace("августа", ".08."),
            x if x.contains("сентября") => x.replace("сентября", ".09."),
            x if x.contains("октября") => x.replace("октября", ".10."),
            x if x.contains("ноября") => x.replace("ноября", ".11."),
            x if x.contains("декабря") => x.replace("декабря", ".12."),
            _ => String::new(),
        };
        let m = RegexTools::del_all_ws(&res);
        m
    }
}
