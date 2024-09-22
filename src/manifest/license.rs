use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum License {
    License(String),
    Custom(CustomLicense),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CustomLicense {
    pub identifier: Option<String>,
    pub url: Option<String>,
}
