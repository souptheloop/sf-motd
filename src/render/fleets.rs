use chrono::format::{DelayedFormat, StrftimeItems};
use crate::models::fleet::{Fleet, FleetType};

struct Colors;

impl Colors {
    const HS: &'static str = "#ff00ff00";
    const LS: &'static str = "#ffffff00";
    const NS: &'static str = "#ffff0000";
    const COVOPS: &'static str = "#ffb2b2b2";
    const EVENT: &'static str = "#ff00ffff";
}

const NUM_FLEETS: usize = 3;

pub fn render_motd(fleets: &Vec<Fleet>) -> String {
    let mut sorted = fleets.clone();
    sorted.sort_by(|a, b| a.start.cmp(&b.start));


    let fleet_strings: Vec<String> = sorted
        .iter()
        .enumerate()
        .filter(|(i, _)| *i < NUM_FLEETS)
        .map(|(_, fleet)| render_fleet(fleet))
        .collect();

    return fleet_strings.join("<br/>");
}

fn render_fleet(fleet: &Fleet) -> String {
    let time = fleet.start.format("%d/%m %H:%M");
    let color = match fleet.fleet_type {
        FleetType::HS => Colors::HS,
        FleetType::LS => Colors::LS,
        FleetType::NS => Colors::NS,
        FleetType::COVOPS => Colors:: COVOPS,
        FleetType::EVENT => Colors::EVENT,
    };
    return format_fleet(color, time, fleet.url.as_str(), fleet.name.as_str(), fleet.fc.as_str(), fleet.formup.as_str());
}

fn format_fleet(color: &str, time: DelayedFormat<StrftimeItems>, url: &str, name: &str, fc: &str, location: &str) -> String {
    format!(r###"<font size="12" color="{color}"><b>{time} <a href="{url}">{name}</a> w/{fc} @{location}</b></font>"###, color = color, time = time, url = url, name = name, fc = fc, location = location)
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use rocket::form::validate::Contains;
    use crate::models::fleet::{Fleet, FleetType};

    use crate::render;

    #[test]
    fn renders_first_three_in_date_order() {
        let fleets = &vec![
            fleet("fleet 1", "FC 1", "Jita", "2022-04-11 18:00:00.000000", "", FleetType::HS),
            fleet("fleet 2", "FC 2", "Jita", "2022-04-11 19:00:00.000000", "", FleetType::HS),
            fleet("fleet 3", "FC 3", "Jita", "2022-05-20 18:00:00.000000", "", FleetType::HS),
            fleet("fleet 4", "FC 4", "Jita", "2022-04-14 18:00:00.000000", "", FleetType::HS)];

        let result = render::fleets::render_motd(fleets);
        assert_eq!(true, result.contains("fleet 1"));
        assert_eq!(true, result.contains("fleet 2"));
        assert_eq!(true, result.contains("fleet 4"));
    }

    #[test]
    fn renders_when_single_fleet() {
        let fleets = &vec![
            fleet("fleet 1", "FC 1", "Jita", "2022-04-11 18:00:00.000000", "", FleetType::HS)];

        let result = render::fleets::render_motd(fleets);

        assert_eq!(true, result.contains("fleet 1"));
    }

    #[test]
    fn renders_when_no_fleet() {
        let fleets = &vec![];

        let result = render::fleets::render_motd(fleets);

        assert_eq!(result, "");
    }

    #[test]
    fn renders_hs_html() {
        let result = render::fleets::render_motd(&vec![
            fleet("A Fleet", "A FC", "Jita", "2022-01-02 18:00:00.000000", "http://www.spectre-fleet.space", FleetType::HS)]);

        assert_eq!(result, r###"<font size="12" color="#ff00ff00"><b>02/01 18:00 <a href="http://www.spectre-fleet.space">A Fleet</a> w/A FC @Jita</b></font>"###);
    }

    #[test]
    fn renders_ls_html() {
        let result = render::fleets::render_motd(&vec![
            fleet("A Fleet", "A FC", "Jita", "2022-01-02 18:00:00.000000", "http://www.spectre-fleet.space", FleetType::LS)]);

        assert_eq!(result, r###"<font size="12" color="#ffffff00"><b>02/01 18:00 <a href="http://www.spectre-fleet.space">A Fleet</a> w/A FC @Jita</b></font>"###);
    }

    fn fleet(name: &str, fc: &str, form_up: &str, start: &str, url: &str, fleet_type: FleetType) -> Fleet {
        render::fleets::Fleet {
            name: name.to_string(),
            fc: fc.to_string(),
            formup: form_up.to_string(),
            url: url.to_string(),
            start: Utc.datetime_from_str(start, "%F %H:%M:%S.%f").unwrap(),
            fleet_type: fleet_type,
        }
    }
}