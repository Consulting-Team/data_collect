use argparse::{ArgumentParser, Store};
use chrono::NaiveDate;
use dotenv::dotenv;
use std::{env, error::Error, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    pub base: PathBuf,
    pub out: PathBuf,
    pub data_dir: PathBuf,
    pub imo: String,
    pub date: NaiveDate,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let exe = env::current_exe()?;
        let base = exe
            .parent()
            .ok_or("Failed to get the base path.")?
            .to_path_buf();
        let mut imo = String::default();
        let mut date = String::default();
        let mut out = base.join("out");

        dotenv().ok();

        // parse the arguments
        {
            let mut ap = ArgumentParser::new();
            ap.set_description("Read data files and concat it");
            ap.refer(&mut imo)
                .required()
                .add_option(&["--imo"], Store, "hull number");
            ap.refer(&mut date)
                .required()
                .add_option(&["-d", "--date"], Store, "date");
            ap.refer(&mut out)
                .add_option(&["-o", "--out"], Store, "output directory");
            ap.parse_args_or_exit();
        }

        // set IMO number
        imo = imo.replace("IMO", "");

        // parse date to NaiveDate
        let date = NaiveDate::parse_from_str(&date, "%Y%m%d")?;

        unsafe {
            env::set_var("LOG_OUT", &out);
        }

        // init log4rs
        // log4rs::init_file(base.join(env::var("LOG_CONFIG")?), Default::default())?;
        log4rs::init_file(base.join(env::var("LOG_CONFIG")?), Default::default())
            .map_err(|e| format!("Failed to get the log config path.{e}"))?;

        // set data path
        let data_dir = PathBuf::from(format!(
            "{}/IMO{}/{}/{}/{}",
            env::var("DATA_SOURCE")?,
            imo,
            date.format("%Y"),
            date.format("%m"),
            date.format("%d")
        ));

        Ok(Config {
            base,
            out,
            imo,
            date,
            data_dir,
        })
    }
}
