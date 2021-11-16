extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use anyhow::Result;
use std::process::Command;
use structopt::StructOpt;

/// flou-poc cmd option
#[derive(StructOpt, Debug)]
#[structopt(name = "flou-poc")]
pub struct Opt {
  /// Flou File Input
  #[structopt(short, long, default_value = "flow.flou")]
  pub flou: String,
  /// SVG File Output
  #[structopt(short, long, default_value = "flow.svg")]
  pub svg: String,
  /// CSS File to apply
  #[structopt(short, long, default_value = "nord.css")]
  pub css: String,
}

pub fn get_opt() -> Opt {
  return Opt::from_args();
}

fn main() -> Result<()> {
  better_panic::install();
  pretty_env_logger::init();

  let opt = get_opt();

  let output = Command::new("flou")
    .arg(opt.flou)
    .arg("-o")
    .arg(&opt.svg)
    .arg("--css")
    .arg(opt.css)
    .output()
    .expect("failed to execute process");

  info!("status: {}", output.status);

  Ok(())
}
