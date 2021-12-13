use std::{
	env, io,
	path::PathBuf,
	process::{Command, ExitStatus},
};

const CHANNELS_TO_BUILD: [&str; 3] = ["stable", "beta", "nightly"];
const TOOLS_TO_BUILD: [&str; 3] = ["rustfmt", "clippy", "miri"];
const REPOSITORY: &str = "starlightpyro";

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=images");

	for channel in CHANNELS_TO_BUILD {
		build_channel(channel).unwrap();
	}

	for tool in TOOLS_TO_BUILD {
		build_tool(tool).unwrap();
	}
}

fn build_channel(channel: &str) -> io::Result<()> {
	let image_name = format!("rust-{channel}", channel = channel);
	let full_name = format!(
		"{repository}/{image_name}",
		repository = REPOSITORY,
		image_name = image_name
	);

	let pull_statuses = vec![
		full_name.as_str(),
		format!("{full_name}:munge", full_name = full_name).as_str(),
		format!("{full_name}:sources", full_name = full_name).as_str(),
	]
	.into_iter()
	.map(pull_image)
	.collect::<Result<Vec<bool>, _>>()?;

	if pull_statuses.iter().all(|stat| *stat) {
		return tag_image(&full_name, &image_name);
	}

	for target in ["munge", "sources"] {
		let exit_status = Command::new("docker")
			.args([
				"build",
				"-t",
				&format!(
					"{full_name}:{target}",
					full_name = full_name,
					target = target
				),
			])
			.args(["--cache-from", &full_name])
			.args([
				"--cache-from",
				&format!(
					"{full_name}:{target}",
					full_name = full_name,
					target = target
				),
			])
			.args(["--target", target])
			.args([
				"--build-arg",
				&format!("channel={channel}", channel = channel),
			])
			.arg(
				PathBuf::from(env!("CARGO_MANIFEST_DIR"))
					.join("images")
					.join("base"),
			)
			.spawn()?
			.wait()?;

		handle_exit_status(exit_status)?;
	}

	let last_build = Command::new("docker")
		.args(["build", "-t", &full_name])
		.args([
			"--build-arg",
			&format!("channel={channel}", channel = channel),
		])
		.args(["--cache-from", &full_name])
		.arg(
			PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.join("images")
				.join("base"),
		)
		.spawn()?
		.wait()?;

	handle_exit_status(last_build)?;

	tag_image(&full_name, &image_name)?;

	Ok(())
}

fn build_tool(tool: &str) -> io::Result<()> {
	let full_name = format!("{repository}/{tool}", repository = REPOSITORY, tool = tool);

	let pulled_image = pull_image(&full_name)?;

	if pulled_image {
		return tag_image(&full_name, tool)
	}

	let build_status = Command::new("docker")
		.args(["build", "-t", &full_name])
		.arg(
			PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.join("images")
				.join(tool),
		)
		.spawn()?
		.wait()?;

	handle_exit_status(build_status)?;

	Ok(())
}

fn handle_exit_status(status: ExitStatus) -> io::Result<()> {
	if let Some(code) = status.code() {
		if code != 0 {
			return Err(io::Error::from_raw_os_error(code));
		}
	}

	Ok(())
}

fn tag_image(full_name: &str, tag: &str) -> io::Result<()> {
	let tag_status = Command::new("docker")
		.args(["tag", full_name, tag])
		.spawn()?
		.wait()?;

	handle_exit_status(tag_status)
}

fn pull_image(full_name: &str) -> io::Result<bool> {
	Ok(Command::new("docker")
		.args(["pull", full_name])
		.spawn()?
		.wait()?
		.success())
}
