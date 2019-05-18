use std::process::exit;
use structopt::StructOpt;
use chrono::prelude::{NaiveDateTime, DateTime};
use chrono::{Utc};

#[derive(StructOpt)]
#[structopt(name = "repoch")]
/// Convert the epoch value to datetime and datetime to epoch (UTC)
struct Opt {
    /// Value to convert
    value: String,
    /// If will convert from date to epoch or not
    #[structopt(short = "d", long = "date")]
    is_date: bool,
    /// Conversion format, Epoch is output and Date is input
    #[structopt(short = "f", long = "format", default_value = "%Y-%m-%d %H:%M:%S")]
    format: String
}

fn convert_epoch(value: String, format: String) -> Result<String, String> {
    let value_convert = match value.parse::<i64>() {
        Ok(value)  => value,
        Err(_) => return Err("The epoch cannot be converted to datetime".to_string()),
    };
    let naive_date = NaiveDateTime::from_timestamp(value_convert, 0);
    let date_tz: DateTime<Utc> = DateTime::from_utc(naive_date, Utc);
    let date_str = date_tz.format(&format).to_string();
    Ok(date_str)
}

fn convert_datetime(value: String, format: String) -> Result<String, String> {
    let naive_date = match NaiveDateTime::parse_from_str(&value, &format) {
        Ok(value)  => value,
        Err(_) => return Err("The datetime cannot be converted to epoch".to_string()),
    };

    let date_tz: DateTime<Utc> = DateTime::from_utc(naive_date, Utc);

    let dt_timestamp: i64 = date_tz.timestamp();

    Ok(dt_timestamp.to_string())
}

fn main() {
    let opt = Opt::from_args();

    let result_obj: Result<String, String>;

    // local: DateTime<Local> = Local::now();

    if opt.is_date {
        result_obj = convert_datetime(opt.value, opt.format);
        
    } else {
        result_obj = convert_epoch(opt.value, opt.format);
    }

    let result = match result_obj {
        Ok(value)  => value,
        Err(err) => {
            println!("{}", err);
            exit(1);
        },
    };

    
    println!{"{}", result};
}
