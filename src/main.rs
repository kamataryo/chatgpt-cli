use std::io::Write;
use clap::Parser;
use tokio::runtime::Runtime;
use whoami;
use ansi_term::Colour::{Green, Cyan};

use std::thread;
use std::time;
use tokio::sync::oneshot;

mod credential;
use crate::credential::parse_credentials;

mod ask;
use crate::ask::ask;

#[derive(Parser, Debug)]
struct Args {
  question: String,
}

fn main() {

  let credential = parse_credentials();

  let args = Args::parse();
  let question = args.question.as_str();

  println!("{} >> {}", Green.paint(whoami::username()), question);

  let (stop_signal_tx, stop_signal_rx) = oneshot::channel();
  let handle = thread::spawn(move || show_loading(stop_signal_rx));
  let future = ask(question, credential.clone());
  let rt = Runtime::new().unwrap();
  let result = rt.block_on(future);
  stop_signal_tx.send(()).unwrap();

  handle.join().unwrap();

  match result {
    Ok(answer) => println!("{} << {}", Cyan.paint(answer.role), answer.content),
    Err(err) => {},
  }
}

fn show_loading(mut stop_signal_rx: tokio::sync::oneshot::Receiver<()>) {
  let mut i = 0;
  loop {
      i = (i + 1) % 4;

      let message = match i {
          0 => "   ",
          1 => ".  ",
          2 => ".. ",
          _ => "...",
      };

      print!("\r{}", message);
      std::io::stdout().flush().unwrap();
      thread::sleep(time::Duration::from_millis(300));

      match stop_signal_rx.try_recv() {
        Ok(_) => {
          print!("\r{}", "\x08\x08\x08");
          break;
        },
        Err(_err) => (),
      }
  }
}



