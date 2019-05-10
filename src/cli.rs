use git_version::git_version;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    rename_all = "kebab-case",
    raw(
        version = "&*Box::leak(format!(\"{}-{}\", crate_version!(), git_version!()).into_boxed_str())"
    )
)]
pub struct Opts {
    /// Names of the modules to build
    #[structopt(long)]
    pub module_names: Vec<String>,
    /// Directories to search for module source code
    #[structopt(long)]
    pub search_dirs: Vec<PathBuf>,
    /// Directory to build in
    #[structopt(long, default_value = "build")]
    pub build_dir: PathBuf,
    /// Top-level CakeML function to run when the program executes
    #[structopt(long, default_value = "main")]
    pub cakeml_entry_point: String,
    /// Name to use for the top-level function symbol in the generated assembly
    #[structopt(long, default_value = "main")]
    pub asm_entry_point: String,
    /// Path to a checkout of the CakeML compiler repository
    #[structopt(long)]
    pub cakeml_dir: PathBuf,
    /// HOL4 dependency directories required by the build
    ///
    /// Will be built using Holmake
    #[structopt(long)]
    pub hol_includes: Vec<PathBuf>,
    /// Path to Holmake binary
    ///
    /// Will use the `Holmake` from $PATH if none is provided
    #[structopt(long, default_value = "Holmake")]
    pub holmake: PathBuf,
    /// Path to CakeML compiler binary
    ///
    /// Will use the `cake` from $PATH if none is provided
    #[structopt(long, default_value = "cake")]
    pub cakeml_bin: PathBuf,
}