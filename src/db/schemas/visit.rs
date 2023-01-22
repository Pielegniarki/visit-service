use mongodb::bson::{DateTime, serde_helpers::bson_datetime_as_rfc3339_string, oid::ObjectId};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Visit {
    pub _id: ObjectId,
    #[serde(rename = "doctorId")]
    pub doctor_id: ObjectId,
    #[serde(rename = "patientId")]
    pub patient_id: u32,
    pub closed: bool,
    #[serde(with="bson_datetime_as_rfc3339_string")]
    pub date: DateTime
}