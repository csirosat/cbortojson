use std::iter::Iterator;

fn main() {
	let mut d = serde_cbor::Deserializer::from_reader(std::io::stdin())
		.into_iter::<serde_cbor::Value>();

	while let Some(Ok(v)) = d.next() {
		println!("{}\n", serde_json::to_string_pretty(&v).unwrap());
	}
}
