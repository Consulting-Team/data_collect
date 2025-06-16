mod config;

use config::Config;
use log::{error, info};
use polars::{io::SerReader, prelude::CsvReader};
use std::{
    error::Error,
    fs::File,
    io::{Cursor, Read},
    path::PathBuf,
    process::{Command, Stdio},
};

const NUM_DASHES: usize = 10;

fn main() -> Result<(), Box<dyn Error>> {
    //! 데이터 합치기
    let start = std::time::Instant::now();
    let config = Config::new()?;

    info!(
        "{dashes} THE PROGRAM STARTED {dashes}",
        dashes = "-".repeat(NUM_DASHES)
    );

    if !config.data_dir.exists() {
        error!(
            "Not existing data path: \"{}\"\n",
            config.data_dir.display()
        );
        std::process::exit(1);
    }

    // print inputs
    print_inputs(&config)?;

    // get data files corresponding to input date
    let paths = get_zip_list(&config)?;

    //
    read_zips(&paths)?;

    info!(
        "👀 Elapsed time: {:.2} (sec)\n",
        start.elapsed().as_secs_f32()
    );

    Ok(())
}

fn print_inputs(config: &Config) -> Result<(), Box<dyn Error>> {
    info!("👉 Show Inputs");
    info!("- IMO NUMBER: {}", config.imo);
    info!("- DATE: {}", config.date);
    info!("- BASE DIRECTORY: {}", config.base.display());
    info!("- OUTPUT DIRECTORY: {}", config.out.display());
    info!("- DATA DIRECTORY: {}", config.data_dir.display());
    info!("");

    Ok(())
}

fn get_zip_list(config: &Config) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    //! Returns the list of files in data directory.
    info!("👉 Get the list of data files");

    let data_dir = &config.data_dir;

    let ls = Command::new("ls")
        .arg(data_dir)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or("Failed to excute ls cmd.")?;

    let grep = Command::new("grep")
        // .arg(date.format("%Y%m%d").to_string())
        .arg("Zip")
        .stdin(Stdio::from(ls))
        .output()?;

    let out = String::from_utf8(grep.stdout)?;
    let paths = out.lines().map(|f| data_dir.join(f)).collect::<Vec<_>>();

    // print file list
    // for (i, path) in paths.iter().enumerate() {
    //     let fname = path
    //         .file_name()
    //         .ok_or("Failed to get the file name.")?
    //         .to_str()
    //         .ok_or("Failed to parse &OsSt to &str")?;
    //     info!("[{i}] {}", fname);
    // }

    info!("- The number of zip files: {}", paths.len());
    info!("");

    Ok(paths)
}

fn read_zips(paths: &[PathBuf]) -> Result<(), Box<dyn Error>> {
    info!("👉 Read the zip files");

    let mut archive;

    for path in paths {
        archive = zip::ZipArchive::new(File::open(path)?)?;

        // for each csv file
        for i in 0..archive.len() {
            let mut csv = archive.by_index(i)?;
            let mut buf = Vec::new();

            if !csv.name().contains(".csv") {
                continue;
            }

            csv.read_to_end(&mut buf)?;

            let cursor = Cursor::new(buf);
            let df = CsvReader::new(cursor).finish()?;

            let col_name = df
                .column_iter()
                .map(|c| c.name().as_str())
                .collect::<Vec<_>>();

            info!("- n_columns: {}  csv: {}", col_name.len(), csv.name());
        }
    }

    Ok(())
}
