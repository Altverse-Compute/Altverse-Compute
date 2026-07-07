use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  napi_build::setup();

  let schema_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/proto/flat");

  let schema_file = schema_dir.join("game.fbs");

  println!("cargo:rerun-if-changed={}", schema_dir.display());
  println!("cargo:rerun-if-changed={}", schema_file.display());

  let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/proto/gen/flat");

  let status = Command::new("flatc")
    .arg("--rust")
    .arg("-o")
    .arg(&out_dir)
    .arg(&schema_file)
    .status()?;

  if !status.success() {
    panic!("flatc failed");
  }
  Ok(())
}
