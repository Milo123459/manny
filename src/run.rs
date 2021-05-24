use crate::args;
use crate::cli;

pub fn run(args: args::Arguments) -> anyhow::Result<()> {
	cli::match_cmds(args)?;
	Ok(())
}
