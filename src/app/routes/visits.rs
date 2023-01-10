use std::sync::Arc;

use axum::{extract::{State, Query}, response::IntoResponse, Json, http::StatusCode};
use mongodb::bson::{doc, oid::ObjectId, DateTime, serde_helpers::bson_datetime_as_rfc3339_string};
use serde::Deserialize;

use futures::TryStreamExt;

use crate::{app::AppState, db::schemas::Visit};

#[derive(Deserialize, Debug)]
pub struct ScheduleVisitParams { 
    #[serde(rename(deserialize = "patientId"))]
    pub patient_id: u32,
    #[serde(rename(deserialize = "doctorId"))]
    pub doctor_id: String,
    #[serde(with="bson_datetime_as_rfc3339_string")]
    pub date: DateTime
}

impl<'a> Into<Visit> for ScheduleVisitParams {
    fn into(self) -> Visit {
        Visit { 
            _id: ObjectId::new(), 
            doctor_id: self.doctor_id, 
            patient_id: self.patient_id, 
            date: self.date 
        }
    }
}

pub async fn schedule_visit(
    State(state): State<Arc<AppState>>,
    Json(param): Json<ScheduleVisitParams>,
) -> impl IntoResponse {
    let colls = state.db.collections();
    let visits = colls.visit();

    let visit: Visit = param.into();

    let Ok(_) = visits.insert_one(visit, None).await else {
        return StatusCode::NOT_FOUND
    };

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct ListAllOfPatientParams {
    pub id: u32
}

pub async fn list_all_of_patient(
    Query(params): Query<ListAllOfPatientParams>,
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let colls = state.db.collections();
    let visits = colls.visit();

    
    dbg!("XD1");

    let result = match visits.find(doc! { "patientId": params.id }, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Json(vec![])
    };

    let res = result.try_collect().await.unwrap();

    Json(res)
}


#[derive(Deserialize)]
pub struct ListAllOfDoctorParams {
    pub id: String
}

pub async fn list_all_of_doctor(
    Query(params): Query<ListAllOfDoctorParams>,
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let colls = state.db.collections();
    let visits = colls.visit();

    let result = match visits.find(doc! { "doctorId": params.id }, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Json(vec![])
    };

    let Ok(visits) = result.try_collect().await else {
        return Json(vec![])
    };


    Json(visits)
}