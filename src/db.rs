use dirs_next::home_dir;
use std::fs;
use std::process::{Command, Output};

pub fn clone() -> anyhow::Result<Output> {
	Ok(Command::new("git")
		.current_dir(home_dir().unwrap().join(".manny"))
		.arg("clone")
		.arg("https://github.com/Milo123459/mannydb")
		.arg("db")
		.output()?)
}

pub fn update() -> anyhow::Result<Output> {
	Ok(Command::new("git")
		.current_dir(home_dir().unwrap().join(".manny").join("db"))
		.arg("pull")
		.output()?)
}

pub fn should_clone() -> anyhow::Result<bool> {
	let mut folder_path = home_dir().unwrap();
	folder_path.push(".manny");
	fs::create_dir_all(folder_path)?;
	let should = !home_dir().unwrap().join(".manny").join("db").exists();

	Ok(should)
}
