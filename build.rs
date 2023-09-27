use clap::Command;
use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell;
use std::env;
use std::error::Error;

include!("src/cli.rs");

fn generate_completion(
    mut cmd: &mut Command,
    shell: Shell,
    bin_name: &str,
    out_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let path = generate_to(shell, &mut cmd, bin_name, out_dir)?;
    println!("completion file for '{:?}' is generated: {:?}", shell, path);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=cli.rs");

    // get cargo out dir

    let out_dir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(out_dir) => PathBuf::from(out_dir).join("completions"),
    };

    if !out_dir.exists() {
        std::fs::create_dir(&out_dir)?;
    }

    let mut cmd = Cli::command();

    let shells = [
        Shell::Zsh,
        Shell::Bash,
        Shell::Elvish,
        Shell::Fish,
        Shell::PowerShell,
    ];

    for shell in shells.iter() {
        generate_completion(&mut cmd, *shell, "veil", &out_dir)?;
    }

    Ok(())
}
