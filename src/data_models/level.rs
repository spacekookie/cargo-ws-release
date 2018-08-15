pub static MAJOR: &'static str = "major";
pub static MINOR: &'static str = "minor";
pub static PATCH: &'static str = "patch";
pub static ALPHA: &'static str = "alpha";
pub static BETA: &'static str = "beta";
pub static RC: &'static str = "rc";

#[derive(Debug, PartialEq)]
pub enum Level {
    Major,
    Minor,
    Patch,
    RC,
    Beta,
    Alpha,
}

impl Level {
    pub fn from_str(s: &str) -> Option<Level> {
        match s {
            "major" => Some(Level::Major),
            "minor" => Some(Level::Minor),
            "patch" => Some(Level::Patch),
            "rc" => Some(Level::RC),
            "beta" => Some(Level::Beta),
            "alpha" => Some(Level::Alpha),
            _ => None,
        }
    }
}
