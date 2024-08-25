// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use clap::{ArgGroup, Parser};

use crate::command::Command;
use modlib;

#[derive(Parser, Clone, PartialEq, Eq, Debug)]
pub struct App {
    #[arg(hide = true, value_parser = clap::builder::PossibleValuesParser::new(["modules"]))]
    pub dummy: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

impl App {
    pub fn sanitized_command(self) -> Command {
        let mut command = self.command;
        command.sanitize();
        command
    }
}

#[derive(Parser, Clone, PartialEq, Eq, Debug)]
#[group(id = "ProjectOptions")]
#[command(group = ArgGroup::new("target-group"))]
pub struct ProjectOptions {
    /// Process only this package's library.
    #[arg(long = "lib", group = "target-group")]
    pub lib: bool,

    /// Process only the specified binary.
    #[arg(long = "bin", group = "target-group")]
    pub bin: Option<String>,

    /// Package to process (see `cargo help pkgid`).
    #[arg(short = 'p', long = "package")]
    pub package: Option<String>,

    /// Do not activate the `default` feature.
    #[arg(long = "no-default-features")]
    pub no_default_features: bool,

    /// Activate all available features.
    #[arg(long = "all-features")]
    pub all_features: bool,

    /// List of features to activate.
    /// This will be ignored if `--cargo-all-features` is provided.
    #[arg(long = "features")]
    pub features: Vec<String>,

    /// Analyze for target triple.
    #[arg(long = "target")]
    pub target: Option<String>,

    /// Path to Cargo.toml.
    #[arg(long = "manifest-path", default_value = ".")]
    pub manifest_path: PathBuf,
}

impl Into<modlib::options::ProjectOptions> for ProjectOptions {
    fn into(self) -> modlib::options::ProjectOptions {
        modlib::options::ProjectOptions {
            lib: self.lib,
            bin: self.bin,
            package: self.package,
            no_default_features: self.no_default_features,
            all_features: self.all_features,
            features: self.features,
            target: self.target,
            manifest_path: self.manifest_path,
        }
    }
}

#[derive(Parser, Clone, PartialEq, Eq, Debug)]
#[group(id = "GeneralOptions")]
pub struct GeneralOptions {
    /// Use verbose output.
    #[arg(long = "verbose")]
    pub verbose: bool,
}

impl Into<modlib::options::GeneralOptions> for GeneralOptions {
    fn into(self) -> modlib::options::GeneralOptions {
        modlib::options::GeneralOptions {
            verbose: self.verbose,
        }
    }
}
