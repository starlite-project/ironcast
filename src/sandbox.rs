use std::{error::Error, fmt::{Display, Formatter, Result as FmtResult}, io, path::PathBuf, string::FromUtf8Error, time::Duration};

use serde::{Deserialize, Serialize};
use tempdir::TempDir;
use tokio::time::error::Elapsed;

#[derive(Debug)]
pub enum SandboxError {
	UnableToCreateTempDir(io::Error),
	UnableToCreateOutputDir(io::Error),
	UnableToSetOutputPermissions(io::Error),
	UnableToCreateSourceFile(io::Error),
	UnableToSetSourcePermissions(io::Error),
	UnableToStartCompiler(io::Error),
	MissingCompilerId,
	UnableToWaitForCompiler(io::Error),
	UnableToGetOutputFromCompiler(io::Error),
	UnableToRemoveCompiler(io::Error),
	CompilerExecutionTimeout(Elapsed, Duration),
	UnableToReadOutput(io::Error),
	UnableToParseCrateInformation(serde_json::Error),
	OutputNotUtf8(FromUtf8Error),
	OutputMissing,
	VersionReleaseMissing,
	VersionHashMissing,
	VersionDateMissing,
}

impl Display for SandboxError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::UnableToCreateTempDir(e) => {
				f.write_str("unable to create temporary directory: ")?;
				Display::fmt(&e, f)
			}
			Self::UnableToCreateOutputDir(e) => {
				f.write_str("unable to create output directory: ")?;
				Display::fmt(&e, f)
			}
			Self::UnableToSetOutputPermissions(e) => {
				f.write_str("unable to set permissions for output directory: ")?;
				Display::fmt(&e, f)
			}
			Self::UnableToCreateSourceFile(e) => {
				f.write_str("unable to create the source file: ")?;
				Display::fmt(&e, f)
			}
			Self::UnableToSetSourcePermissions(e) => {
				f.write_str("unable to set source permissions: ")?;
				Display::fmt(&e, f)
			}
			Self::UnableToStartCompiler(e) => {
				f.write_str("unable to start compiler: ")?;
				Display::fmt(&e, f)
			}
			Self::MissingCompilerId => f.write_str("unable to find compiler ID"),
			Self::UnableToWaitForCompiler(e) => {
				f.write_str("unable to wait for the compiler: ")?;
				Display::fmt(&e, f)
			}
			Self::UnableToGetOutputFromCompiler(e) => {
				f.write_str("unable to get output from the compiler")?;
				Display::fmt(&e, f)
			}
			Self::UnableToRemoveCompiler(e) => {
				f.write_str("unable to remove the compiler: ")?;
				Display::fmt(&e, f)
			}
			Self::CompilerExecutionTimeout(_, timeout) => {
				f.write_str("compiler execution took longer than ")?;
				Display::fmt(&timeout.as_millis(), f)?;
				f.write_str(" ms")
			}
			Self::UnableToReadOutput(e) => {
				f.write_str("unable to read output file: ")?;
				Display::fmt(&e, f)
			}
			Self::UnableToParseCrateInformation(e) => {
				f.write_str("unable to parse crate information: ")?;
				Display::fmt(&e, f)
			}
			Self::OutputNotUtf8(e) => {
				f.write_str("output was not valid utf-8: ")?;
				Display::fmt(&e, f)
			}
			Self::OutputMissing => f.write_str("output was missing"),
			Self::VersionReleaseMissing => {
				f.write_str("release was missing from the version output")
			}
			Self::VersionHashMissing => {
				f.write_str("commit hash was missing from the version output")
			}
			Self::VersionDateMissing => {
				f.write_str("commit date was missing from the version output")
			}
		}
	}
}

impl Error for SandboxError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::UnableToCreateTempDir(e)
			| Self::UnableToCreateOutputDir(e)
			| Self::UnableToSetOutputPermissions(e)
			| Self::UnableToCreateSourceFile(e)
			| Self::UnableToSetSourcePermissions(e)
			| Self::UnableToStartCompiler(e)
			| Self::UnableToGetOutputFromCompiler(e)
			| Self::UnableToRemoveCompiler(e)
			| Self::UnableToReadOutput(e) => Some(e),
			Self::CompilerExecutionTimeout(e, _) => Some(e),
			Self::OutputNotUtf8(e) => Some(e),
			_ => None,
		}
	}
}

impl From<FromUtf8Error> for SandboxError {
	fn from(e: FromUtf8Error) -> Self {
		Self::OutputNotUtf8(e)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateInformation {
	pub name: String,
	pub version: String,
	pub id: String,
}

#[derive(Debug, Clone)]
pub struct Version {
	pub release: String,
	pub commit_hash: String,
	pub commit_date: String,
}

pub struct Sandbox {
	scratch: TempDir,
	input_path: PathBuf,
	output_path: PathBuf,
}

fn vec_to_str(v: Vec<u8>) -> Result<String, SandboxError> {
	Ok(String::from_utf8(v)?)
}
