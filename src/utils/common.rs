use std::{io::Error, time::Duration};

use rand::Rng;
use reqwest::Client as HttpClient;
use serde::Deserialize;
use serenity::client::Context;
use tokio::{fs::File, io::AsyncReadExt};

use crate::{models::welcome_farewell::WelcomeFarewellData, HttpKey};

pub async fn get_http_client(ctx: &Context) -> HttpClient {
    let data = ctx.data.read().await;
    let http_client = data.get::<HttpKey>().unwrap().clone();
    http_client
}

pub fn duration_to_string(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let seconds = seconds % 60;
    let minutes = minutes % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

pub fn get_random_index_from_vec<T>(vec: &Vec<T>) -> usize {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..vec.len());
    index
}

async fn read_json_data<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, Error> {
    let mut file = File::open(path).await?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;

    let data: T = serde_json::from_str(&buffer)?;
    Ok(data)
}

const WELLCOME_FAREWELL_PATH: &str = "data/welcome_farewell/data.json";
const DICE_PATH: &str = "https://raw.githubusercontent.com/Renzott/renzott/main/assets/dice_@.png";

pub fn get_dice_image_url(roll: u8) -> String {
    DICE_PATH.replace("@", &roll.to_string())   
}

pub async fn get_welcome_farewell_data() -> Result<WelcomeFarewellData, Error> {
    let data = match read_json_data::<WelcomeFarewellData>(WELLCOME_FAREWELL_PATH).await {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("Error reading welcome farewell data: {:?}", e);
            Err(e)
        }
    };
    data
}

pub fn get_random_number_from_range(min: u8, max: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(min..=max);
    number
}

pub fn get_color_for_roll(roll: u8) -> u32 {
    match roll {
        1 => 0x8B0000,       // Pifia (1)
        2..=5 => 0xFF0000,   // Muy bajo (2-5)
        6..=9 => 0xFF8C00,   // Bajo (6-9)
        10..=11 => 0xFFA500, // Medio-bajo (10-11)
        12..=14 => 0xFFFF00, // Medio (12-14)
        15..=16 => 0xADFF2F, // Medio-alto (15-16)
        17..=18 => 0x32CD32, // Alto (17-18)
        19 => 0x006400,      // Muy alto (19)
        20 => 0xFFD700,      // Crítico (20)
        _ => 0xFFFFFF,       // Default (por si acaso)
    }
}

/*

{
 "dices" : {
    "mistake": [MENSAJES DE ROL SEGUN EL RESULTADO],
    "very-low"[MENSAJES DE ROL SEGUN EL RESULTADO],
    "low": [MENSAJES DE ROL SEGUN EL RESULTADO],
    "medium-low": [MENSAJES DE ROL SEGUN EL RESULTADO],
    "medium": [MENSAJES DE ROL SEGUN EL RESULTADO],
    "medium-high": [MENSAJES DE ROL SEGUN EL RESULTADO],
    "high" [MENSAJES DE ROL SEGUN EL RESULTADO],
    "very-high": [MENSAJES DE ROL SEGUN EL RESULTADO],
    "critical":  [MENSAJES DE ROL SEGUN EL RESULTADO]
  }
}

 */
