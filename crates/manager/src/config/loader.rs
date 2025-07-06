use std::path::Path;

use tokio::fs;

pub async fn load_configs() {
    let path = Path::new("./config");
    if !path.exists() {
        fs::create_dir(path).await.unwrap();
    }
}
