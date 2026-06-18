use vazi_ranker::rank;

#[test]
fn ranks_by_engagement() {
    let candidates = r#"[
        {"outfit_id":"a","style_tier":"casual","avg_sti":80,"total_price":3000,"item_count":3,"freshness_hours":10,"impressions":100,"clicks":20,"purchases":2,"color_harmony":0.8,"price_coherence":0.9,"has_video":false},
        {"outfit_id":"b","style_tier":"casual","avg_sti":70,"total_price":2500,"item_count":2,"freshness_hours":10,"impressions":100,"clicks":5,"purchases":0,"color_harmony":0.7,"price_coherence":0.8,"has_video":false}
    ]"#;
    let ctx = r#"{"user_id":null,"county":null,"recent_clicks":[],"recent_purchases":[],"session_hour":12,"is_new_user":false}"#;
    let weights = "{}";

    let result = rank(candidates, ctx, weights);
    let scored: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();

    assert_eq!(scored[0]["outfit_id"], "a");
    assert_eq!(scored[1]["outfit_id"], "b");
}

#[test]
fn video_boost_works() {
    let candidates = r#"[
        {"outfit_id":"no-video","style_tier":"casual","avg_sti":80,"total_price":3000,"item_count":3,"freshness_hours":10,"impressions":50,"clicks":10,"purchases":1,"color_harmony":0.8,"price_coherence":0.9,"has_video":false},
        {"outfit_id":"has-video","style_tier":"casual","avg_sti":80,"total_price":3000,"item_count":3,"freshness_hours":10,"impressions":50,"clicks":10,"purchases":1,"color_harmony":0.8,"price_coherence":0.9,"has_video":true}
    ]"#;
    let ctx = r#"{"user_id":null,"county":null,"recent_clicks":[],"recent_purchases":[],"session_hour":12,"is_new_user":false}"#;
    let weights = "{}";

    let result = rank(candidates, ctx, weights);
    let scored: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();

    assert_eq!(scored[0]["outfit_id"], "has-video");
}

#[test]
fn cold_start_boost_works() {
    let candidates = r#"[
        {"outfit_id":"established","style_tier":"casual","avg_sti":80,"total_price":3000,"item_count":3,"freshness_hours":10,"impressions":200,"clicks":20,"purchases":2,"color_harmony":0.8,"price_coherence":0.9,"has_video":false},
        {"outfit_id":"new","style_tier":"casual","avg_sti":80,"total_price":3000,"item_count":3,"freshness_hours":1,"impressions":5,"clicks":1,"purchases":0,"color_harmony":0.8,"price_coherence":0.9,"has_video":false}
    ]"#;
    let ctx = r#"{"user_id":null,"county":null,"recent_clicks":[],"recent_purchases":[],"session_hour":12,"is_new_user":true}"#;
    let weights = r#"{"engagement_weight":0.1,"freshness_weight":0.2,"sti_weight":0.1,"price_coherence_weight":0.1,"color_harmony_weight":0.1,"video_boost":0.1,"diversity_penalty":0.05,"cold_start_boost":0.4}"#;

    let result = rank(candidates, ctx, weights);
    let scored: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();

    // New outfit should rank higher with high cold_start_boost
    assert_eq!(scored[0]["outfit_id"], "new");
}

#[test]
fn handles_empty_candidates() {
    let result = rank("[]", "{}", "{}");
    assert_eq!(result, "[]");
}

#[test]
fn handles_invalid_json() {
    let result = rank("not json", "{}", "{}");
    assert_eq!(result, "[]");
}
