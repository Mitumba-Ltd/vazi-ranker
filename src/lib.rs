mod scoring;
mod types;

use types::{OutfitCandidate, RankWeights, ScoredOutfit, UserContext};
use wasm_bindgen::prelude::*;

/// Rank outfit candidates by relevance to the user.
///
/// All inputs/outputs are JSON strings for WASM interop.
/// The CF Worker calls this with pre-loaded data — no network calls happen here.
#[wasm_bindgen]
pub fn rank(candidates_json: &str, context_json: &str, weights_json: &str) -> String {
    let candidates: Vec<OutfitCandidate> = match serde_json::from_str(candidates_json) {
        Ok(c) => c,
        Err(_) => return "[]".to_string(),
    };

    let ctx: UserContext = match serde_json::from_str(context_json) {
        Ok(c) => c,
        Err(_) => return "[]".to_string(),
    };

    let weights: RankWeights = serde_json::from_str(weights_json)
        .unwrap_or_default();

    let mut scored: Vec<ScoredOutfit> = candidates
        .iter()
        .map(|c| ScoredOutfit {
            outfit_id: c.outfit_id.clone(),
            score: scoring::compute_score(c, &ctx, &weights),
        })
        .collect();

    // Sort descending by score
    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    serde_json::to_string(&scored).unwrap_or_else(|_| "[]".to_string())
}
