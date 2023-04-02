use std::io::{self, Write, Read};
use tokio::runtime::Runtime;
use whoami;
use ansi_term::Colour::{Green, Cyan};
use std::process;
use std::thread;
use std::time;
use tokio::sync::oneshot;

mod credential;
use crate::credential::parse_credentials;

mod ask;
use crate::ask::ask;

fn main() {

  let mut question = String::new();
  io::stdin().read_to_string(&mut question).unwrap();

  if question.is_empty() {
    eprintln!("No questions. Process will exit.");
    process::exit(0);
  }
  question = question.trim_end_matches('\n').to_string();

  eprintln!("{} >> {}", Green.paint(whoami::username()), question);

  let (stop_signal_tx, stop_signal_rx) = oneshot::channel();
  let handle = thread::spawn(move || show_loading(stop_signal_rx));
  let future = ask(&question, parse_credentials().clone());
  let rt = Runtime::new().unwrap();
  let result = rt.block_on(future);
  stop_signal_tx.send(()).unwrap();

  handle.join().unwrap();

  match result {
    Ok(answer) => {
      eprint!("{} << ", Cyan.paint(answer.role));
      println!("{}", answer.content);
    },
    Err(err) => {
      eprintln!("{}", err);
      process::exit(1);
    },
  }

  process::exit(0);
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

      eprint!("\r{}", message);
      std::io::stdout().flush().unwrap();
      thread::sleep(time::Duration::from_millis(300));

      match stop_signal_rx.try_recv() {
        Ok(_) => {
          eprint!("\r{}", "\x08\x08\x08");
          break;
        },
        Err(_err) => (),
      }
  }
}



