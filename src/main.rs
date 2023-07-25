use anyhow::Result;
use clap::Parser;
use indicatif::ProgressBar;
#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, trace, warn, Level};
use osmio;
use osmio::changesets::ChangesetReader;
use read_progress::ReadWithSize;

mod columns;
use columns::Column;
mod cli_args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli_args::Args::parse();

    env_logger::Builder::from_env(env_logger::Env::default())
        .filter_level(args.verbose.log_level_filter())
        .init();

    let columns: Vec<Column> = args
        .columns
        .split(',')
        .map(|col_str| col_str.parse())
        .collect::<Result<Vec<Column>>>()?;

    if args.output.exists() && args.output.is_file() && !args.overwrite {
        // check .is_file(), so we can do -o /dev/null
        error!("Output file {} already exists and --overwrite not used. Refusing to overwrite, and exiting early", &args.output.display());
        anyhow::bail!("Output file {} already exists and --overwrite not used. Refusing to overwrite, and exiting early", &args.output.display());
    }

    let mut reader =
        ChangesetReader::from_bz2_reader(read_progress::ReaderWithSize::from_path(args.input)?);
    let mut output = csv::Writer::from_path(args.output)?;
    let bar = ProgressBar::new(1000);
    bar.set_style(indicatif::ProgressStyle::with_template("Reading changesets file [{elapsed_precise}] {bar:40.cyan/blue} {percent:>3}% ETA: {eta}")
        .unwrap());

    for c in &columns {
        output.write_field(c.header().as_ref())?;
    }
    output.write_record(None::<&[u8]>)?;

    let mut output_tags = if let Some(output_tags) = args.output_tags {
        if output_tags.exists() && output_tags.is_file() && !args.overwrite {
            error!("Output file {} already exists and --overwrite not used. Refusing to overwrite, and exiting early", &output_tags.display());
            anyhow::bail!("Output file {} already exists and --overwrite not used. Refusing to overwrite, and exiting early", &output_tags.display());
        }

        let mut output_tags = csv::Writer::from_path(output_tags)?;
        output_tags.write_record(&["changeset_id", "key", "value"])?;
        Some(output_tags)
    } else {
        None
    };

    while let Some(c) = reader.next_changeset()? {
        bar.set_position((1000. * reader.get_ref().get_ref().fraction()).round() as u64);
        for col in &columns {
            match col {
                Column::ChangesetId(_title) => output.write_field(c.id.to_string())?,
                Column::Created(_title) => output.write_field(c.created.to_iso_string())?,
                Column::Closed(_title) => output.write_field(
                    c.closed
                        .clone()
                        .map_or("".to_string(), |cl| cl.to_iso_string()),
                )?,
                Column::Uid(_title) => {
                    output.write_field(c.uid.clone().map_or("".to_string(), |x| x.to_string()))?
                }
                Column::User(_title) => {
                    output.write_field(c.user.clone().map_or("".to_string(), |x| x.to_string()))?
                }
                Column::Open(_title) => output.write_field(c.open.to_string())?,
                Column::NumChanges(_title) => output.write_field(c.num_changes.to_string())?,
                Column::CommentsCount(_title) => {
                    output.write_field(c.comments_count.to_string())?
                }

                Column::Tag(tag, _title) => {
                    output.write_field(c.tag(tag).unwrap_or("").to_string())?
                }
            }
        }

        if let Some(ref mut output_tags) = output_tags {
            let c_id = c.id.to_string();
            for (k, v) in c.tags.iter() {
                output_tags.write_record(&[&c_id, k, v])?;
            }
        }

        output.write_record(None::<&[u8]>)?;
    }

    bar.finish();
    output.flush()?;

    Ok(())
}
