use std::io::{stdout, Write, Stdout};
use std::process;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};
use crossterm::{ExecutableCommand, terminal, execute, cursor};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use game::{Map, Game};
use tokio::sync::watch;

mod game;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameInput {
    Up,
    Down,
    Left,
    Right,
    Exit,
}

#[tokio::main]
async fn main() {
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::MoveTo(0,0)).unwrap();
    let (input_sender, input_receiver) = watch::channel(GameInput::Right);

    tokio::spawn(async move {
        loop {
            if let Event::Key(event) = event::read().unwrap() {
                let input = match event.code {
                    KeyCode::Up => Some(GameInput::Up),
                    KeyCode::Down => Some(GameInput::Down),
                    KeyCode::Left => Some(GameInput::Left),
                    KeyCode::Right => Some(GameInput::Right),
                    KeyCode::Char('q') => Some(GameInput::Exit),
                    _ => None,
                };

                if let Some(input) = input {
                    input_sender.send(input).unwrap();
                }
            }
        }
    });

    let mut game = Game::new(10,10);

    loop {
        let current_input = *input_receiver.borrow();

        if current_input == GameInput::Exit {
            break;
        }

        game.update(current_input);

        game.map.print(&mut stdout);
        println!("Moving in direction: {:?}", current_input);
        println!("Score: {}", game.snake.pos_history.len());

        tokio::time::sleep(Duration::from_millis(500)).await;
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::MoveTo(0,0)).unwrap();
    }

    stdout.execute(LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    process::exit(1);
}
