use std::fmt::Display;

pub fn format_vec_string<T>(v: &[T]) -> String where T: Display {
    if v.len() == 0 {
        return String::from("");
    }

    let mut iter = v.iter();
    let mut s = format!("{}", iter.next().unwrap());

    for card in iter {
        s = format!("{}, {}", s, card);
    }

    s
}
