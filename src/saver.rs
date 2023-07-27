use tokio::time::{sleep_until, Instant, Duration};
use teloxide::{
    prelude::*, 
};

use std::process::Command;
use std::io::{self, Write};



pub async fn get_video(msg: Message, bot: Bot, _url: String) {
    sleep_until(Instant::now() + Duration::from_millis(100)).await;
    bot.send_message(msg.chat.id, "Spacecraft observer to finding next landing land 🛸 ☄️. Please try later").await;

    let result = Command::new("ytdl")
        .args(["-S", "ext","--compat-options", "filename", &_url])
        .output()
        .expect("failed to execute process");

    println!("status: {}", result.status);

    let filename = Command::new("ytdl")
        .args(["--print", "filename", "--compat-options", "filename",&_url])
        .output()
        .expect("failed to execute process");
 
    let mut binary_path = String::new();
    binary_path = String::from_utf8(filename.stdout).unwrap();

    bot.send_message(msg.chat.id, format!("Spacecraft landing on asteroid  🛸 ☄️ {} Congratulation.", binary_path)).await;
}