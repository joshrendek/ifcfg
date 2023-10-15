use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ASLookup {
    pub status: String,
    #[serde(rename = "status_message")]
    pub status_message: String,
    pub data: Data,
    #[serde(rename = "@meta")]
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub asn: i64,
    pub name: String,
    #[serde(rename = "description_short")]
    pub description_short: String,
    #[serde(rename = "description_full")]
    pub description_full: Vec<String>,
    #[serde(rename = "country_code")]
    pub country_code: String,
    pub website: String,
    #[serde(rename = "email_contacts")]
    pub email_contacts: Vec<String>,
    #[serde(rename = "abuse_contacts")]
    pub abuse_contacts: Vec<String>,
    #[serde(rename = "traffic_estimation")]
    pub traffic_estimation: String,
    #[serde(rename = "traffic_ratio")]
    pub traffic_ratio: String,
    #[serde(rename = "owner_address")]
    pub owner_address: Vec<String>,
    #[serde(rename = "rir_allocation")]
    pub rir_allocation: RirAllocation,
    #[serde(rename = "iana_assignment")]
    pub iana_assignment: IanaAssignment,
    #[serde(rename = "date_updated")]
    pub date_updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RirAllocation {
    #[serde(rename = "rir_name")]
    pub rir_name: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    #[serde(rename = "date_allocated")]
    pub date_allocated: String,
    #[serde(rename = "allocation_status")]
    pub allocation_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IanaAssignment {
    #[serde(rename = "assignment_status")]
    pub assignment_status: String,
    pub description: String,
    #[serde(rename = "whois_server")]
    pub whois_server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(rename = "api_version")]
    pub api_version: i64,
    #[serde(rename = "execution_time")]
    pub execution_time: String,
}
