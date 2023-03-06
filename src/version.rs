//! Conan executable version and API revision functions

use std::path::Path;
use std::process::Command;

use crate::find_program;

/// Conan executable version
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConanVersion {
    /// Major version component
    pub major: u8,
    /// Minor version component
    pub minor: u8,
    /// Patch version component
    pub patch: u8,
}

/// Conan interface (API) revision
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConanApi {
    /// Conan 1.x API
    #[default]
    V1,
    /// Conan 2.x API
    V2,
}

impl ConanVersion {
    /// Discovers the `"conan"` executable version
    pub fn detect() -> Option<Self> {
        Self::query_from(&find_program()?)
    }

    /// Queries the provided Conan executable version
    pub fn query_from(conan_program: &Path) -> Option<Self> {
        let output = Command::new(conan_program).arg("--version").output().ok()?;

        // $ conan --version
        // Conan version 1.14.3

        let output_stdout = String::from_utf8(output.stdout).ok()?;
        let version_str = output_stdout.as_str().trim();

        let captures = lazy_regex!(r"version (\d+)\.(\d+)\.(\d+)$").captures(version_str)?;
        let major: u8 = captures[1].parse().ok()?;
        let minor: u8 = captures[2].parse().ok()?;
        let patch: u8 = captures[3].parse().ok()?;

        Some(ConanVersion { major, minor, patch })
    }

    /// Infers the Conan API revision from the version
    pub fn api(self) -> Option<ConanApi> {
        match self.major {
            1 => Some(ConanApi::V1),
            2 => Some(ConanApi::V2),
            _ => None,
        }
    }
}
