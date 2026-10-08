use std::collections::HashMap;

use chrono::{NaiveDateTime, Utc};
use racking::{
    TrackedOwned,
    source_tracker::SourceTracker,
    steps::{HttpMethod, HttpSource, Step},
};
use serde_json::Value;

pub struct Proposal {
    pub id: TrackedOwned<String>,
    pub ityp: TrackedOwned<String>,
    pub gp: TrackedOwned<String>,
    pub inr: TrackedOwned<u64>,
    /*pub title: String,
    pub proposal_type: ProposalType,
    pub description: Option<String>,
    pub issuer_ids: Vec<u64>,
    pub receiver_ids: Vec<u64>,
    pub documents: Vec<Document>,
    pub topics: Vec<String>,
    pub eurovoc_topics: Vec<String>,
    pub other_keyword_topics: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    */
}

fn conv_json(raw_proposal: Value) -> Option<Proposal> {
    let mut tracker = SourceTracker::default();
    tracker.add_source(Step::Http(HttpSource {
        url: "https://www.parlament.gv.at/Filter/api/filter/data/101?js=eval&showAll=true"
            .to_string(),
        method: HttpMethod::Post,
        body: None,
        headers: HashMap::default(),
        fetched_at: Utc::now(),
    }));

    tracker.add_source(Step::Http(HttpSource {
        url: "https://www.parlament.gv.at/gegenstand/XXVII/UEA/568?json=true".to_string(),
        method: HttpMethod::Get,
        body: None,
        headers: HashMap::default(),
        fetched_at: Utc::now(),
    }));

    let raw_proposal = tracker.var(raw_proposal);
    let raw_proposal = raw_proposal.as_tracked();
    let content = raw_proposal.get("content");
    let ityp = content.get("ityp").into_string().transpose()?;
    // let inr = content["inr"].as_u64()?;
    //
    let updated_at = content
        .get("update")
        .map(|val| {
            let val = val.as_str();

            val.map(|updated_at| {
                NaiveDateTime::parse_from_str(updated_at, "%Y-%m-%dT%H:%M:%S").unwrap()
            })
        })
        .transpose()?;

    dbg!(updated_at);

    let updated_at = content.get("update").as_str().transpose()?;
    let updated_at = updated_at.as_tracked();
    let updated_at = updated_at
        .map(|updated_at| NaiveDateTime::parse_from_str(updated_at, "%Y-%m-%dT%H:%M:%S").unwrap());

    dbg!(updated_at);
    // .map(convert_vienna_datetime_to_utc)?;

    None
}

#[tokio::test]
async fn test_simple_proposal_conv_from_parlament_gv_at() {
    let data: Value =
        reqwest::get("https://www.parlament.gv.at/gegenstand/XXVII/UEA/568?json=true")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
    conv_json(data);
}
