use music_bot::{player::MusicPlayer, telegram};

fn main() {
    let player = MusicPlayer::build();
    let mut bot = telegram::TelegramBot::build(player);
    bot.telegram_main();
}
