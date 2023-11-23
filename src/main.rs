use std::{error::Error, io, sync::mpsc, thread, time::Duration};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::{self, new_frame, Drawable},
    player::Player,
    render,
};
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

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    // channel to communicate
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        while let Ok(curr_frame) = render_rx.recv() {
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
        // better way is to use while let, I just keep this for anyone who might want to compare it
        // loop {
        //     let curr_frame = match render_rx.recv() {
        //         Ok(x) => x,
        //         Err(_) => break,
        //     };
        // render::render(&mut stdout, &last_frame, &curr_frame, false);
        // last_frame = curr_frame;
        // }
    });

    // Game Loop
    let mut player = Player::new();
    'gameloop: loop {
        // Per-frame init
        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Draw and Render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame); // ignore errors rather than terminating the thread
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
