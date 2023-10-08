use axum::{extract::Query, routing::get, Json, Router};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use smartfit::{
    location::{Data, Loc, Location},
    loctemplate::{get_alt, get_source, LocTemplate, Prohib, ProhibObj},
};

async fn get_locations() -> Json<Vec<Loc>> {
    let file = std::fs::read_to_string("locations.json").expect("JSON file should be accessible");

    let data: Data = serde_json::from_str(&file).unwrap();

    Json(data.locations)
}

#[derive(Serialize, Deserialize)]
struct QueryParams {
    day_period: DayPeriod,
    show_closed: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum DayPeriod {
    Morning,
    Afternoon,
    Evening,
}

fn get_results(Query(q): Query<QueryParams>) -> Vec<LocTemplate> {
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

    loctemplates
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
    }
}

trait QueryMatch {
    fn matches_query(&self, q: &QueryParams) -> bool;
}

impl QueryMatch for Location {
    fn matches_query(&self, q: &QueryParams) -> bool {
        if !q.show_closed && !self.opened {
            return false;
        }

        match q.day_period {
            DayPeriod::Morning => self.schedules.iter().any(|schedule| {
                parse_interval(&schedule.hour).0
                    <= NaiveTime::parse_from_str("12h00", "%Hh%M").unwrap()
                    && parse_interval(&schedule.hour).1
                        >= NaiveTime::parse_from_str("6h00", "%Hh%M").unwrap()
            }),
            DayPeriod::Afternoon => self.schedules.iter().any(|schedule| {
                parse_interval(&schedule.hour).0
                    <= NaiveTime::parse_from_str("18h00", "%Hh%M").unwrap()
                    && parse_interval(&schedule.hour).1
                        >= NaiveTime::parse_from_str("12h01", "%Hh%M").unwrap()
            }),
            DayPeriod::Evening => self.schedules.iter().any(|schedule| {
                parse_interval(&schedule.hour).0
                    <= NaiveTime::parse_from_str("23h00", "%Hh%M").unwrap()
                    && parse_interval(&schedule.hour).1
                        >= NaiveTime::parse_from_str("18h01", "%Hh%M").unwrap()
            }),
        }
    }
}

fn parse_interval(interval: &str) -> (NaiveTime, NaiveTime) {
    match interval.split(' ').collect::<Vec<&str>>()[..] {
        [x, _, y] => (
            NaiveTime::parse_from_str(x, "%Hh%M")
                .unwrap_or_else(|_| NaiveTime::parse_from_str(x, "%Hh").unwrap()),
            NaiveTime::parse_from_str(y, "%Hh%M")
                .unwrap_or_else(|_| NaiveTime::parse_from_str(x, "%Hh").unwrap()),
        ),
        _ => panic!(),
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/locations", get(get_locations));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_chrono() {
        let x = chrono::NaiveTime::parse_from_str("06h30", "%Hh%M").unwrap();
        let y = chrono::NaiveTime::parse_from_str("06h30", "%Hh%M").unwrap();
        let z = chrono::NaiveTime::parse_from_str("06h", "%Hh%M").unwrap();
        println!("{:?}", x >= y);
    }
}
