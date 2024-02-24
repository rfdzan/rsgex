use regex::{Regex, RegexBuilder};
use rsgex::prelude::*;
use std::io::{self, prelude::*, BufReader};
fn main() {
    let (pattern, hay) = get_user_input();
    let build_regex = build_regex(pattern);
    start_matching(build_regex, hay)
}
fn start_matching(regex: Regex, hay: HayContainer) {
    let finds = regex
        .find_iter(hay.get())
        .map(|f| f.as_str().to_string())
        .collect::<Vec<String>>();
    for f in finds.into_iter() {
        println!("{f}");
    }
}
fn build_regex(regex: RegexContainer) -> Regex {
    RegexBuilder::new(regex.sanitize().to_string().as_str())
        .case_insensitive(true)
        .multi_line(true)
        .unicode(true)
        .build()
        .expect("A RegexBuilder error occured: \n")
}
fn get_user_input() -> (RegexContainer, HayContainer) {
    let pattern = collector(UserPattern::new(), InputKind::Pattern).unwrap();
    let hay = collector(UserHay::new(), InputKind::Hay).unwrap();
    (RegexContainer::new(pattern), HayContainer::new(hay))
}
fn collector<T>(func: T, kind: InputKind) -> io::Result<String>
where
    T: CreateOutput,
{
    match kind {
        InputKind::Pattern => {
            println!("Input Pattern: ");
        }
        InputKind::Hay => {
            println!("Input Hay: ");
        }
    }
    let stdin = io::stdin().lock();
    let mut buf = String::new();
    let mut reader = BufReader::new(stdin);
    reader.read_line(&mut buf)?;
    let output = func.new_output(buf, kind);
    match output {
        OutputKind::Pattern(pattern) => Ok(pattern.to_string()),
        OutputKind::Hay(hay) => Ok(hay.to_string()),
        OutputKind::Default => Ok(String::new()),
    }
}
