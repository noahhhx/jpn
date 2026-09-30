//! The shapes the itinerary is built from, plus the date helpers the pages need.
//!
//! The content itself lives in `itinerary.rs`. Day `n` falls on `START + n`.

pub use crate::itinerary::{DAYS, INFO, START};

pub struct Day {
    pub city: &'static str,
    pub title: &'static str,
    pub sections: &'static [Section],
}

/// A titled chunk of a page. An empty `heading` renders without one.
pub struct Section {
    pub heading: &'static str,
    pub blocks: &'static [Block],
}

pub enum Block {
    /// A paragraph. Newlines are kept as line breaks.
    Text(&'static str),
    /// Something that must not be missed.
    Note(&'static str),
    /// Bulleted things to do or know, each with an optional note and link.
    List(&'static [Entry]),
    /// Short single-line items, laid out in columns.
    Checklist(&'static [&'static str]),
    /// A timed plan.
    Schedule(&'static [Step]),
    /// Somewhere to go, with a Google Maps link.
    Place(Place),
    Links(&'static [Link]),
    /// An image under `public/`, e.g. `/img/tokyo-map.webp`.
    Image { src: &'static str, caption: &'static str },
}

pub struct Entry {
    pub name: &'static str,
    pub note: &'static str,
    pub url: &'static str,
}

pub struct Step {
    pub time: &'static str,
    pub title: &'static str,
    pub note: &'static str,
}

pub struct Place {
    pub name: &'static str,
    pub address: &'static str,
    pub note: &'static str,
}

pub struct Link {
    pub label: &'static str,
    pub url: &'static str,
}

pub const fn item(name: &'static str, note: &'static str) -> Entry {
    Entry { name, note, url: "" }
}

impl Entry {
    pub const fn url(self, url: &'static str) -> Self {
        Entry { url, ..self }
    }
}

pub const fn step(time: &'static str, title: &'static str, note: &'static str) -> Step {
    Step { time, title, note }
}

pub const fn place(name: &'static str, address: &'static str, note: &'static str) -> Block {
    Block::Place(Place { name, address, note })
}

pub const fn link(label: &'static str, url: &'static str) -> Link {
    Link { label, url }
}

pub const fn image(src: &'static str, caption: &'static str) -> Block {
    Block::Image { src, caption }
}

impl Place {
    /// Google Maps search for the address (or the name, if there's no address).
    pub fn maps_url(&self) -> String {
        let query = if self.address.is_empty() { self.name } else { self.address };
        let mut url = String::from("https://www.google.com/maps/search/?api=1&query=");
        for b in query.bytes() {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    url.push(b as char)
                }
                b' ' => url.push('+'),
                _ => url.push_str(&format!("%{b:02X}")),
            }
        }
        url
    }
}

impl Link {
    /// "example.com" for display next to the label.
    pub fn host(&self) -> &'static str {
        let rest = self.url.split_once("://").map_or(self.url, |(_, r)| r);
        let host = rest.split(['/', '?', '#']).next().unwrap_or(rest);
        host.strip_prefix("www.").unwrap_or(host)
    }
}

/// A calendar date, stored as days since 1970-01-01.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(i64);

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

impl Date {
    pub fn from_ymd((y, m, d): (i32, u32, u32)) -> Self {
        // Howard Hinnant's days_from_civil.
        let y = if m <= 2 { y - 1 } else { y } as i64;
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let m = m as i64;
        let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        Date(era * 146097 + doe - 719468)
    }

    pub fn ymd(self) -> (i32, u32, u32) {
        // Howard Hinnant's civil_from_days.
        let z = self.0 + 719468;
        let era = z.div_euclid(146097);
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        let y = (yoe + era * 400 + if m <= 2 { 1 } else { 0 }) as i32;
        (y, m, d)
    }

    pub fn add_days(self, n: i64) -> Self {
        Date(self.0 + n)
    }

    pub fn weekday(self) -> &'static str {
        WEEKDAYS[(self.0 + 4).rem_euclid(7) as usize]
    }

    /// "Sat 10 Oct"
    pub fn short(self) -> String {
        let (_, m, d) = self.ymd();
        format!("{} {} {}", self.weekday(), d, MONTHS[m as usize - 1])
    }

    /// "10 Oct 2026"
    pub fn long(self) -> String {
        let (y, m, d) = self.ymd();
        format!("{} {} {}", d, MONTHS[m as usize - 1], y)
    }

    /// Today's date in Japan (UTC+9), or `None` if the clock is unavailable.
    pub fn today_in_japan() -> Option<Self> {
        const JST_OFFSET_MS: i64 = 9 * 60 * 60 * 1000;
        const MS_PER_DAY: i64 = 24 * 60 * 60 * 1000;
        now_ms().map(|ms| Date((ms + JST_OFFSET_MS).div_euclid(MS_PER_DAY)))
    }
}

#[cfg(feature = "ssr")]
fn now_ms() -> Option<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_millis() as i64)
}

#[cfg(all(not(feature = "ssr"), feature = "hydrate"))]
fn now_ms() -> Option<i64> {
    Some(js_sys::Date::now() as i64)
}

#[cfg(not(any(feature = "ssr", feature = "hydrate")))]
fn now_ms() -> Option<i64> {
    None
}

/// Date of day `n` (Day 0 is `START`).
pub fn date_of(n: usize) -> Date {
    Date::from_ymd(START).add_days(n as i64)
}

/// Look up day `n`.
pub fn day(n: usize) -> Option<&'static Day> {
    DAYS.get(n)
}

/// Number of the last day.
pub fn last() -> usize {
    DAYS.len() - 1
}

/// The day number for today, if today is during the trip.
pub fn today_number() -> Option<usize> {
    let today = Date::today_in_japan()?;
    (0..DAYS.len()).find(|&n| date_of(n) == today)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_roundtrip() {
        for ymd in [(1970, 1, 1), (2000, 2, 29), (2026, 10, 10), (2026, 12, 31)] {
            assert_eq!(Date::from_ymd(ymd).ymd(), ymd);
        }
        assert_eq!(Date::from_ymd((1970, 1, 1)).0, 0);
    }

    #[test]
    fn weekday_and_rollover() {
        assert_eq!(Date::from_ymd((2026, 10, 10)).weekday(), "Sat");
        assert_eq!(Date::from_ymd((2026, 10, 31)).add_days(1).ymd(), (2026, 11, 1));
    }

    #[test]
    fn trip_dates() {
        assert_eq!(date_of(0).short(), "Wed 14 Oct");
        assert_eq!(date_of(last()).short(), "Mon 9 Nov");
    }

    #[test]
    fn maps_url_encodes() {
        let p = Place { name: "", address: "828番地1 Yufu, Oita", note: "" };
        assert_eq!(
            p.maps_url(),
            "https://www.google.com/maps/search/?api=1&query=828%E7%95%AA%E5%9C%B01+Yufu%2C+Oita"
        );
    }
}
