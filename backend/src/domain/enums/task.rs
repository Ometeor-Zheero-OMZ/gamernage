use std::str::FromStr;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Status::Active),
            "inactive" => Ok(Status::Inactive),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}
