use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Specialization {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub desc: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Doctor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub short_intro: String,
    pub intro: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinic: Option<ObjectId>,
    pub specializations: Vec<String>,
    pub positions: Vec<String>,
    pub experiences: Vec<String>,
    pub awards: Vec<String>,
    pub educations: Vec<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clinic {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,    
    pub name: String,
    pub desc: String,
    pub hospital: Option<ObjectId>,
    pub specializations: Vec<ObjectId>,
    pub doctor: Vec<ObjectId>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hospital {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,    
    pub name: String,
    pub desc: String,
    pub location: String,
    pub province: ObjectId,
    pub clinic: Vec<ObjectId>,
    pub specializations: Vec<ObjectId>,
    pub doctor: Vec<ObjectId>,
    pub telephone: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Province {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shift {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]    
    pub doctor: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinic: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]    
    pub hospital: Option<ObjectId>,
    pub start_time: u32, // timestamp in second
    pub duration: u32, // time in second
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookingTicket {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub date_of_birth: u32, // timestamp in second
    pub email: String,
    pub phone_number: String,
    pub gender_is_male: bool,
    pub clinic: ObjectId,
    pub doctor: Option<ObjectId>,
    pub time: u32, // timestamp in second
    pub desc_symptoms: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginSession {
    #[serde(rename = "_id")]
    pub token: String,
    pub expired_time: u32,
    pub user: String,
}