use std::error::Error;

use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "./Audio/explode.wav");
    audio.add("lose", "./Audio/lose.wav");
    audio.add("move", "./Audio/move.wav");
    audio.add("pew", "./Audio/pew.wav");
    audio.add("startup", "./Audio/startup.wav");
    audio.add("win", "./Audio/win.wav");

    audio.play("startup");

    // Cleanup
    audio.wait();

    Ok(())
}
