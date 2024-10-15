pub const DEFAULT_APP_ID: u64 = 1295708432044916796;

pub struct RPCConfig {
    pub app_id: u64
}

impl Default for RPCConfig {
    fn default() -> Self {
        Self {
            app_id: DEFAULT_APP_ID
        }
    }
}

pub struct RPCPlugin {
    pub config: RPCConfig
}

impl RPCPlugin {
    pub fn new(app_id: u64) -> Self {
        Self {
            config: RPCConfig { app_id }
        }
    }
}

impl From<RPCConfig> for RPCPlugin {
    fn from(config: RPCConfig) -> Self {
        Self { config }
    }
}