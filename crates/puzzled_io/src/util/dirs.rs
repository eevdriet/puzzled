use std::{fs, io, path::PathBuf};

use directories::ProjectDirs;
use puzzled_core::Puzzle;

const QUALIFIER: &str = "";
const ORGANIZATION: &str = "eevdriet";
const APPLICATION: &str = "puzzled";

fn project_dirs() -> io::Result<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or_else(|| io::Error::other(format!("Could not determine {APPLICATION} directories")))
}

pub fn puzzle_dir<P>() -> io::Result<PathBuf>
where
    P: Puzzle,
{
    let dir = project_dirs()?.data_dir().join(P::NAME.to_lowercase());
    fs::create_dir_all(&dir)?;

    Ok(dir)
}

pub fn puzzle_config_dir<P>() -> io::Result<PathBuf>
where
    P: Puzzle,
{
    let dir = project_dirs()?.config_dir().join(P::NAME.to_lowercase());
    fs::create_dir_all(&dir)?;

    Ok(dir)
}
