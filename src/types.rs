use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OutfitCandidate {
    pub outfit_id: String,
    pub style_tier: String,
    pub avg_sti: f32,
    pub total_price: u32,
    pub item_count: u8,
    pub freshness_hours: u32,
    pub impressions: u32,
    pub clicks: u32,
    pub purchases: u32,
    pub color_harmony: f32,
    pub price_coherence: f32,
    pub has_video: bool,
}

#[derive(Debug, Deserialize)]
pub struct UserContext {
    pub user_id: Option<String>,
    pub county: Option<String>,
    pub recent_clicks: Vec<String>,
    pub recent_purchases: Vec<String>,
    pub session_hour: u8,
    pub is_new_user: bool,
}

#[derive(Debug, Deserialize)]
pub struct RankWeights {
    pub engagement_weight: f32,
    pub freshness_weight: f32,
    pub sti_weight: f32,
    pub price_coherence_weight: f32,
    pub color_harmony_weight: f32,
    pub video_boost: f32,
    pub diversity_penalty: f32,
    pub cold_start_boost: f32,
}

impl Default for RankWeights {
    fn default() -> Self {
        Self {
            engagement_weight: 0.30,
            freshness_weight: 0.20,
            sti_weight: 0.15,
            price_coherence_weight: 0.10,
            color_harmony_weight: 0.10,
            video_boost: 0.20,
            diversity_penalty: 0.05,
            cold_start_boost: 0.15,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ScoredOutfit {
    pub outfit_id: String,
    pub score: f32,
}
