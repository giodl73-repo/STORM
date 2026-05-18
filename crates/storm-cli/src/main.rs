use std::{env, error::Error, fs, io};
use storm_core::{has_errors, validate_document, StormDocument};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    match args.as_slice() {
        [_, command, path] if command == "validate" => {
            let text = fs::read_to_string(path)?;
            let document = serde_json::from_str::<StormDocument>(&text)?;
            let findings = validate_document(&document);
            println!("{}", serde_json::to_string_pretty(&findings)?);
            if has_errors(&findings) {
                std::process::exit(1);
            }
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "usage: storm validate <storm-document.json>",
            )
            .into());
        }
    }
    Ok(())
}
