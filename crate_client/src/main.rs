use clap::Parser;
use env_logger::Builder;

use std::io::Write;

// there is unconnected yew frontend in the works
mod backend;
use backend::send_and_receive;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = names::Generator::default().next().unwrap())]
    nick: String,
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,
    #[arg(long, default_value_t = String::from("11111"))]
    port: String,
}

impl Args {
    fn deconstruct(self) -> (String, String, String) {
        (self.host, self.port, self.nick)
    }
}

fn main() {
    let (host, port, nick) = Args::parse().deconstruct();

    // env_logger as backend for log here
    Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.args()
            )
        })
        .init();

    // terminal for now
    //yew::Renderer::<App>::new().render();

    // client loop
    send_and_receive(host, port, nick)
}
