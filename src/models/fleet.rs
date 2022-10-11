use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Debug)]
pub enum FleetType {
    HS,
    LS,
    NS,
    COVOPS,
    TRAINING,
    GATECAMP,
    EVENT,
}

#[derive(Clone, Debug)]
pub struct Fleet {
   pub name: String,
   pub fc: String,
   pub formup: String,
   pub url: String,
   pub start: DateTime<Utc>,
   pub fleet_type: FleetType,
}
