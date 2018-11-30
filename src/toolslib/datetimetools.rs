extern crate chrono;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::Duration;
use chrono::prelude::*;

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

    pub fn return_min_datetime() -> DateTime<FixedOffset> {
        let d = DateTime::parse_from_rfc3339("1970-01-01T00:00:00+03:00").unwrap();
        d
    }

    pub fn return_datetime_now() -> DateTime<FixedOffset> {
        let d = Utc::now();
        let dt = d.with_timezone(&FixedOffset::east(3 * 3600));
        dt
    }
}