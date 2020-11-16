use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(json) = cbortojson::cbor_reader_to_json_string(std::io::stdin()) {
        stdout().write_all(json.as_bytes())?;
    }
    Ok(())
}
