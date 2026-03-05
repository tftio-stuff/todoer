use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    New,
    InProgress,
    Completed,
    Abandoned,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::New => "NEW",
            Status::InProgress => "IN-PROGRESS",
            Status::Completed => "COMPLETED",
            Status::Abandoned => "ABANDONED",
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NEW" => Ok(Status::New),
            "IN-PROGRESS" => Ok(Status::InProgress),
            "COMPLETED" => Ok(Status::Completed),
            "ABANDONED" => Ok(Status::Abandoned),
            _ => Err(format!("invalid status: {s}")),
        }
    }
}
