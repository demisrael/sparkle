use std::fmt;

pub struct Url(String);

impl From<&str> for Url {
    fn from(url: &str) -> Self {
        Url(url.to_string())
    }
}

impl From<String> for Url {
    fn from(url: String) -> Self {
        Url(url)
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Url {
    pub fn join<P: fmt::Display>(&self, path: P) -> Url {
        Url(format!("{}{}", self.0, path))
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Url> for String {
    fn from(url: Url) -> Self {
        url.0
    }
}
