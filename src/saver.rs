use tokio::time::{sleep_until, Instant, Duration};
use teloxide::{
    prelude::*, 
};

use std::process::Command;



pub async fn get_video(msg: Message, bot: Bot) {
    sleep_until(Instant::now() + Duration::from_millis(100)).await;
    let output = Command::new("pwd")
        .spawn()
        .expect("pwd command failed to start");

    let output = Command::new("ls")
        .arg("-a")
        .arg("-l")
        .spawn()
        .expect("cwd command failed to start");
    bot.send_message(msg.chat.id, "Spacecraft observer to finding next landing land 🛸 ☄️. Please try later").await;

    Command::new("ytdl --format-sort https://www.youtube.com/watch?v=SV2myatYA5c")
        .spawn()
        .expect("ytdl command failed to start");

    bot.send_message(msg.chat.id, "Spacecraft crash into asteroid 🛸 ☄️. Please try later").await;
}