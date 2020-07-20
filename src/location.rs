use std::fmt;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Location {
    city: String,
    state: String,
    country: String,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{},{}]", self.city, self.state, self.country)
    }
}

impl Location {
    pub fn city<'a, 'b>(&'a mut self, city: &'b str) -> &'a mut Self {
        self.city = city.to_owned();
        self
    }
    pub fn get_city(&self) -> &str {
        self.city.as_str()
    }
    pub fn state<'a, 'b>(&'a mut self, state: &'b str) -> &'a mut Self {
        self.state = state.to_owned();
        self
    }
    pub fn get_state(&self) -> &str {
        self.state.as_str()
    }
    pub fn country<'a, 'b>(&'a mut self, country: &'b str) -> &'a mut Self {
        self.country = country.to_owned();
        self
    }
    pub fn get_country(&self) -> &str {
        self.country.as_str()
    }
}
