use fluvio_smartstream::{smartstream, Record, RecordData};
use regex::Regex;

#[smartstream(map)]
pub fn map(record: &Record) -> (Option<RecordData>, RecordData) {
    let key = record.key.clone();

    let string_result = std::str::from_utf8(record.value.as_ref());
    let string = match string_result {
        Ok(s) => s,
        Err(_) => return (key, record.value.clone()),
    };

    let social_security_regex = Regex::new(r"\d{3}-\d{2}-\d{4}").unwrap();
    let output = social_security_regex
        .replace_all(string, "***-**-****")
        .to_string();

    (key, output.into())
}
