mod args;
mod cli;
mod db;
mod macros;
mod run;
pub use anyhow::Context;
pub use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
	run::run(args::Arguments::from_args_safe()?)?;
	Ok(())
}
