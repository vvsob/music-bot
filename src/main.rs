use music_bot::{player::MusicPlayer, telegram};

fn main() {
    let player = MusicPlayer::new();
    let mut bot = telegram::TelegramBot::build(player);
    bot.telegram_main();
}
