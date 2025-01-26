use clap::{command, ArgGroup, Command};

fn cli() -> Command {
  command!().group(ArgGroup::new("pack"))
}

fn main() {
  let matches = cli().get_matches();

  match matches.subcommand() {
    Some(("pack", _)) => {
      println!("pack");
    }
    _ => unreachable!(),
  }
}
