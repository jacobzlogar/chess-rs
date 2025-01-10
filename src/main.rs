use chess_rs::{boards::bishop_moves, Game, Result};
use std::io::{self, Write};


fn main() -> Result<()> {
    // bishop_moves();
    // return Ok(());
    let game = Game::default();
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        if let Some(_) = input.find("display") {
            println!("{game}");
        }
        if let Some((command, rest)) = input.split_once(' ') {
            match command {
                "query" => {
                    if let Ok(position) = rest.trim().parse::<u64>() {
                        game.query(position);
                    }
                },
                "move" => {
                    if let Some((src, dest)) = rest.split_once(' ') {
                        match (src.trim().parse::<u64>(), dest.trim().parse::<u64>()) {
                            (Ok(start), Ok(end)) => {
                                game.advance(start, end);
                            },
                            (Ok(_), Err(_)) => {
                                println!("invalid ending coordinate")
                            },
                            (Err(_), Ok(_)) => {
                                println!("invalid starting coordinate")
                            }, 
                            _ => println!("unexpected error occurred")
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
