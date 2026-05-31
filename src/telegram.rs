use std::sync::{Arc, Mutex};
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{download, player::MusicPlayer};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "play youtube url.")]
    Play(String),
    #[command(description = "stop playing.")]
    Stop,
    #[command(description = "pause playing.")]
    Pause,
    #[command(description = "resume playing.")]
    Resume,
    #[command(description = "skip the track that is playing.")]
    Skip,
    #[command(description = "list all tracks in queue.")]
    List,
}

#[derive(Clone, Default)]
enum State {
    #[default]
    Start,
}

pub struct TelegramBot {
    player: Arc<Mutex<MusicPlayer>>,
}

impl TelegramBot {
    async fn handle_command(
        bot: Bot,
        msg: Message,
        player: Arc<Mutex<MusicPlayer>>,
        cmd: Command,
    ) -> Result<(), teloxide::RequestError> {
        match cmd {
            Command::Play(url) => match download::download_from_youtube(&url) {
                Ok(track_info) => {
                    player.lock().unwrap().enqueue(track_info);
                    bot.send_message(msg.chat.id, "Added to the queue.").await?;
                }
                Err(_) => {
                    bot.send_message(msg.chat.id, "Failed to download.").await?;
                }
            },
            Command::Stop => {
                player.lock().unwrap().stop();
                bot.send_message(msg.chat.id, "Stopped.").await?;
            }
            Command::Pause => {
                player.lock().unwrap().pause();
                bot.send_message(msg.chat.id, "Paused.").await?;
            }
            Command::Resume => {
                player.lock().unwrap().play();
                bot.send_message(msg.chat.id, "Resumed.").await?;
            }
            Command::Skip => {
                player.lock().unwrap().skip_one();
                bot.send_message(msg.chat.id, "Skipped.").await?;
            }
            Command::List => {
                let (current, queue) = player.lock().unwrap().list_tracks();
                let message = match current {
                    None => "Nothing is playing.".to_string(),
                    Some(track) => if queue.is_empty() { 
                        format!("Currently playing: {} ({})", track.name, track.url) 
                    } else {
                        let queue_msg = queue
                        .iter()
                        .map(|t| format!("{} ({})", t.name, t.url))
                        .collect::<Vec<String>>()
                        .join("\n");
                        format!("Currently playing: {} ({})\nQueue:\n{}", track.name, track.url, queue_msg)
                    }
                };
                bot.send_message(msg.chat.id, message).await?;
            }
        };
        Ok(())
    }

    async fn async_telegram_main(&mut self) {
        pretty_env_logger::init();
        log::info!("Starting bot...");

        let bot = Bot::from_env();

        Dispatcher::builder(
            bot,
            Update::filter_message().branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(Self::handle_command),
            ),
        )
        .dependencies(dptree::deps![self.player.clone()])
        .build()
        .dispatch()
        .await;
    }

    pub fn telegram_main(&mut self) {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move { self.async_telegram_main().await })
    }

    pub fn build(player: MusicPlayer) -> TelegramBot {
        TelegramBot {
            player: Arc::new(Mutex::new(player)),
        }
    }
}
