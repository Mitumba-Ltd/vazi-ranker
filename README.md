# vazi-ranker

The VAZI outfit ranking engine — scores and sorts outfit candidates by engagement, freshness, style coherence, and user affinity.

## What this is

A pure scoring function compiled to WASM. No network calls, no DB access, no side effects. Takes candidates + context + weights, returns a sorted list.

Used by the VAZI Cloudflare Worker to rank outfit feeds in real-time (~1ms per 100 candidates).

## Build

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for Cloudflare Workers
wasm-pack build --target bundler --out-dir pkg

# Run tests
cargo test
```

## Usage (from TypeScript)

```typescript
import { rank } from './pkg/vazi_ranker';

const scored = JSON.parse(rank(
  JSON.stringify(candidates),
  JSON.stringify(userContext),
  JSON.stringify(weights)
));
```

## Scoring factors

| Factor | Weight (default) | Signal |
|---|---|---|
| Engagement (CTR) | 0.30 | clicks / impressions |
| Freshness | 0.20 | Decays over 1 week |
| Seller trust (STI) | 0.15 | avg STI of outfit sellers |
| Price coherence | 0.10 | Items in similar price tier |
| Color harmony | 0.10 | Aesthetic color compatibility |
| Video boost | 0.20 | Outfits with rendered video rank higher |
| Cold start | 0.15 | Boost new outfits with few impressions |
| Diversity penalty | -0.05 | Penalize same style repeating |

Weights are loaded from KV at runtime and updated daily by the RL trainer.

## License

MIT — © 2026 StaNLink Inc. / Mitumba Ltd.
