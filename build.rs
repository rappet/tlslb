include!("src/cli.rs");

use clap::CommandFactory;
use clap_complete::{generate_to, shells::Shell};

fn main() -> std::io::Result<()> {
    let cmd = Cli::command();

    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();

    std::fs::write(format!("man/{crate_name}.1"), buffer)?;

    for shell in [
        Shell::Bash,
        Shell::Elvish,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Zsh,
    ] {
        generate_to(shell, &mut cmd.clone(), &crate_name, "./completions")?;
    }

    Ok(())
}
