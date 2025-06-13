use crate::constants::DIST_DIRECTORY;
use rocket::fs::NamedFile;
use std::path::PathBuf;

pub mod auth;
pub mod entity;
pub mod metadata;

#[get("/<path..>", rank = 2)]
pub async fn spa_fallback(path: PathBuf) -> Option<NamedFile> {
    let file_path = DIST_DIRECTORY.join(&path);

    if file_path.is_file() {
        return NamedFile::open(file_path).await.ok();
    }

    NamedFile::open(DIST_DIRECTORY.join("index.html"))
        .await
        .ok()
}
