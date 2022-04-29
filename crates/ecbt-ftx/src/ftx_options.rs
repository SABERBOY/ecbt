use ecbt_exchange::exchange::Environment;
use std::env::var;

#[derive(Default, Clone, Debug)]
pub struct FtxParameters {
    pub environment: Environment,
    pub options: Options,
}

impl FtxParameters {
    pub fn ws(&self) -> &'static str {
        match self.environment {
            Environment::Production => "wss://ftx.com/ws",
            Environment::Sandbox => "wss://ftxsandbox.com/ws",
        }
    }
}

#[derive(Debug, Clone)]
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
    pub fn us() -> Self {
        Options {
            endpoint: Endpoint::Us,
            ..Default::default()
        }
    }

    pub fn from_env() -> Self {
        Options::default()
            .authenticate(
                var("API_KEY").expect("API Key is not defined."),
                var("API_SECRET").expect("API Secret is not defined."),
            )
            .subaccount_optional(var("SUBACCOUNT").ok())
    }

    pub fn from_env_us() -> Self {
        Options::us()
            .authenticate(
                var("API_KEY").expect("API Key is not defined."),
                var("API_SECRET").expect("API Secret is not defined."),
            )
            .subaccount_optional(var("SUBACCOUNT").ok())
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
}
