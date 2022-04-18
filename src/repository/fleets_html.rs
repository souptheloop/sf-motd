use std::fmt;
use std::fmt::{Formatter};

use chrono::{TimeZone, Utc};
use scraper;
use scraper::ElementRef;

use crate::models::fleet::{Fleet, FleetType};

#[derive(Debug, Clone)]
pub struct ParseError;
impl fmt::Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse")
    }
}

pub async fn get_fleets(server_loc: String) -> Result<Vec<Fleet>, ParseError> {
    let res = match reqwest::get(format!("{}", server_loc)).await {
        Ok(b) => b,
        Err(_) => { return Result::Err(ParseError{}) }
    };

    let bytes = match res.bytes().await {
        Ok(b) => b,
        Err(_) => { return Result::Err(ParseError{}) }
    };

    let html = match String::from_utf8(bytes.to_vec()) {
        Ok(s) => s,
        Err(_) => { return Result::Err(ParseError{}) }
    };

    let document = scraper::Html::parse_document(&html);

    let rows_selector = scraper::Selector::parse("table.calendar-table tbody tr").unwrap();
    let fleets = document.select(&rows_selector)
        .map(|row| {row_to_fleet(row)})
        .collect::<Vec<Fleet>>();

    return Result::Ok(fleets);
}

fn row_to_fleet(row: ElementRef) -> Fleet {
    let td_selector = scraper::Selector::parse("td").unwrap();
    let data = row.select(&td_selector).collect::<Vec<ElementRef>>();

    let anchor_selector = scraper::Selector::parse("a").unwrap();
    let doctrine = data[4].select(&anchor_selector).next().unwrap();
    let doctrine_name = doctrine.inner_html().trim().to_string();
    let doctrine_url = doctrine.value().attr("href").unwrap();

    let date = Utc.datetime_from_str(&data[6].inner_html(), "%B %d, %Y %H:%M").unwrap();

    let span_selector = scraper::Selector::parse("span").unwrap();
    let fleet_type_class = data[7].select(&span_selector).next().unwrap().value().attr("class").unwrap();

    let fleet_type: FleetType = match fleet_type_class {
        "dot-HS" => FleetType::HS,
        "dot-LS" => FleetType::LS,
        "dot-NS" => FleetType::NS,
        "dot-VNt" => FleetType::EVENT,
        "dot-COv" => FleetType::COVOPS,
        _ => FleetType::EVENT,
    };

    Fleet {
        name: doctrine_name,
        fc: data[2].inner_html(),
        formup: data[5].inner_html(),
        url: format!("https://www.spectre-fleet.space{}", doctrine_url),
        start: date,
        fleet_type,
    }
}


#[cfg(test)]
mod tests {
    use std::fs;

    use chrono::{TimeZone, Utc};
    use httpmock::Method::GET;
    use httpmock::MockServer;

    use crate::models::fleet::FleetType;
    use crate::repository;

    #[async_test]
    async fn returns_fleets() {
        let fixture = fs::read_to_string("src/repository/fixture.html").expect("could not read fixture.html");

        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/");
            then.status(200)
                .body(fixture);
        });

        let server_loc = server.url("");
        let result = repository::fleets_html::get_fleets(server_loc).await;
        let fleets = result.unwrap();

        assert_eq!(fleets[0].name, "BVL Flying Circus - Panzerliede!");
        assert_eq!(fleets[0].fc, "Larkness");
        assert_eq!(fleets[0].url, "https://www.spectre-fleet.space/d/PQ7w");
        assert_eq!(fleets[0].start, Utc.datetime_from_str("2022-04-18 12:00:00.000000", "%F %H:%M:%S.%f").unwrap());
        assert_eq!(fleets[0].fleet_type, FleetType::LS);

        assert_eq!(fleets[1].name, "Golden Hunters");
        assert_eq!(fleets[1].fc, "Arwen Estalia");
        assert_eq!(fleets[1].url, "https://www.spectre-fleet.space/d/AYIyg");
        assert_eq!(fleets[1].start, Utc.datetime_from_str("2022-04-18 18:00:00.000000", "%F %H:%M:%S.%f").unwrap());
        assert_eq!(fleets[1].fleet_type, FleetType::NS);

    }
}