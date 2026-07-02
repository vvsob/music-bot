use std::sync::Arc;
use tokio::sync::Mutex;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{download::{Downloader}, player::{MusicPlayer}};

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

pub struct TelegramBot {
    player: Arc<Mutex<MusicPlayer>>,
    downloader: Arc<Mutex<Downloader>>,
}

impl TelegramBot {
    async fn handle_command(
        bot: Bot,
        msg: Message,
        player: Arc<Mutex<MusicPlayer>>,
        downloader: Arc<Mutex<Downloader>>,
        cmd: Command,
    ) -> Result<(), teloxide::RequestError> {
        match cmd {
            Command::Play(url) => {
                let reply_future = bot.send_message(msg.chat.id, "Downloading...").send();
                let mut downloader_ref = downloader.lock().await; 
                let download_future = downloader_ref.download_from_youtube(&url);
                let (reply_result, download_result) = tokio::join!(reply_future, download_future);
                let reply = reply_result?;
                match download_result {
                    Ok(track) => {
                        let name = track.info.name.clone();
                        player.lock().await.enqueue(track);
                        bot.edit_message_text(msg.chat.id, reply.id, format!("Added to the queue: {}", name)).await?;
                    }
                    Err(_) => {
                        bot.edit_message_text(msg.chat.id, reply.id, "Failed to download.").await?;
                    }
                }
            },
            Command::Stop => {
                player.lock().await.stop();
                bot.send_message(msg.chat.id, "Stopped.").await?;
            }
            Command::Pause => {
                player.lock().await.pause();
                bot.send_message(msg.chat.id, "Paused.").await?;
            }
            Command::Resume => {
                player.lock().await.play();
                bot.send_message(msg.chat.id, "Resumed.").await?;
            }
            Command::Skip => {
                player.lock().await.skip_one();
                bot.send_message(msg.chat.id, "Skipped.").await?;
            }
            Command::List => {
                let (current, queue) = player.lock().await.list_tracks();
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
        .dependencies(dptree::deps![self.player.clone(), self.downloader.clone()])
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
            downloader: Arc::new(Mutex::new(Downloader::new()))
        }
    }
}
