use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct InvalidTypeError(Vec<String>);

impl InvalidTypeError {
	fn new(values: &[&str]) -> Self {
		Self(values.iter().map(ToString::to_string).collect())
	}
}

impl Display for InvalidTypeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("an invalid type was given, expected one of: ")?;

		for value in self.0.iter() {
			Display::fmt(value, f)?;
			f.write_str(", ")?;
		}

		f.write_str("")
	}
}

impl Error for InvalidTypeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RustChannel {
	Stable,
	Beta,
	Nightly,
}

impl Default for RustChannel {
	fn default() -> Self {
		Self::Nightly
	}
}

impl Display for RustChannel {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::Stable => "stable",
			Self::Beta => "beta",
			Self::Nightly => "nightly",
		})
	}
}

impl FromStr for RustChannel {
	type Err = InvalidTypeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"stable" => Ok(Self::Stable),
			"beta" => Ok(Self::Beta),
			"nightly" => Ok(Self::Nightly),
			_ => Err(InvalidTypeError::new(&["stable", "beta", "nightly"])),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Edition {
	#[serde(rename = "2015")]
	E2015,
	#[serde(rename = "2018")]
	E2018,
	#[serde(rename = "2021")]
	E2021,
}

impl Default for Edition {
	fn default() -> Self {
		Self::E2018
	}
}

impl Display for Edition {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::E2015 => "2015",
			Self::E2018 => "2018",
			Self::E2021 => "2021",
		})
	}
}

impl FromStr for Edition {
	type Err = InvalidTypeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"2015" => Ok(Self::E2015),
			"2018" => Ok(Self::E2018),
			"2021" => Ok(Self::E2021),
			_ => Err(InvalidTypeError::new(&["2015", "2018", "2021"])),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CrateType {
	#[serde(rename = "bin")]
	Binary,
	#[serde(rename = "lib")]
	Library,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildMode {
	Debug,
	Release,
}

impl Default for BuildMode {
	fn default() -> Self {
		Self::Debug
	}
}

impl Display for BuildMode {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::Debug => "debug",
			Self::Release => "release",
		})
	}
}

impl FromStr for BuildMode {
	type Err = InvalidTypeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"debug" => Ok(Self::Debug),
			"release" => Ok(Self::Release),
			_ => Err(InvalidTypeError::new(&["debug", "release"])),
		}
	}
}
