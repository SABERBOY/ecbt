use ecbt_exchange::model::market_pair::MarketPair as OMarketPair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketPair(pub String);

impl From<OMarketPair> for MarketPair {
    fn from(from: OMarketPair) -> MarketPair {
        MarketPair(format!("{}-{}", from.0, from.1).to_uppercase())
    }
}
