extern crate chrono;

use crate::toolslib::regextools::RegexTools;
use chrono::prelude::*;
use chrono::Duration;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, UTC};

pub struct DateTimeTools {}

impl DateTimeTools {
    ///
    /// Parses a date string into a `DateTime<FixedOffset>` object using the provided pattern.
    ///
    /// # Arguments
    /// - `s`: A string slice representing the date to be parsed, e.g., `"2023-10-10"`.
    /// - `pattern`: A string slice representing the date format pattern, e.g., `"%Y-%m-%d"`.
    ///
    /// # Returns
    /// - `Option<DateTime<FixedOffset>>`: If the parsing is successful, returns a `DateTime` object
    ///   with a fixed offset of 3 hours (east). Otherwise, returns `None` if parsing fails or an error occurs.
    ///
    /// # Remarks
    /// - The function uses the `NaiveDate::parse_from_str` method to parse the input date string based
    ///   on the provided pattern.
    /// - The time part is set to `00:00:00` unless explicitly included in the input pattern.
    /// - After creating the `DateTime<FixedOffset>` object with a fixed offset of 3 hours east,
    ///   the function subtracts 3 hours from it to ensure the final result aligns correctly with the original input date.
    ///
    /// # Example
    /// ```rust
    /// use chrono::{DateTime, FixedOffset};
    ///
    /// let date_str = "2023-10-10";
    /// let pattern = "%Y-%m-%d";
    /// let result = get_date_from_string(date_str, pattern);
    ///
    /// match result {
    ///     Some(date_time) => println!("Parsed date: {}", date_time),
    ///     None => println!("Failed to parse date"),
    /// }
    /// ```
    ///
    /// # Errors
    /// - Emits a warning log if the date string cannot be parsed with the given pattern.
    ///
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
