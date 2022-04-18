use std::collections::HashMap;
use crate::repository::fleets_html::ParseError;

pub struct Fleet {
    name: String,
    fc: String,
    formup: String,
}

pub async fn get_fleets(server_loc: String) -> Result<Vec<Fleet>, ParseError> {
    let resp = match reqwest::get(format!("{}/api/events", server_loc)).await {
        Ok(b) => b,
        Err(_) => { return Result::Err(ParseError {}); }
    };

    let json = match resp.json::<Vec<HashMap<String, String>>>().await {
        Ok(b) => b,
        Err(_) => { return Result::Err(ParseError {}); }
    };

    let fleets = json.iter().map(|event_map| {
        return Fleet {
            name: event_map.get("name").unwrap().clone(),
            fc: event_map.get("FC").unwrap().clone(),
            formup: event_map.get("formup").unwrap().clone(),
        };
    }).collect();

    return Result::Ok(fleets);
}


#[cfg(test)]
mod tests {
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use crate::repository;


    #[async_test]
    async fn returns_fleets() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET)
                .path("/api/events");
            then.status(200)
                .body(r#"[{
    "doctrine_name": "GANKED Doctrine: TBA",
    "Description": "V2VsY29tZSB0byBHQU5LRUQsIFNwZWN0cmUgRmxlZXQncyBwcmVtaWVyIGV2ZW50IGZvciB0aGUgd2Vlay4NCldlIGZseSBldmVyeSBTYXR1cmRheSBuaWdodCwgbG9va2luZyBmb3IgdHJvdWJsZSBhbmQgZnVuIGluIE5ldyBFZGVuLg0KQW55IHF1ZXN0aW9ucz8gRmVlbCBmcmVlIHRvIGNvbnRhY3QgdXMgb24gRGlzY29yZCwgd2UncmUgaGVyZSB0byBoZWxwLg0KWW91IGFyZSBhbGwgd2VsY29tZSwgcGxlYXNlIGpvaW4gdXMg4p2k77iPDQoNCuKepCBJTkdBTUUgQ0hBTk5FTDogU0YgU3BlY3RyZSBGbGVldA0K4p6kIERJU0NPUkQgU0VSVkVSOiBodHRwczovL2Rpc2NvcmQuZ2cvRFBGZTlxdw0K4p6kIEZMRUVUIFNDSEVEVUxFOiBodHRwczovL3d3dy5zcGVjdHJlLWZsZWV0LnNwYWNlLw0K4p6kIEZMRUVUIERPQ1RSSU5FUzogaHR0cHM6Ly93d3cuc3BlY3RyZS1mbGVldC5zcGFjZS9kb2N0cmluZS8NCuKepCBNT1JFIElORk9STUFUSU9OOiBodHRwczovL2JpdC5seS9TcGVjdHJlRmxlZXQ=",
    "group": "Spectre Fleet",
    "start_datetime": "2022-12-24 20:00:00.000000",
    "name": "GANKED 582 [RESERVED]",
    "formup": "Jita",
    "FC": "The FC",
    "pk": "2904",
    "FC_id": "",
    "FTtype": "VNt",
    "user": "80"
  },
  {
    "FTtype": "VNt",
    "formup": "Jita",
    "start_datetime": "2022-12-31 20:00:00.000000",
    "Description": "V2VsY29tZSB0byBHQU5LRUQsIFNwZWN0cmUgRmxlZXQncyBwcmVtaWVyIGV2ZW50IGZvciB0aGUgd2Vlay4NCldlIGZseSBldmVyeSBTYXR1cmRheSBuaWdodCwgbG9va2luZyBmb3IgdHJvdWJsZSBhbmQgZnVuIGluIE5ldyBFZGVuLg0KQW55IHF1ZXN0aW9ucz8gRmVlbCBmcmVlIHRvIGNvbnRhY3QgdXMgb24gRGlzY29yZCwgd2UncmUgaGVyZSB0byBoZWxwLg0KWW91IGFyZSBhbGwgd2VsY29tZSwgcGxlYXNlIGpvaW4gdXMg4p2k77iPDQoNCuKepCBJTkdBTUUgQ0hBTk5FTDogU0YgU3BlY3RyZSBGbGVldA0K4p6kIERJU0NPUkQgU0VSVkVSOiBodHRwczovL2Rpc2NvcmQuZ2cvRFBGZTlxdw0K4p6kIEZMRUVUIFNDSEVEVUxFOiBodHRwczovL3d3dy5zcGVjdHJlLWZsZWV0LnNwYWNlLw0K4p6kIEZMRUVUIERPQ1RSSU5FUzogaHR0cHM6Ly93d3cuc3BlY3RyZS1mbGVldC5zcGFjZS9kb2N0cmluZS8NCuKepCBNT1JFIElORk9STUFUSU9OOiBodHRwczovL2JpdC5seS9TcGVjdHJlRmxlZXQ=",
    "FC_id": "",
    "FC": "",
    "pk": "2905",
    "doctrine_name": "GANKED Doctrine: TBA",
    "name": "GANKED 583 [RESERVED]",
    "group": "Spectre Fleet",
    "user": "80"
  }]"#);
        });

        let server_loc = server.url("");
        let result = repository::fleets_rest::get_fleets(server_loc).await;
        let fleets = result.unwrap();

        assert_eq!(fleets[0].name, "GANKED 582 [RESERVED]");
        assert_eq!(fleets[0].fc, "The FC");
        assert_eq!(fleets[0].formup, "Jita");
        assert_eq!(fleets[1].name, "GANKED 583 [RESERVED]");
        assert_eq!(fleets[1].fc, "");
        assert_eq!(fleets[1].formup, "Jita");

        //TODO fleet type
        //TODO start time
        //TODO Doctrine - missing from response!
    }
}