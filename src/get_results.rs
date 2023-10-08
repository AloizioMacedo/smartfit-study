use askama::Template;
use axum::{extract::Query, response::Html};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::{
    location::{Data, Loc, Location},
    loctemplate::{get_alt, get_source, LocTemplate, Prohib, ProhibObj, ResultsTemplate},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParams {
    day_period: DayPeriod,

    #[serde(default)]
    show_closed: Switch,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
enum Switch {
    On,

    #[default]
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum DayPeriod {
    Morning,
    Afternoon,
    Evening,
}

pub async fn get_results(Query(q): Query<QueryParams>) -> Html<String> {
    let file = std::fs::read_to_string("locations.json").expect("JSON file should be accessible");

    let data: Data = serde_json::from_str(&file).unwrap();

    let mut loctemplates = vec![];

    for location in data.locations {
        match location {
            Loc::Location(loc) => {
                if loc.matches_query(&q) {
                    loctemplates.push(parse_location(loc))
                }
            }
            _ => continue,
        }
    }

    let results_template = ResultsTemplate {
        results: loctemplates,
    };

    let rendered = results_template.render().unwrap();

    Html(rendered)
}

fn parse_location(location: Location) -> LocTemplate {
    let (open_class, opened_status) = if location.opened {
        ("open-facility", "Aberto")
    } else {
        ("closed-facility", "Fechado")
    };

    let mut prohibs = vec![];

    let mask = location.mask;
    let source = get_source(&ProhibObj::Mask, &mask);
    let alt = get_alt(&ProhibObj::Mask, &mask);
    prohibs.push(Prohib {
        prohib_source: source,
        alt,
    });

    let towel = location.towel;
    let source = get_source(&ProhibObj::Towel, &towel);
    let alt = get_alt(&ProhibObj::Towel, &towel);
    prohibs.push(Prohib {
        prohib_source: source,
        alt,
    });

    let fountain = location.fountain;
    let source = get_source(&ProhibObj::Fountain, &fountain);
    let alt = get_alt(&ProhibObj::Fountain, &fountain);
    prohibs.push(Prohib {
        prohib_source: source,
        alt,
    });

    let locker_room = location.locker_room;
    let source = get_source(&ProhibObj::Locker, &locker_room);
    let alt = get_alt(&ProhibObj::Locker, &locker_room);
    prohibs.push(Prohib {
        prohib_source: source,
        alt,
    });

    LocTemplate {
        open_class,
        opened_status,
        title: location.title,
        address: location.content,
        prohibs,
        schedules: location.schedules,
    }
}

trait QueryMatch {
    fn matches_query(&self, q: &QueryParams) -> bool;
}

impl QueryMatch for Location {
    fn matches_query(&self, q: &QueryParams) -> bool {
        if matches!(q.show_closed, Switch::Off) && !self.opened {
            return false;
        }

        match q.day_period {
            DayPeriod::Morning => self.schedules.iter().any(|schedule| {
                let parsed = parse_interval(&schedule.hour);
                if let Some(parsed) = parsed {
                    parsed.0 <= NaiveTime::parse_from_str("12h00", "%Hh%M").unwrap()
                        || parsed.1 >= NaiveTime::parse_from_str("6h00", "%Hh%M").unwrap()
                } else {
                    false
                }
            }),
            DayPeriod::Afternoon => self.schedules.iter().any(|schedule| {
                let parsed = parse_interval(&schedule.hour);

                if let Some(parsed) = parsed {
                    parsed.0 <= NaiveTime::parse_from_str("18h00", "%Hh%M").unwrap()
                        || parsed.1 >= NaiveTime::parse_from_str("12h01", "%Hh%M").unwrap()
                } else {
                    false
                }
            }),
            DayPeriod::Evening => self.schedules.iter().any(|schedule| {
                let parsed = parse_interval(&schedule.hour);

                if let Some(parsed) = parsed {
                    parsed.0 <= NaiveTime::parse_from_str("23h00", "%Hh%M").unwrap()
                        || parsed.1 >= NaiveTime::parse_from_str("18h01", "%Hh%M").unwrap()
                } else {
                    false
                }
            }),
        }
    }
}

fn parse_interval(interval: &str) -> Option<(NaiveTime, NaiveTime)> {
    match interval.split(' ').collect::<Vec<&str>>()[..] {
        [x, _, y] => Some((
            NaiveTime::parse_from_str(x, "%Hh%M").unwrap_or_else(|_| {
                NaiveTime::from_hms_opt(x[0..x.len() - 1].parse().unwrap(), 0, 0).unwrap()
            }),
            NaiveTime::parse_from_str(y, "%Hh%M").unwrap_or_else(|_| {
                NaiveTime::from_hms_opt(x[0..x.len() - 1].parse().unwrap(), 0, 0).unwrap()
            }),
        )),
        _ => None,
    }
}
