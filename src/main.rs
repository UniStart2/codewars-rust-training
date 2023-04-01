mod array_diff;
mod unique_in_order;
mod your_order_please;
mod reverse_words;
mod find_next_square;
mod double_integer;
mod string_with_end;
mod rectangle_into_square;
mod range_extraction;
mod number_of_trailing_zeros_of_n;
mod going_to_zero_or_to_infinity;
mod common_denominator;
mod all_balanced_parentheses;


use std::cmp::Ordering;
use std::collections::BTreeMap;
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Name {
    pub name: String,
    pub flags: u32,
}

impl Name {
    pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            flags,
        }
    }
}

fn main() {
    // let mut map = BTreeMap::new();
    // map.insert(Name::new("/etc/password", 0x1), 12);
    // map.insert(Name::new("/etc/hosts", 0x1), 4);
    // map.insert(Name::new("/home/tchen", 0x0), 28);
    //
    // for item in map.iter() {
    //     println!("{:?}", item);
    // }
}
