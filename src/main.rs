use crossterm::terminal::{Clear, ClearType::CurrentLine};
use crossterm::{cursor, ExecutableCommand};
use hello_world::{printin_time, sparkle};
use std::io::stdout;

// define a closure which takes a usize + a loop of what you want to print
// call the printin_time function and pass your closure,
// tweak the numbers - number of loops, length and speed

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // hello world
    let h = "Hello, World!";
    let h_char: Vec<char> = h.clone().chars().collect();

    // the closures are defined here (plus the sparkle func in lib.rs)
    let print_chars = |mapped_value: usize| {
        for ch in h_char.iter().take(mapped_value + 1) {
            print!("{}", ch);
        }
    };
    let print_hellos = |mapped_value: usize| {
        for _ in 0..mapped_value {
            print!("{} ", h)
        }
    };

    // this is where the stuff runs
    printin_time(h_char.len() * 3, h_char.len(), 25, print_chars)?;
    printin_time(12, 4, 80, print_hellos)?;
    for (i, l) in (1..=5).rev().enumerate() {
        printin_time(16, 2, 25 * l, sparkle(h_char.clone(), i + 1))?
    }
    printin_time(32, 2, 15, sparkle(h_char.clone(), 6))?;

    stdout().execute(Clear(CurrentLine))?;
    stdout().execute(cursor::MoveToColumn(0))?;
    println!("{h}");

    Ok(())
}
