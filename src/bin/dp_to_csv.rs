use argparse::{ArgumentParser, Store};
use csv::Writer;
use data_collect::parse_dp;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::default();
    let mut output = String::default();

    {
        // parse arguments
        let mut ap = ArgumentParser::new();
        ap.set_description("Export dp");
        ap.refer(&mut input)
            .required()
            .add_option(&["-f", "--file"], Store, "input file name");
        ap.refer(&mut output)
            .add_option(&["-o", "--out"], Store, "out file path");
        ap.parse_args_or_exit();
    }

    // output file name
    if output.is_empty() {
        output = std::path::Path::new(&input)
            .file_stem()
            .ok_or("err")?
            .to_str()
            .ok_or("err")?
            .to_string();
        output = format!("{output}.csv");
    }

    // parse dp and get data
    let data = parse_dp(&input)?;

    // export to file
    let mut writer = Writer::from_path(&output)?;

    for row in data {
        writer.serialize(row)?
    }

    println!("Input  : {input}");
    println!("Output : {output}");

    Ok(())
}
