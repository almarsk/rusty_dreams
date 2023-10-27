use ascii_table::AsciiTable;
use csv::Reader;
use std::error::Error;

pub fn parse_into_ascii_table(path: String) -> Result<String, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;

    let mut data: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        data.push(record.iter().map(|s| s.to_string()).collect());
    }

    let mut ascii_table = AsciiTable::default();

    rdr.headers()?.iter().enumerate().for_each(|(i, h)| {
        ascii_table.column(i).set_header(h);
    });

    if data.is_empty() {
        let mut dummy_vec = vec![];
        for _ in 0..rdr.headers()?.len() {
            dummy_vec.push(String::from(""))
        }
        data.push(dummy_vec)
    }

    let output = ascii_table.format(data);

    Ok(output)
}
