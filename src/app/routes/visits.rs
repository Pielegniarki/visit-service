use std::{str::FromStr, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::bson::{doc, oid::ObjectId, serde_helpers::bson_datetime_as_rfc3339_string, DateTime};
use serde::Deserialize;

use futures::TryStreamExt;

use crate::{
    app::{token_extractor::ExtractId, AppState},
    db::schemas::Visit,
};

#[derive(Deserialize, Debug)]
pub struct ScheduleVisitParams {
    #[serde(rename(deserialize = "doctorId"))]
    pub doctor_id: ObjectId,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
}

pub async fn schedule_visit(
    ExtractId(patient_id): ExtractId,
    State(state): State<Arc<AppState>>,
    Json(params): Json<ScheduleVisitParams>,
) -> impl IntoResponse {
    let colls = state.db.collections();
    let visits = colls.visit();

    let visit = Visit {
        _id: ObjectId::new(),
        doctor_id: params.doctor_id,
        patient_id: *patient_id.as_patient().unwrap(),
        closed: false,
        date: params.date,
    };

    let Ok(_) = visits.insert_one(visit, None).await else {
        return StatusCode::NOT_FOUND
    };

    StatusCode::OK
}

pub async fn list_all_of_patient(
    ExtractId(id): ExtractId,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let colls = state.db.collections();
    let visits = colls.visit();

    let result = match visits
        .find(doc! { "patientId": id.as_patient().unwrap() }, None)
        .await
    {
        Ok(cursor) => cursor,
        Err(_) => return Json(vec![]),
    };

    let res = result.try_collect().await.unwrap();

    Json(res)
}

pub async fn list_all_of_doctor(
    ExtractId(id): ExtractId,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let colls = state.db.collections();
    let visits = colls.visit();

    let result = match visits
        .find(
            doc! { "doctorId": ObjectId::from_str(&id.as_doctor().unwrap()).unwrap() },
            None,
        )
        .await
    {
        Ok(cursor) => cursor,
        Err(_) => return Json(vec![]),
    };

    let Ok(visits) = result.try_collect().await else {
        return Json(vec![])
    };

    Json(visits)
}

#[derive(Deserialize, Debug)]
pub struct CloseVisitParams {
    #[serde(rename(deserialize = "visitId"))]
    pub visit_id: String,
}

pub async fn close_visit(
    State(state): State<Arc<AppState>>,
    Json(params): Json<CloseVisitParams>,
) -> impl IntoResponse {
    let visits = state.db.collections().visit();

    let Ok(x) = visits.update_one(
        doc!{ "_id": ObjectId::from_str(&params.visit_id).unwrap()  },
        doc!{ "$set": { "closed": true }},
        None
    ).await else {
        return "Err"
    };

    dbg!(x);

    "Ok"
}
