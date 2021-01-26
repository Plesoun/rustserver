use std::collections::HashMap;

pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, Value<'buffer>>
}

pub enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>)
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// query string example 'a=1&b=2&c&d=&e===&d=7&d=abc'

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(s: &'buffer str) -> Self {
        let mut data = HashMap::new();


        for sub_string in s.split("&") {
            let mut key = sub_string;
            let mut value = "";
            if let Some(i) = s.find("=") {
                key = &sub_string[..i];
                value = &sub_string[i + 1..];
            }
        }
        QueryString { data }
    }
}
