pub struct LatestJson {
    pub web: LatestJsonItem,
    pub manager: LatestJsonItem,
    pub worker: LatestJsonItem,
}

pub struct LatestJsonItem {
    pub version: String,
    pub url: String,
}
