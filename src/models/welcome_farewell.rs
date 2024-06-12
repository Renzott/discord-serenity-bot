use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct WelcomeFarewellData {
    pub welcome_messages: Vec<String>,
    pub farewell_messages: Vec<String>,
    pub dices: DiceData,
}

// serde string-string
#[derive(Deserialize, Debug, Clone, Default)]
pub struct DiceData {
    pub mistake: Vec<String>,
    #[serde(rename = "very-low")]
    pub very_low: Vec<String>,
    pub low: Vec<String>,
    #[serde(rename = "medium-low")]
    pub medium_low: Vec<String>,
    pub medium: Vec<String>,
    #[serde(rename = "medium-high")]
    pub medium_high: Vec<String>,
    pub high: Vec<String>,
    #[serde(rename = "very-high")]
    pub very_high: Vec<String>,
    pub critical: Vec<String>,
}

