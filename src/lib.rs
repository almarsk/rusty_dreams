use crossterm::terminal::{Clear, ClearType::CurrentLine};
use crossterm::{cursor, ExecutableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

pub fn printin_time<T>(
    loops: usize,
    threshold: usize,
    frame_delay: u64,
    to_print: T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Fn(usize),
{
    for frame in 0..loops {
        let mapped_value = map_value(frame, threshold);

        stdout().execute(Clear(CurrentLine))?;
        stdout().execute(cursor::MoveToColumn(0))?;
        to_print(mapped_value);

        stdout().flush()?;
        sleep(Duration::from_millis(frame_delay));
    }

    Ok(())
}

pub fn chain_iter(series: Vec<char>, how_many: usize) -> impl Iterator<Item = (usize, char)> {
    let space = ' ';
    let series: Vec<char> = (0..how_many - 1).fold(series.clone(), |acc, _| {
        acc.into_iter()
            .chain(std::iter::once(space))
            .chain(series.clone())
            .collect()
    });
    series.into_iter().enumerate()
}

fn map_value(input_value: usize, threshold: usize) -> usize {
    let sector = input_value / threshold;
    if sector % 2 == 0 {
        input_value - threshold * sector
    } else {
        (threshold * (sector + 1)) - input_value
    }
}
