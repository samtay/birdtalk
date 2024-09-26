use std::fmt::Display;

use chrono::NaiveDate;

pub fn join(values: impl IntoIterator<Item = impl Display>, sep: impl Display) -> String {
    use std::fmt::Write;

    let mut s = String::new();
    let mut iter = values.into_iter();
    if let Some(v) = iter.next() {
        write!(s, "{v}").unwrap();
        for v in iter {
            write!(s, "{sep}{v}").unwrap();
        }
    }
    s
}

pub fn today() -> NaiveDate {
    chrono::offset::Local::now().date_naive()
}
