use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{Chit, UserId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub users: Vec<UserId>,
    pub year: u16,
    pub month: u8,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub chit: Vec<Chit>,
}
