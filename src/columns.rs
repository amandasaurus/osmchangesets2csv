#![allow(warnings)]
use std::borrow::Cow;
use std::str::FromStr;

const POPULAR_CHANGESET_TAGS: &[&str] = &[
    "created_by",
    "comment",
    "imagery_used",
    "locale",
    "source",
    "host",
    "changesets_count",
    "hashtags",
    "version",
    "build",
    "review_requested",
    "bundle_id",
    "platform",
    "browser",
    "bot",
    "warnings",
];


/// CSV output column
// last inner String is the column name
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Column {
    ChangesetId(String),
    Created(String),
    Closed(String),
    Uid(String),
    User(String),
    Open(String),
    NumChanges(String),
    CommentsCount(String),
    Tag(String, String),

}

impl FromStr for Column {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let mut args = val.splitn(2, &['→', ':']).map(|s| s.trim().to_string()).collect::<Vec<String>>();
        if args.len() == 1 {
            args.push(args[0].clone());
        }
        let title = args.pop().unwrap();
        let code = args.pop().unwrap();
        match code.as_str() {
            "changeset_id" => Ok(Column::ChangesetId(title)),
            "created" => Ok(Column::Created(title)),
            "closed" => Ok(Column::Closed(title)),
            "uid" => Ok(Column::Uid(title)),
            "user" => Ok(Column::User(title)),
            "open" => Ok(Column::Open(title)),
            "num_changes" => Ok(Column::NumChanges(title)),
            "comments_count" => Ok(Column::CommentsCount(title)),

            col => {
                if let Some(tag) = col.strip_prefix("tag.") {
                    Ok(Column::Tag(tag.to_string(), title))
                } else if POPULAR_CHANGESET_TAGS.contains(&col) {
                    Ok(Column::Tag(col.to_string(), title))
                } else {
                    Err(anyhow::anyhow!("Unknown column value: {}", col))
                }
            },
        }
    }
}

impl Column {
    pub(crate) fn header(&self) -> Cow<str> {
        match &self {
            Column::ChangesetId(title) => title.into(),
            Column::Created(title) => title.into(),
            Column::Closed(title) => title.into(),
            Column::Uid(title) => title.into(),
            Column::User(title) => title.into(),
            Column::Open(title) => title.into(),
            Column::NumChanges(title) => title.into(),
            Column::CommentsCount(title) => title.into(),
            Column::Tag(_tag, title) => title.into(),
        }
    }
}
