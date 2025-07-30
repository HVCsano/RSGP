use serde::{Deserialize, Serialize};

pub struct LatestJson {
    pub web: LatestJsonItem,
    pub manager: LatestJsonItem,
    pub worker: LatestJsonItem,
}

pub struct LatestJsonItem {
    pub version: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Egg {
    pub install: EggInstallConf,
    pub running: EggRunConf,
    pub version: String,
    pub upstream: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EggInstallConf {
    pub dependency_installs: Vec<Vec<String>>,
    pub egg_installs: Vec<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EggRunConf {
    pub start_command: String,
    pub running_text: String,
}
