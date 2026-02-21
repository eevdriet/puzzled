use directories::ProjectDirs;

pub fn dirs() -> Option<ProjectDirs> {
    let author = env!("CARGO_PKG_AUTHORS").split(",").next()?;
    let app = env!("CARGO_PKG_NAME");

    ProjectDirs::from("com", author, app)
}
