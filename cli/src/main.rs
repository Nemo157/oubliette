use anyhow::Context;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "oubliette", setting = structopt::clap::AppSettings::ColoredHelp)]
enum Oubliette {
    Archive {
        /// Directory to archive
        directory: String,
        /// Index file to write
        index: String,
    },
    Get {
        /// Index file to read
        index: String,
        /// File to read from archive
        path: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Oubliette::from_args_safe()?;

    match args {
        Oubliette::Archive { directory, index } => {
            let mut builder =
                oubliette::ArchiveBuilder::new(oubliette::ZstdCodec::new(10), 10 * 1024 * 1024);
            builder.add_directory(&directory)?;
            let archive = builder.finish()?;
            let (index_data, data) = archive.serialize();
            std::fs::write(&index, index_data)?;
            std::fs::write(index + ".data", data)?;
        }
        Oubliette::Get { index, path } => {
            let archive = oubliette::Archive::new(
                oubliette::Index::deserialize(std::fs::read(&index)?)?,
                std::fs::read(index + ".data")?,
            );
            let data = archive
                .get(&path)?
                .with_context(|| format!("File {} not found in index", path))?;
            std::io::copy(&mut data.as_slice(), &mut std::io::stdout().lock())?;
        }
    }

    Ok(())
}
