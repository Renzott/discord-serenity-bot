use chrono::{Datelike, Utc};
use std::{env, sync::Arc};

use serde::Deserialize;
use serenity::{
    all::{CreateMessage, UserId},
    client::Context,
};

use serenity::model::id::ChannelId;
use tokio::{fs::File, io::AsyncReadExt, io::Error};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::{
    utils::{common::get_random_index_from_vec, embed::build_new_birthay_embed},
    workers::structs::{BirthdayData, BirthdayUserData}
};

const BIRTHDAY_USERS_DATA_PATH: &str = "src/data/birthday/users.json";
const BIRTHDAY_DATA_PATH: &str = "src/data/birthday/data.json";

fn get_birthday_channel_id() -> u64 {
    let birthday_channel_id = env::var("DISCORD_CHANNEL").expect("BIRTHDAY_CHANNEL_ID must be set");
    birthday_channel_id
        .parse()
        .expect("BIRTHDAY_CHANNEL_ID must be a number")
}

fn check_current_date_birthday(birthday_date: String) -> bool {
    let current_date = Utc::now();
    let current_day = current_date.day();
    let current_month = current_date.month();

    let birthday_date_parts: Vec<&str> = birthday_date.split("/").collect();
    let birthday_day = birthday_date_parts[0].parse::<u32>().unwrap();
    let birthday_month = birthday_date_parts[1].parse::<u32>().unwrap();

    if current_day == birthday_day && current_month == birthday_month {
        return true;
    }
    false
}

async fn read_birthday_data<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, Error> {
    let mut file = File::open(path).await?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;

    let data: T = serde_json::from_str(&buffer)?;
    Ok(data)
}

async fn send_birthday_message(
    _context: &Context,
    birthday_data: BirthdayData,
    _channel_id: ChannelId,
    user_id: u64,
) {
    let image = birthday_data
        .images
        .get(get_random_index_from_vec(&birthday_data.images))
        .unwrap_or(&String::from("https://i.imgur.com/1QZ0Q8w.jpg"))
        .clone();
    let message = birthday_data
        .messages
        .get(get_random_index_from_vec(&birthday_data.messages))
        .unwrap_or(&String::from("Feliz cumpleaños!"))
        .clone();

    let user = UserId::from(user_id);

    let user_data = match _context.http.get_user(user).await {
        Ok(data) => data,
        Err(why) => {
            println!("Error getting user data: {:?}", why);
            return;
        }
    };

    let username = user_data.global_name.unwrap_or(user_data.name);

    let birthday_embed = build_new_birthay_embed(username, image.to_string(), message.to_string());

    let builder = CreateMessage::new().embed(birthday_embed);

    match _channel_id.send_message(&_context, builder).await {
        Ok(_) => println!("Message sent!"),
        Err(why) => println!("Error sending message: {:?}", why),
    }
}

pub async fn birthday_cron(context: Context) -> Result<(), JobSchedulerError> {
    let sched = JobScheduler::new().await?;

    let context_clone = Arc::new(context);

    let job = Job::new_async("0 0 3 * * *", move |_uuid, _lock| {
        let _channel_id = ChannelId::from(get_birthday_channel_id());
        let _context = context_clone.clone();
        Box::pin(async move {
            let birthday_data: BirthdayData = match read_birthday_data(BIRTHDAY_DATA_PATH).await {
                Ok(data) => data,
                Err(why) => {
                    println!("Error reading birthday data: {:?}", why);
                    return;
                }
            };

            let birthday_users_data: BirthdayUserData =
                match read_birthday_data(BIRTHDAY_USERS_DATA_PATH).await {
                    Ok(data) => data,
                    Err(why) => {
                        println!("Error reading birthday users data: {:?}", why);
                        return;
                    }
                };

            for user in birthday_users_data.users {
                if check_current_date_birthday(user.cron_string) {
                    let user_id = user.id.parse::<u64>().unwrap();
                    let _birthday_data = birthday_data.clone();
                    send_birthday_message(&_context, _birthday_data, _channel_id, user_id).await;
                }
            }
        })
    });

    sched.add(job?).await?;
    sched.start().await?;
    Ok(())
}
