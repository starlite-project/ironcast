use std::{
	env, io,
	path::PathBuf,
	process::{Command, ExitStatus},
};

const CHANNELS_TO_BUILD: [&str; 3] = ["stable", "beta", "nightly"];
const TOOLS_TO_BUILD: [&str; 3] = ["rustfmt", "clippy", "miri"];
const REPOSITORY: &str = "starlightpyro";

fn main() -> io::Result<()> {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=images");

	for channel in CHANNELS_TO_BUILD {
		build_channel(
			channel,
			env::var("PROFILE").map_or(false, |val| val == "release"),
		)?;
	}

	for tool in TOOLS_TO_BUILD {
		build_tool(tool)?;
	}

	Ok(())
}

fn build_channel(channel: &str, build_extra: bool) -> io::Result<()> {
	let image_name = format!("rust-{channel}", channel = channel);
	let full_name = format!(
		"{repository}/{image_name}",
		repository = REPOSITORY,
		image_name = image_name
	);

	let _ = Command::new("docker")
		.args(["pull", &full_name, "||", "true"])
		.status();

	if build_extra {
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

	let tag = Command::new("docker")
		.args(["tag", &full_name, &image_name])
		.spawn()?
		.wait()?;

	handle_exit_status(tag)?;

	Ok(())
}

fn build_tool(tool: &str) -> io::Result<()> {
	let full_name = format!("{repository}/{tool}", repository = REPOSITORY, tool = tool);

	let _ = Command::new("docker")
		.args(["pull", &full_name, "||", "true"])
		.status();

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
