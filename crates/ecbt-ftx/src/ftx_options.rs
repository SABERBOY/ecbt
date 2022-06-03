use ecbt_exchange::exchange::Environment;
use ecbt_exchange::model::market_pair::MarketPair;
use std::env::var;

// type Endpoint = ecbt_exchange::exchange::Endpoint;

#[derive(Default, Clone, Debug)]
pub struct FtxParameters {
    pub environment: Environment,
    pub options: Options,
}

impl FtxParameters {
    pub fn ws(&self) -> &'static str {
        match self.environment {
            Environment::Production => "wss://ftx.com/ws",
            Environment::Sandbox => "wss://ftx.com/ws",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {
    Com,
    Us,
}

impl Endpoint {
    pub fn ws(&self) -> &'static str {
        match self {
            Endpoint::Com => "wss://ftx.com/ws",
            Endpoint::Us => "wss://ftx.us/ws",
        }
    }

    pub fn rest(&self) -> &'static str {
        match self {
            Endpoint::Com => "https://ftx.com/api",
            Endpoint::Us => "https://ftx.us/api",
        }
    }

    pub fn header_prefix(&self) -> &'static str {
        match self {
            Endpoint::Com => "FTX",
            Endpoint::Us => "FTXUS",
        }
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint::Com
    }
}

#[derive(Debug, Default, Clone)]
pub struct Options {
    pub endpoint: Endpoint,
    pub key: Option<String>,
    pub secret: Option<String>,
    pub subaccount: Option<String>,
}

impl Options {
    const FTX_API_KEY: &'static str = "FTX_API_KEY";
    const FTX_API_SECRET: &'static str = "FTX_API_SECRET";
    const FTX_SUBACCOUNT: &'static str = "FTX_SUBACCOUNT";

    pub fn us() -> Self {
        Options {
            endpoint: Endpoint::Us,
            ..Default::default()
        }
    }

    pub fn from_env() -> Self {
        Options::default()
            .authenticate(
                var(Self::FTX_API_KEY).expect("FTX_API_KEY Key is not defined."),
                var(Self::FTX_API_SECRET).expect("FTX_API_SECRET Secret is not defined."),
            )
            .subaccount_optional(var(Self::FTX_SUBACCOUNT).ok())
    }

    pub fn from_env_us() -> Self {
        Options::us()
            .authenticate(
                var(Self::FTX_API_KEY).expect("FTX_API_KEY Key is not defined."),
                var(Self::FTX_API_SECRET).expect("FTX_API_SECRET Secret is not defined."),
            )
            .subaccount_optional(var(Self::FTX_SUBACCOUNT).ok())
    }

    #[must_use]
    pub fn authenticate(mut self, key: String, secret: String) -> Self {
        self.key = Some(key);
        self.secret = Some(secret);
        self
    }

    #[must_use]
    pub fn subaccount(mut self, subaccount: String) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    #[must_use]
    pub fn subaccount_optional(mut self, subaccount: Option<String>) -> Self {
        self.subaccount = subaccount;
        self
    }

    /// Returns the market pair for the given currency pair.
    /// # Examples Com:{}-{} US:{}/{}
    pub fn to_market(&self, market_pair: MarketPair) -> String {
        match self.endpoint {
            Endpoint::Com => format!("{}-{}", market_pair.0, market_pair.1),
            Endpoint::Us => format!("{}/{}", market_pair.0, market_pair.1),
        }
    }
}
