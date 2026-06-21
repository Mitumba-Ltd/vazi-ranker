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
    let weights = r#"{"engagement_weight":0.1,"conversion_weight":0.05,"freshness_weight":0.2,"sti_weight":0.1,"price_coherence_weight":0.1,"color_harmony_weight":0.1,"video_boost":0.1,"diversity_penalty":0.05,"cold_start_boost":0.4}"#;

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

#[test]
fn bayesian_ctr_prevents_low_impression_inflation() {
    // 10 clicks / 60 impressions raw CTR 0.17 vs 100 clicks / 1000 impressions raw CTR 0.10
    // Without Bayesian smoothing, the first would dominate. With smoothing, the proven one wins.
    let candidates = r#"[
        {"outfit_id":"proven","style_tier":"casual","avg_sti":80,"total_price":3000,"item_count":3,"freshness_hours":10,"impressions":1000,"clicks":100,"purchases":10,"color_harmony":0.8,"price_coherence":0.9,"has_video":false},
        {"outfit_id":"fluky","style_tier":"casual","avg_sti":80,"total_price":3000,"item_count":3,"freshness_hours":10,"impressions":60,"clicks":10,"purchases":0,"color_harmony":0.8,"price_coherence":0.9,"has_video":false}
    ]"#;
    let ctx = r#"{"user_id":null,"county":null,"recent_clicks":[],"recent_purchases":[],"session_hour":12,"is_new_user":false}"#;

    let result = rank(candidates, ctx, "{}");
    let scored: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();

    // Proven outfit (with purchases + more data) should rank higher
    assert_eq!(scored[0]["outfit_id"], "proven");
}

#[test]
fn conversion_rate_boosts_purchasing_outfits() {
    let candidates = r#"[
        {"outfit_id":"clicked","style_tier":"casual","avg_sti":80,"total_price":3000,"item_count":3,"freshness_hours":10,"impressions":500,"clicks":50,"purchases":0,"color_harmony":0.8,"price_coherence":0.9,"has_video":false},
        {"outfit_id":"purchased","style_tier":"casual","avg_sti":80,"total_price":3000,"item_count":3,"freshness_hours":10,"impressions":500,"clicks":50,"purchases":20,"color_harmony":0.8,"price_coherence":0.9,"has_video":false}
    ]"#;
    let ctx = r#"{"user_id":null,"county":null,"recent_clicks":[],"recent_purchases":[],"session_hour":12,"is_new_user":false}"#;

    let result = rank(candidates, ctx, "{}");
    let scored: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();

    assert_eq!(scored[0]["outfit_id"], "purchased");
}

#[test]
fn score_never_negative() {
    // Heavy diversity penalty scenario
    let candidates = r#"[
        {"outfit_id":"penalized","style_tier":"casual","avg_sti":10,"total_price":500,"item_count":2,"freshness_hours":500,"impressions":1000,"clicks":1,"purchases":0,"color_harmony":0.1,"price_coherence":0.1,"has_video":false}
    ]"#;
    let ctx = r#"{"user_id":"u1","county":null,"recent_clicks":["casual","casual","casual","casual","casual"],"recent_purchases":[],"session_hour":12,"is_new_user":false}"#;

    let result = rank(candidates, ctx, "{}");
    let scored: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();

    let score = scored[0]["score"].as_f64().unwrap();
    assert!(score >= 0.0, "Score should never be negative, got {}", score);
}
