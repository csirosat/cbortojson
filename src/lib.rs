use std::iter::Iterator;

pub fn cbor_reader_to_json_string<R>(reader: R) -> Option<String>
where
    R: std::io::Read,
{
    let mut d = serde_cbor::Deserializer::from_reader(reader).into_iter::<serde_cbor::Value>();

    while let Some(Ok(v)) = d.next() {
        return Some(serde_json::to_string_pretty(&v).unwrap());
    }
    None
}
