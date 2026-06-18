use crate::types::{OutfitCandidate, RankWeights, UserContext};

pub fn compute_score(c: &OutfitCandidate, ctx: &UserContext, w: &RankWeights) -> f32 {
    let ctr = if c.impressions > 0 {
        c.clicks as f32 / c.impressions as f32
    } else {
        0.0
    };

    // Freshness: decays over 1 week (168 hours)
    let freshness = 1.0 / (1.0 + (c.freshness_hours as f32 / 168.0));

    // Video multiplier
    let video_mult = if c.has_video { 1.0 + w.video_boost } else { 1.0 };

    // Cold start: boost new outfits with few impressions
    let cold_start = if c.impressions < 50 {
        w.cold_start_boost * (1.0 - c.impressions as f32 / 50.0)
    } else {
        0.0
    };

    // Style affinity: slight boost if user recently clicked same style
    let style_affinity = if ctx.recent_clicks.iter().any(|s| s == &c.style_tier) {
        0.1
    } else {
        0.0
    };

    // Diversity: penalize if same style as many recent clicks (avoid echo chamber)
    let style_count = ctx.recent_clicks.iter().filter(|s| *s == &c.style_tier).count();
    let diversity = if style_count > 2 {
        -(w.diversity_penalty * style_count as f32)
    } else {
        0.0
    };

    let base = (ctr * w.engagement_weight)
        + (freshness * w.freshness_weight)
        + ((c.avg_sti / 100.0) * w.sti_weight)
        + (c.price_coherence * w.price_coherence_weight)
        + (c.color_harmony * w.color_harmony_weight)
        + cold_start
        + style_affinity
        + diversity;

    base * video_mult
}
