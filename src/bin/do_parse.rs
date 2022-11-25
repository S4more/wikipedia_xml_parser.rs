use once_cell::sync::Lazy;
use regex::Regex;

static TAG_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\[\[).+?(]|\|)").unwrap());

static NAME_EXCLUDER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\(disambiguation\)|File:|.+:.+").unwrap());

fn parse_into_pages() {}

pub fn main() {}
