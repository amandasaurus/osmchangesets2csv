use clap_verbosity_flag::{Verbosity};
use clap::Parser;
use std::path::PathBuf;

/// Convert an OSM changeset dump file to CSV
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct Args {
    /// Path of the input changeset.osm.bz2 file
    #[arg(short, long, value_name="changeset-NNNNNN.osmn.bz2")]
    pub input: PathBuf,

    /// Path to write output data to
    #[arg(short, long, value_name="FILENAME.csv")]
    pub output: PathBuf,

    /// If the output file already exists, overwrite it. By default, exit if the output already
    /// exists
    #[arg(long)]
    pub overwrite: bool,

    /// (Optionally) write *all* changeset tags to this file, columns: changeset_id, key, value.
    /// One line per changeset tag.
    #[arg(long)]
    pub output_tags: Option<PathBuf>,

    /// Columns to include in output
    #[arg(short, long, default_value="changeset_id,created,closed,uid,user,open,num_changes,comments_count,created_by,comment,tag.source→source,tag.imagery_used→imagery_used")]
    pub columns: String,

    #[command(flatten)]
    pub verbose: Verbosity<clap_verbosity_flag::InfoLevel>,

}

