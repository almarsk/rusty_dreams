use crossterm::terminal::{Clear, ClearType::CurrentLine};
use crossterm::{cursor, ExecutableCommand};
use hello_world::{chain_iter, printin_time};
use std::io::stdout;

// define a closure which takes a usize + a loop of what you want to print
// call the printin_time function and pass your closure,
// tweak the numbers - number of loops, length and speed

// I procrastinated from other stuff doing this assignment, that's why it's so dragged out

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // hello world
    let h = "Hello, World!";
    let h_char: Vec<char> = h.clone().chars().collect();

    // the closures are defined here
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

    // I tried to make a function, that would return this closure with different number passed in the chain_iter
    // but I didn't figure out how to make a function return a Fn closure which also takes arguments
    // the compiler really wanted me to make the return type FnOnce, but then I couldn't use the closure in a loop
    // Also I didn't figure out how to make a lifetime contract here, since apparently the issue was the uncertainty
    // that the mapped_value would live as long as the closure ://
    let print_inverse = |mapped_value: usize| {
        for (i, ch) in chain_iter(h_char.clone(), 5) {
            let should_print_char =
                (mapped_value % 2 == 0 && i % 2 == 0) || (mapped_value % 2 != 0 && i % 2 != 0);
            if should_print_char {
                print!("{}", ch);
            } else {
                print!(" ");
            }
        }
    };

    // this is where the stuff runs
    printin_time(h_char.len() * 5, h_char.len(), 25, print_chars)?;
    printin_time(28, 4, 80, print_hellos)?;
    printin_time(16, 2, 125, print_inverse)?;

    stdout().execute(Clear(CurrentLine))?;
    stdout().execute(cursor::MoveToColumn(0))?;
    println!("{h}");

    Ok(())
}
