use regex::{Regex, RegexBuilder};
use std::{
    fmt::Display,
    io::{self, prelude::*, BufReader},
};
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
    T: CreateObject,
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
    let output = func.new_object(buf, kind);
    match output {
        OutputKind::Pattern(pattern) => Ok(pattern.to_string()),
        OutputKind::Hay(hay) => Ok(hay.to_string()),
        OutputKind::Default => Ok(String::new()),
    }
}
struct RegexContainer {
    pattern: String,
}
impl RegexContainer {
    fn new(pattern: String) -> RegexContainer {
        RegexContainer { pattern }
    }
    fn sanitize(mut self) -> Self {
        self.pattern = self.pattern.trim_end_matches("\n").to_string();
        self
    }
}
impl Display for RegexContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"{}", self.pattern)
    }
}

struct HayContainer {
    hay: String,
}
impl HayContainer {
    fn new(hay: String) -> HayContainer {
        HayContainer { hay }
    }
    fn get(&self) -> &str {
        self.hay.as_str()
    }
}
trait CreateObject {
    fn new_object(&self, s: String, kind: InputKind) -> OutputKind {
        match kind {
            InputKind::Pattern => OutputKind::Pattern(UserPattern { pattern: s }),
            InputKind::Hay => OutputKind::Hay(UserHay { hay: s }),
        }
    }
}
struct UserPattern {
    pattern: String,
}
impl UserPattern {
    fn new() -> UserPattern {
        UserPattern {
            pattern: String::new(),
        }
    }
}
impl CreateObject for UserPattern {
    fn new_object(&self, s: String, kind: InputKind) -> OutputKind {
        match kind {
            InputKind::Pattern => OutputKind::Pattern(UserPattern { pattern: s }),
            _ => OutputKind::Default,
        }
    }
}
impl Display for UserPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"{}", self.pattern)
    }
}
struct UserHay {
    hay: String,
}
impl UserHay {
    fn new() -> UserHay {
        UserHay { hay: String::new() }
    }
}
impl CreateObject for UserHay {
    fn new_object(&self, s: String, kind: InputKind) -> OutputKind {
        match kind {
            InputKind::Hay => OutputKind::Hay(UserHay { hay: s }),
            _ => OutputKind::Default,
        }
    }
}
impl Display for UserHay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hay)
    }
}
enum InputKind {
    Pattern,
    Hay,
}
enum OutputKind {
    Pattern(UserPattern),
    Hay(UserHay),
    Default,
}
