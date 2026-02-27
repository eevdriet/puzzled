use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use puzzled_core::Puzzle;

pub fn puzzle_dir<P>() -> std::io::Result<PathBuf>
where
    P: Puzzle,
{
    let proj_dirs = ProjectDirs::from("com", "eevdriet", "puzzled")
        .ok_or_else(|| std::io::Error::other("Could not determine project directory"))?;

    let dir = proj_dirs.data_dir().join(P::NAME.to_lowercase());

    fs::create_dir_all(&dir)?;

    Ok(dir)
}
