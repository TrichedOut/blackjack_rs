use std::fmt::Display;

/**
 * Formats a vector to a string of its elements.
 * Example: Vec<u32>{1, 2, 3, 4} -> "1, 2, 3, 4"
 */
pub fn format_vec_string<T>(v: &[T]) -> String where T: Display {
    // if there are no elements, return empty
    if v.len() == 0 {
        return String::from("");
    }

    // create the iter, set the first element
    let mut iter = v.iter();
    let mut s = format!("{}", iter.next().unwrap());

    // for the remaining elements, append ', {}'
    for lmnt in iter {
        s = format!("{}, {}", s, lmnt);
    }

    s
}
