use std::time::Duration;

use rand::Rng;
use serenity::client::Context;
use reqwest::Client as HttpClient;

use crate::HttpKey;

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