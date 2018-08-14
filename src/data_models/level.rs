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
