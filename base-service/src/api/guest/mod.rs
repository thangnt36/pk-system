use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use actix_web::{web, HttpResponse};
use mongodb::bson::oid::ObjectId;

use crate::models::*;
use crate::db;
use crate::api::{do_response, form::{SearchQuery, RelateSearchQuery}};


pub async fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (web::Query(query): web::Query<SearchQuery>) -> HttpResponse
{
    do_response(db::search::<T>(query)).await
}

pub async fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>    
    (item: web::Json<ObjectId>) -> HttpResponse
{
    let data = item.into_inner();
    do_response(db::get::<T, ObjectId>(data)).await
}

pub async fn relate
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<RelateSearchQuery>) -> HttpResponse
{
    let query = item.into_inner();
    do_response(db::search_relate::<T>(query)).await
}

pub async fn new_booking_ticket
    (item: web::Json<BookingTicket>) -> HttpResponse
{
    let data = item.into_inner();
    do_response(db::create::<BookingTicket>(&data)).await    
}
