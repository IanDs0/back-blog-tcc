use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum Level {
    Free = 0,
    Contributor = 1,
    Admin = 2,
}