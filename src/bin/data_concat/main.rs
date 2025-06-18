mod config;

use chrono::NaiveDateTime;
use config::Config;
use data_collect::parse_dp;
use log::{error, info, warn};
use polars::{frame::DataFrame, io::SerReader, prelude::CsvReader};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{Cursor, Read},
    path::PathBuf,
    process::{Command, Stdio},
};

const NUM_DASHES: usize = 10;

type DPList = HashMap<String, HashMap<String, String>>;
type DFList = HashMap<String, DataFrame>;

fn main() -> Result<(), Box<dyn Error>> {
    //! 데이터 합치기
    let start = std::time::Instant::now();

    // set a new configuration
    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            error!("{e}");
            return Err("Failed to get config.")?;
        }
    };

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
    check_inputs(&config);

    // read dp files
    let dp_list: DPList = match read_dps(&config) {
        Ok(hashmap) => hashmap,
        Err(e) => {
            error!("{e}");
            return Err("Failed to read the dp files.")?;
        }
    };

    // read zip files
    let dfs: DFList = match read_zips(&config) {
        Ok(paths) => paths,
        Err(e) => {
            error!("{e}");
            return Err("Failed to read the csv files.")?;
        }
    };

    info!(
        "👀 Elapsed time: {:.2} (sec)\n",
        start.elapsed().as_secs_f32()
    );

    Ok(())
}

fn rename_columns(dp_list: DPList, df_list: DFList) -> Result<(), Box<dyn Error>> {

    Ok(())
}

fn check_inputs(config: &Config) {
    info!("👉 Show Inputs");
    info!("- IMO NUMBER: {}", config.imo);
    info!("- DATE: {}", config.date);
    info!("- BASE DIRECTORY: {}", config.base.display());
    info!("- OUTPUT DIRECTORY: {}", config.out.display());
    info!("- DATA DIRECTORY: {}", config.data_dir.display());
    info!("");
}

fn get_zip_list(config: &Config) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    //! Returns the list of files in data directory.

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

    Ok(paths)
}

fn read_dps(config: &Config) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn Error>> {
    //! Reads the DP files corresponding to the IMO number from the $DP_PATH.

    info!("👉 Read the DP files.");

    let imo = &config.imo;
    let dp_dir = &config.dp_dir;
    let mut dp_list: DPList = HashMap::new();

    let ls = Command::new("ls")
        .arg(dp_dir)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or("Failed to excute ls cmd.")?;

    let grep = Command::new("grep")
        .arg(imo)
        .stdin(Stdio::from(ls))
        .output()?;

    let out = String::from_utf8(grep.stdout)?;
    let fnames = out.lines().collect::<Vec<_>>();

    // dp_list = Vec::with_capacity(fnames.len());

    for fname in fnames {
        let path = dp_dir.join(fname);
        let dp = parse_dp(&path.to_string_lossy())?;
        let hashmap: HashMap<String, String> = HashMap::from_iter(
            dp.into_iter().map(|row| (row.LocalID, row.OriginTag))
        );

        dp_list.insert(fname.to_string(), hashmap);

        info!("- {fname}");
    }

    info!("");
    Ok(dp_list)
}

fn read_zips(config: &Config) -> Result<HashMap<String, DataFrame>, Box<dyn Error>> {
    //! Reads zip files to DataFrame.
    //! paths : list of path to zip file
    // let paths = get_zip_list(config)?;

    info!("👉 Read the zip files");

    let mut archive;
    let mut dfs: DFList = HashMap::new();

    for path in get_zip_list(config)? {
        archive = zip::ZipArchive::new(File::open(&path)?)?;

        // for each csv file
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let mut buf = Vec::new();
            let file_path = PathBuf::from(file.name());

            let extension = file_path
                .extension()
                .map(|ext| ext.to_string_lossy().to_lowercase())
                .filter(|ext| ext.eq("csv"));

            if extension.is_none() {
                continue;
            }

            let fname = file_path
                .file_stem()
                .ok_or("Failed to get the file stem.")?
                .to_string_lossy()
                .to_string();

            let datetime = fname
                .split("_")
                .find_map(|token| NaiveDateTime::parse_from_str(token, "%Y%m%d%H%M").ok())
                //? 날짜로 파싱 가능하면 break 필요하지 않은지? --> 파일이 많을 경우 효율 증가
                .ok_or(format!(
                    "Failed to extract to datetime from file name: {}",
                    file.name()
                ))?;

            // read csv file
            file.read_to_end(&mut buf)?;
            let cursor = Cursor::new(buf);

            match CsvReader::new(cursor).finish() {
                Ok(df) => {
                    info!("- {}  column count: {}", datetime, df.column_iter().count());
                    dfs.insert(datetime.to_string(), df);
                }
                Err(e) => {
                    warn!("{e} Failed to read the csv file of \"{}\"", path.display());
                    continue;
                }
            }
        }
    }

    info!("");
    Ok(dfs)
}
