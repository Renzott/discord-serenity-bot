use std::{sync::Arc, time::Duration};

use serenity::{
    all::{Context, Guild, Http, UserId},
    async_trait,
};
use songbird::{Event, EventContext, EventHandler, Songbird};
use tokio::{sync::Mutex, task::JoinHandle, time};
use tracing::info;

pub struct ClientDisconnectNotifier {
    pub bot_id: UserId,
    pub guild: Guild,
    pub http: Arc<Http>,
    pub songbird_context: Arc<Songbird>,
    pub serenity_context: Arc<Context>,
    pub debounce: Arc<Mutex<Option<JoinHandle<()>>>>,
}

#[async_trait]
impl EventHandler for ClientDisconnectNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let debounce = Arc::clone(&self.debounce);

        if let Some(handle) = debounce.lock().await.take() {
            handle.abort();
        }

        let bot_id = self.bot_id;
        let guild_id  = self.guild.id;
        let context_arc = Arc::clone(&self.serenity_context);
        let manager_arc = Arc::clone(&self.songbird_context);

        let handle = tokio::spawn(async move {
            time::sleep(Duration::from_secs(60)).await;

            let cache = context_arc.cache.clone();
            let current_guild = cache.guild(guild_id).unwrap().clone();

            let current_channel_len: Option<usize> = current_guild
                .voice_states
                .get(&bot_id)
                .and_then(|voice_state| voice_state.channel_id)
                .map(|channel_id| {
                    current_guild
                        .voice_states
                        .iter()
                        .filter(|(_, v)| v.channel_id == Some(channel_id))
                        .count()
                });
            if let Some(channel_len) = current_channel_len {
                if channel_len == 1 {
                    let has_handler = manager_arc.get(guild_id).is_some();

                    if has_handler {
                        if let Err(e) = manager_arc.remove(guild_id).await {
                            info!("Failed: {:?}", e);
                        }
                    }
                }
            }
        });

        *debounce.lock().await = Some(handle);
        None
    }
}
