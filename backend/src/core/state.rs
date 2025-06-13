use std::collections::HashMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub app_title: String,
    pub app_version: &'static str,
    pub data: HashMap<String, String>,
    pub debug: bool,
}

impl State {
    pub fn new(app_title: &str) -> Self {
        let data = HashMap::new();

        Self {
            app_title: app_title.to_owned(),
            app_version: clap::crate_version!(),
            data,
            debug: cfg!(debug_assertions),
        }
    }

    pub fn with_data(mut self, key: &str, value: impl ToString) -> Self {
        self.data.insert(key.to_owned(), value.to_string());
        self
    }
}
