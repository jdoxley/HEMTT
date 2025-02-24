use std::io::{copy, BufReader, BufWriter};
use std::path::Path;

use crate::{Command, HEMTTError, Project};

pub struct Zip {}
impl Command for Zip {
    fn register(&self) -> clap::App {
        clap::SubCommand::with_name("zip")
            .about("Create a .zip of the latest release")
            .arg(clap::Arg::with_name("name").help("Name of the archive").default_value(""))
    }

    fn run(&self, args: &clap::ArgMatches, p: Project) -> Result<(), HEMTTError> {
        archive(args.value_of("name").unwrap(), p)
    }
}

pub fn archive(name: &str, p: Project) -> Result<(), HEMTTError> {
    let version = p.version()?;

    let release_dir = format!("releases/{}", version);

    let zipname = format!(
        "{}.zip",
        match name {
            "" => format!("{}_{}", p.name.replace(" ", "_"), version),
            _ => p.render(name, None)?,
        }
    );
    debug!("Archiving: {}", zipname);

    let zipsubpath = format!("releases/{}", zipname);
    let zippath = Path::new(&zipsubpath);
    let file = BufWriter::new(create_file!(&zippath)?);

    let dir = walkdir::WalkDir::new(&release_dir);
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // Zip all files and folders in all subdirectories
    for entry in dir.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(&release_dir)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            let _ = zip.start_file(&release_dir, options);
            // zip.start_file(name, options)?;

            let mut f = BufReader::new(open_file!(path)?);

            // Copy directly, without any buffer, as we have no use for the intermediate data
            trace!("Adding file: {}", name.display());
            copy(&mut f, &mut zip)?;
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            trace!("Adding directory: {}", name.display());
            let _ = zip.add_directory(&release_dir, options);
            // zip.add_directory(name, options)?;
        }
    }

    Ok(())
}
