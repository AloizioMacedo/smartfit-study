use crate::location::Requirement;
use askama::Template;

#[derive(Template)]
#[template(path = "result.html")]
struct LocTemplate {
    open_class: &'static str,
    opened_status: &'static str,
    title: String,
    address: String,
    prohibs: Vec<Prohib>,
}

struct Prohib {
    prohib_source: String,
    alt: &'static str,
}

enum OpenStatus {
    Open,
    Closed,
}

impl OpenStatus {
    fn status(&self) -> &'static str {
        match self {
            OpenStatus::Open => "Aberto",
            OpenStatus::Closed => "Fechado",
        }
    }

    fn class(&self) -> &'static str {
        match self {
            OpenStatus::Open => "open-facility",
            OpenStatus::Closed => "closed-facility",
        }
    }
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
        }
    }
}

pub fn get_source(prohib_obj: &ProhibObj, req: &Requirement) -> String {
    req.get_source_component().to_string() + "-" + prohib_obj.get_source_component()
}
