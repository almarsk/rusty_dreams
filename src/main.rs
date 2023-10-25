use rust_course::*;
use std::env;
mod my_error;

// I think I preferred my approach from hw2
// I am following the assignment, so I broke up my hashmap of closures into functions
// but now there is a little more repetition and I am a bit confused about some things:
//              placement of eprinting the operation
//              multiple calls of env::args() (if main is only supposed to handle command arg)
//              Where exactly I was supposed to use the format! macro
//              I ended up not putting the ? operator after the modifying functions calls
//
// I am a little unhappy about the fact that some error get printed out inside an extra Err variant
//
// The error handling taught me new stuff though!
// I hope my build_error function isn't entirely mimo mísu
// I did it this way, because simply returning the Err(Box...) was of type Result<_, Err...>
// and the Ok variant inference wasn't getting along with the prescribed output

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let result = match args[1].as_str() {
            "lowercase" => to_lowercase(),
            "uppercase" => to_uppercase(),
            "slugify" => to_slugified(),
            "no-spaces" => no_spaces(),
            "ale-ironicky" => ale_ironicky(),
            "reverse" => reverse(),
            "csv" => csv(),
            _ => todo!("spin up concurrent interactive mode, no prompt"),
        };
        match result {
            Ok(i) => println!("{}", i),
            Err(e) => eprintln!("{:?}", e),
        }
    } else {
        todo!("spin up concurrent interactive mode, no prompt")
    }
}
