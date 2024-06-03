use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BirthdayUser {
    pub id: String,
    #[serde(rename = "dayMonth")]
    pub day_month: String,
}

#[derive(Deserialize, Debug)]
pub struct BirthdayUserData {
    pub users: Vec<BirthdayUser>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BirthdayData {
    pub messages: Vec<String>,
    pub images: Vec<String>,
}