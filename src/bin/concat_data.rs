use argparse::{ArgumentParser, Store};
use log::{info, warn};
use std::error::Error;

const NUM_DASHES: usize = 50;

fn main() -> Result<(), Box<dyn Error>> {
    //! 데이터 합치기
    let exe = std::env::current_exe()?;
    let base = exe.parent().ok_or("Failed to get the base path.")?;
    let start = std::time::Instant::now();
    let mut imo = String::default();
    let mut date = String::default();
    let mut out = base.join("out");

    // argument 파싱
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

    unsafe {
        std::env::set_var("LOG_OUT", &out);
    }

    // init log4rs
    log4rs::init_file(base.join("config/log.yaml"), Default::default())?;

    info!("{dashes} THE PROGRAM STARTED {dashes}", dashes="-".repeat(NUM_DASHES));
    info!("");
    info!("👉 Check the Inputs");
    info!("IMO NUMBER: {imo}");
    info!("DATE: {date}");
    info!("BASE DIRECTORY: {}", base.display());
    info!("OUTPUT DIRECTORY: {}", out.display());


    println!("{}", std::env::var("LOG_OUT")?);

    info!("");
    info!("👀 Elapsed time: {:.2} (sec)", start.elapsed().as_secs_f32());
    info!("{dashes} THE PROGRAM ENDED. {dashes}", dashes="-".repeat(NUM_DASHES));
    Ok(())
}
