mod args_resolver;

use termion::{color, style};

use crate::exchange_client::get_rates;
use crate::args_resolver::{resolve_args, DATE_FORMAT};

mod exchange_client;
pub mod exchange_rate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (currency, start_date, end_date) = resolve_args();

    println!("Getting rates for {} from {} to {}",
             currency, start_date, end_date);

    let rates = get_rates(&currency, &start_date, &end_date).await?;

    for rate in rates.iter() {
        println!(" {exchange_number} {date} {green}{currency:^5}{reset} {units:^4} {yellow}{middle:.7}{reset}",
                 date = rate.exchange_date.format(DATE_FORMAT),
                 currency = format!(" {} ", rate.currency),
                 middle = rate.middle_rate,
                 exchange_number = rate.exchange_number,
                 units = rate.unit,
                 green = color::Fg(color::Green),
                 yellow = color::Fg(color::Yellow),
                 reset = style::Reset
        );
    }

    Ok(())
}
