use crate::args;
use crate::match_patterns;
use colored::*;
use std::io::Error;

pub fn action(input: Vec<&str>) -> anyhow::Result<()> {
	// this is a vec sanitizer, focusing on removing and formatting things provided by a custom macro to get the match arms (/ keys) in a match statement
	// the given vec has \" \" around the value we want so we remove that
	// we also filter out _ from the given vec, to make sure it doesn't display any not-needed info
	let actions = input
		.into_iter()
		.filter_map(|x| x.strip_prefix('"')?.strip_suffix('"'))
		.collect::<Vec<_>>();
	// log a nice message displaying all the actions / commands
	println!(
		"Actions available:\n{}",
		actions.join(", ").underline().bold().blue()
	);
	Ok(())
}

pub fn match_cmds(args: args::Arguments) -> anyhow::Result<()> {
	let cmd = &args.action;
	match_patterns! { &*cmd.to_lowercase(), patterns,
		"action" => action(patterns)?,
		"actions" => action(patterns)?,
		_ => return Err(anyhow::Error::new(Error::new(
			std::io::ErrorKind::InvalidInput,
			"Invalid action. Try the command `action`",
		)))
	};
	Ok(())
}
