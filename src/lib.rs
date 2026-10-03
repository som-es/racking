#![deny(
    bad_style,
    missing_debug_implementations,
    overflowing_literals,
    patterns_in_fns_without_body,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

pub mod source_tracker;
pub mod steps;
mod tracked_owned;
mod tracked_ref;
pub mod trail;
pub use tracked_owned::*;
pub use tracked_ref::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BuildInfo {
    pub git_sha: &'static str,
    pub dirty: bool,
    pub src_hash: &'static str,
    pub crate_version: &'static str,
}

pub const BUILD: BuildInfo = BuildInfo {
    git_sha: env!("GIT_SHA"),
    dirty: matches!(env!("GIT_DIRTY").as_bytes(), b"true"),
    src_hash: env!("SRC_HASH"),
    crate_version: env!("CARGO_PKG_VERSION"),
};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::Utc;
    use serde_json::json;

    use crate::{
        source_tracker::SourceTracker,
        steps::{HttpMethod, HttpSource, Step},
    };

    #[test]
    fn test_track_source() {
        let mut source_tracker = SourceTracker::default();

        let body = json! {{
          "NRBR": [
            "NR"
          ],
          "GP_CODE": ["XXVII"],
          "VHG": [
            "BNR"
          ],
          "VHG2": [
            "BNR"
          ],
          "DOKTYP": [
            "BNR"
          ]
        }};

        source_tracker.add_source(Step::Http(HttpSource {
            url: "".to_string(),
            method: HttpMethod::Post,
            body: Some(body),
            headers: HashMap::default(),
            fetched_at: Utc::now(),
        }));

        let var = source_tracker.var(json! {{
            "hi": {
                "idx": "data"
            }
        }});
        let hi = var.get("hi").get("idx");
        let str = hi.into_string().transpose().unwrap();
        assert_eq!(&str.data, "data");
        assert_eq!(str.trail().step(), &Step::Str);

        source_tracker.add_source(Step::Http(HttpSource {
            url: "".to_string(),
            method: HttpMethod::Get,
            body: None,
            headers: HashMap::default(),
            fetched_at: Utc::now(),
        }));
    }

    #[test]
    fn track_array_and_str() {
        let mut tracker = SourceTracker::default();
        tracker.add_source(Step::Http(HttpSource {
            url: "https://api.example/legislation".to_string(),
            method: HttpMethod::Get,
            body: None,
            headers: HashMap::default(),
            fetched_at: Utc::now(),
        }));

        let content = tracker.var(json! {{
            "reference": [{ "url": "https://init.example/1" }],
            "title": "Some Law"
        }});

        let legis_init_path = content
            .get("reference")
            .get_index(0)
            .get("url")
            .as_str()
            .data
            .unwrap();
        assert_eq!(legis_init_path, "https://init.example/1");

        let trail = content
            .get("reference")
            .get_index(0)
            .get("url")
            .trail()
            .to_string();
        assert!(trail.contains("http GET https://api.example/legislation"));
        assert!(trail.contains(r#"key "reference""#));
        assert!(trail.contains("index 0"));
        assert!(trail.contains(r#"key "url""#));

        let items = content.get("reference").items().unwrap();
        assert_eq!(items.len(), 1);
        assert!(items[0].trail().to_string().contains("index 0"));
        assert_eq!(content.get("reference").as_array().unwrap().len(), 1);

        let values = content.object_values().unwrap();
        assert_eq!(values.len(), 2);
        // assert!(
        //     values.iter().any(|v| v.as_str() == Some("Some Law")
        //         && v.trail().to_string().contains(r#"key "title""#))
        // );
    }

    #[test]
    #[should_panic(expected = "expected array, found 3")]
    fn expect_array_reports_the_trail() {
        // let mut tracker = SourceTracker::default();
        // let content = tracker.var(json! {{ "reference": 3 }});
        // let items = content.get("reference").expect_array();
        // println!("{:?}", items);
    }

    #[test]
    fn track_custom_transform() {
        let mut tracker = SourceTracker::default();
        let content = tracker.var(json! {{ "slug": "  MiNiStRy  " }});

        let slug = content.get("slug").transform("trim_lowercase_ascii", |v| {
            v.as_str().unwrap_or_default().trim().to_ascii_lowercase()
        });

        assert_eq!(slug.as_tracked().inner(), "ministry");
        let trail = slug.trail().to_string();
        assert!(trail.contains(r#"key "slug""#));
        assert!(trail.contains(r#"transform "trim_lowercase_ascii""#));

        let tagged = content.get("slug").labeled("held_for_review");
        assert_eq!(tagged.inner().as_str().unwrap(), "  MiNiStRy  ");
        assert!(
            tagged
                .trail()
                .to_string()
                .contains(r#"transform "held_for_review""#)
        );

        let failed = content
            .get("slug")
            .transform_opt("needs_number", |v| v.as_u64());
        assert!(failed.is_none());
    }

    #[test]
    #[should_panic(expected = "expected string, found 42")]
    fn expect_str_reports_the_trail() {
        let mut tracker = SourceTracker::default();
        tracker.add_source(Step::Http(HttpSource {
            url: "https://api.example/legislation".to_string(),
            method: HttpMethod::Get,
            body: None,
            headers: HashMap::default(),
            fetched_at: Utc::now(),
        }));
        let content = tracker.var(json! {{ "reference": [{ "url": 42 }] }});
        let url = content.get("reference").get_index(0).get("url");
        println!("{url:?}");
    }
}
