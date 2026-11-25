use url::form_urlencoded::Serializer;

#[derive(Debug, Default)]
pub struct FormData {
    fields: Vec<(String, String)>,
}

impl FormData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.fields.push((name.into(), value.into()));
    }

    pub fn extend(&mut self, other: Self) {
        self.fields.extend(other.fields);
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(field_name, _)| field_name == name)
            .map(|(_, value)| value.as_str())
    }

    pub fn encode(&self) -> String {
        let mut serializer = Serializer::new(String::new());

        for (name, value) in &self.fields {
            serializer.append_pair(name, value);
        }

        serializer.finish()
    }
}
