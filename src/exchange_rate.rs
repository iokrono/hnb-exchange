use serde::Deserialize;
use serde::de::{Deserializer, Visitor, Error};
use std::fmt;
use chrono::{Date, Utc, NaiveDate};

#[derive(Debug, Deserialize)]
pub struct ExchangeRate {
    #[serde(rename = "broj_tecajnice")]
    pub exchange_number: String,
    #[serde(rename = "datum_primjene", deserialize_with = "parse_date")]
    pub exchange_date: Date<Utc>,
    #[serde(rename = "drzava")]
    pub country: String,
    #[serde(rename = "drzava_iso")]
    pub country_iso: String,
    #[serde(rename = "sifra_valute")]
    pub currency_code: String,
    #[serde(rename = "valuta")]
    pub currency: String,
    #[serde(rename = "jedinica")]
    pub unit: u32,
    #[serde(rename = "kupovni_tecaj", deserialize_with = "parse_number")]
    pub buying_rate: f64,
    #[serde(rename = "srednji_tecaj", deserialize_with = "parse_number")]
    pub middle_rate: f64,
    #[serde(rename = "prodajni_tecaj", deserialize_with = "parse_number")]
    pub selling_rate: f64,
}

fn parse_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
{
    struct JsonStringVisitor;

    impl<'de> Visitor<'de> for JsonStringVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing float number with comma (,) as decimal separator")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
        {
            let formatted = v.replace(".", "").replace(",", ".");
            serde_json::from_str(&formatted).map_err(E::custom)
        }
    }

    deserializer.deserialize_any(JsonStringVisitor)
}

fn parse_date<'de, D>(deserializer: D) -> Result<Date<Utc>, D::Error>
    where
        D: Deserializer<'de>,
{
    const DATE_FORMAT: &'static str = "%Y-%m-%d";

    struct JsonStringVisitor;

    impl<'de> Visitor<'de> for JsonStringVisitor {
        type Value = Date<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing date in format %Y-%m-%d")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
        {
            NaiveDate::parse_from_str(&v, DATE_FORMAT)
                .map(|v| Date::from_utc(v, Utc))
                .map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_any(JsonStringVisitor)
}

#[cfg(test)]
mod tests {

    use chrono::{Utc, TimeZone};
    use crate::exchange_rate::ExchangeRate;

    #[test]
    fn distance_test() {
        let data = r#"
        {
            "broj_tecajnice": "161",
            "datum_primjene": "2020-08-21",
            "drzava": "Australija",
            "drzava_iso": "AUS",
            "sifra_valute": "036",
            "valuta": "AUD",
            "jedinica": 1,
            "kupovni_tecaj": "4,539891",
            "srednji_tecaj": "4,553552",
            "prodajni_tecaj": "4,567213"
        }
        "#;

        let er: ExchangeRate = serde_json::from_str(data).unwrap();

        assert_eq!(er.exchange_number, "161");
        assert_eq!(er.exchange_date, Utc.ymd(2020, 8, 21));
        assert_eq!(er.country, "Australija");
        assert_eq!(er.country_iso, "AUS");
        assert_eq!(er.currency_code, "036");
        assert_eq!(er.currency, "AUD");
        assert_eq!(er.unit, 1);
        assert_eq!(er.buying_rate, 4.539891);
        assert_eq!(er.middle_rate, 4.553552);
        assert_eq!(er.selling_rate, 4.567213);
    }
}