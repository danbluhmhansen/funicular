use std::{error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=index.ts");
    println!("cargo:rerun-if-changed=uno.config.ts");
    println!("cargo:rerun-if-changed=assets");
    Command::new("sh").arg("-c").arg("bun install").output()?;
    Command::new("sh").arg("-c").arg("bun run bun").output()?;
    Command::new("sh").arg("-c").arg("bun run bld").output()?;
    Ok(())
}
