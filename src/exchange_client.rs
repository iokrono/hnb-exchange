use crate::exchange_rate::ExchangeRate;
use reqwest::{Error, Client};

const URL: &str = "http://api.hnb.hr/tecajn/v2";

pub async fn get_rates(currency: &str, start_date: &str, end_date: &str)
    -> Result<Vec<ExchangeRate>, Error> {

    let params = [
        ("valuta", currency),
        ("datum-primjene-od", start_date),
        ("datum-primjene-do", end_date)
    ];


    let response = Client::new().get(URL)
        .query(&params)
        .send().await?;

    let url = response.url();
    println!("URL: {}", url);

    response
        .json::<Vec<ExchangeRate>>().await
}