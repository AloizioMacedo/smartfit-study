use crate::location::{Requirement, Schedule};
use askama::Template;

#[derive(Template)]
#[template(path = "result.html")]
pub struct ResultsTemplate {
    pub results: Vec<LocTemplate>,
}

pub struct LocTemplate {
    pub opened_status: &'static str,
    pub open_class: &'static str,
    pub title: String,
    pub address: String,
    pub prohibs: Vec<Prohib>,
    pub schedules: Vec<Schedule>,
}

pub struct Prohib {
    pub prohib_source: String,
    pub alt: String,
}

pub enum ProhibObj {
    Mask,
    Towel,
    Fountain,
    Locker,
}

impl ProhibObj {
    fn get_source_component(&self) -> &'static str {
        match self {
            ProhibObj::Mask => "mask",
            ProhibObj::Towel => "towel",
            ProhibObj::Fountain => "fountain",
            ProhibObj::Locker => "lockerroom",
        }
    }
}

impl Requirement {
    fn get_source_component(&self) -> &'static str {
        match self {
            Requirement::Required => "required",
            Requirement::Partial => "partial",
            Requirement::NotAllowed => "forbidden",
            Requirement::Recommended => "recommended",
            Requirement::Allowed => "required",
            Requirement::Closed => "forbidden",
        }
    }
}

pub fn get_source(prohib_obj: &ProhibObj, req: &Requirement) -> String {
    "images/".to_string()
        + req.get_source_component()
        + "-"
        + prohib_obj.get_source_component()
        + ".png"
}

pub fn get_alt(prohib_obj: &ProhibObj, req: &Requirement) -> String {
    req.get_source_component().to_string() + "-" + prohib_obj.get_source_component()
}
