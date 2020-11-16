use std::iter::Iterator;

pub fn cbor_reader_to_json_string<R>(reader: R) -> Option<String>
where
    R: std::io::Read,
{
    serde_iter_result(serde_cbor::Deserializer::from_reader(reader))
}

pub fn cbor_bytes_to_json_string(slice: &[u8]) -> Option<String> {
    serde_iter_result(serde_cbor::Deserializer::from_slice(slice))
}

fn serde_iter_result<'a, R>(inp: serde_cbor::Deserializer<R>) -> Option<String>
where
    R: serde_cbor::de::Read<'a>,
{
    let mut d = inp.into_iter::<serde_cbor::Value>();
    while let Some(Ok(v)) = d.next() {
        return Some(serde_json::to_string_pretty(&v).unwrap());
    }
    None
}
