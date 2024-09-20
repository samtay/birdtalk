use std::fmt::Display;

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
