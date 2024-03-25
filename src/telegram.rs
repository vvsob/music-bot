use std::sync::{Arc, Mutex};
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{player::MusicPlayer, download};

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
            Command::Play(url) => {
                match download::download_from_youtube(&url) {
                    Ok(track_info) => {
                        player.lock().unwrap().enqueue(track_info);
                        bot.send_message(msg.chat.id, "Added to the queue.").await?;
                    },
                    Err(_) => {
                        bot.send_message(msg.chat.id, "Failed to download.").await?;
                    }
                }
            }
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
                let tracks = player.lock().unwrap().list_tracks();
                bot.send_message(
                    msg.chat.id,
                    tracks
                        .iter()
                        .map(|t| t.path.to_str().unwrap())
                        .collect::<Vec<&str>>()
                        .join("\n"),
                )
                .await?;
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
