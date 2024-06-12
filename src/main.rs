mod commands;
mod utils;
mod workers;
mod models;

use std::env;

use crate::commands::playlist::{
    loop_song::*,
    now_playing::*,
    play::*,
    resume::*,
    stop::*,
    skip::*,
    clear::*,
};

use crate::commands::voice_channel::{
    join::*,
    leave::*,
};

use crate::workers::birthday::birthday_cron;
use reqwest::Client as HttpClient;

use serenity::{
    all::{CreateMessage, Guild, GuildId, Member, Reaction, ReactionType, User}, async_trait, client::{Client, Context, EventHandler}, framework::{
        standard::{
            macros::{command, group},
            CommandResult, Configuration,
        },
        StandardFramework,
    }, model::{channel::Message, gateway::Ready }, prelude::{GatewayIntents, TypeMapKey},
    Result as SerenityResult,
};

use songbird::{
    input::AuxMetadata, SerenityInit
};
use tracing::{info, warn};
use utils::embed::build_new_user_embed;

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

struct AuxMetadataKey;
impl TypeMapKey for AuxMetadataKey {
    type Value = AuxMetadata;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        tokio::spawn(async move {
            let _ = birthday_cron(context).await;
        });
        info!("{} is connected!", ready.user.name);
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member)  {
            
        let guild_id = new_member.guild_id;
        let context = ctx.clone();
    
        let username_id = new_member.user.id;


        let guild: Guild = match ctx.cache.guild(guild_id) {
            Some(guild) => guild.clone(),
            None => {
                warn!("Guild not found");
                return;
            }
        };

        let channel_id = match  guild.system_channel_id {
            Some(channel_id) => channel_id,
            None => {
                warn!("System channel not found");
                return;
            }
        };

        let context_http = context.http.clone();

        let embed = build_new_user_embed(username_id).await;

        if let Ok(embed) = embed {
            let message = CreateMessage::new().embed(embed);

            let _ = channel_id.send_message(context_http, message).await;
        }
        
    }

    async fn guild_member_removal(&self, _ctx: Context, _guild_id: GuildId, user: User, _member_data_if_available: Option<Member>){
        // print the member that left
        info!("Member left: {:?}", user.name);
    }

    // add event to Interactions with the bot (react emojis)
    async fn reaction_add(&self, _ctx: Context, reaction: Reaction) {
        let reaction = &reaction.clone();

        // if is a bot reaction, return
        if let Some(member) = &reaction.member {
           if member.user.bot {
               return;
           }
        }

        let emoji = reaction.clone().emoji;
        println!("Reaction: {:?}", emoji);
        println!("Emote: {:?}", '🎉');
        if emoji == ReactionType::Unicode("🎉".to_string()) {
            println!("Reaction detected");
            println!("Reaction: {:?}", reaction);
        }
    }
}


#[group]
#[commands(
    play_song,
    pause_song,
    resume_song,
    skip_song,
    pause_song,
    loop_song,
    now_playing,
    stop_and_clear
)]
struct PlaylistCommand;

#[group]
#[commands(
    join_channel,
    leave_channel
)]
struct VoiceChannelCommand;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let prefix = match env::var("DISCORD_PREFIX") {
        Ok(val) => val,
        Err(_) => "-".to_string(),
    };

    let framework = StandardFramework::new().group(&VOICECHANNELCOMMAND_GROUP).group(&PLAYLISTCOMMAND_GROUP);
    framework.configure(Configuration::new().prefix(prefix));

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Err creating client");

    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));

    let _signal_err = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Pong!").await);

    Ok(())
}

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        warn!("Error sending message: {:?}", why);
    }
}
