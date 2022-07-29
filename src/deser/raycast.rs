use super::*;
use crate::structures::item::Item;

const FIELD_SEP_ESCAPE_CHAR: char = '\x16';

pub fn write(item: &Item) -> String {
    format!(
        "{hash}{delimiter}{tags}{delimiter}{comment}{delimiter}{icon}{delimiter}{snippet}\n",
        hash = item.hash(),
        tags = item.tags,
        comment = item.comment,
        delimiter = FIELD_SEP_ESCAPE_CHAR,
        icon = item.icon.clone().unwrap_or_default(),
        snippet = &item.snippet.trim_end_matches(LINE_SEPARATOR),
    )
}

pub fn read(line: &str) -> Result<Item> {
    let mut parts = line.split(FIELD_SEP_ESCAPE_CHAR);
    let hash: u64 = parts
        .next()
        .context("no hash")?
        .parse()
        .context("hash not a u64")?;
    let tags = parts.next().context("no tags")?.into();
    let comment = parts.next().context("no comment")?.into();
    let icon_str = parts.next().context("no icon")?;
    let snippet = parts.next().context("no snippet")?.into();

    let icon = if icon_str.is_empty() {
        None
    } else {
        Some(icon_str.into())
    };

    let item = Item {
        tags,
        comment,
        icon,
        snippet,
        ..Default::default()
    };

    if item.hash() != hash {
        dbg!(&item.hash());
        dbg!(hash);
        Err(anyhow!("Incorrect hash"))
    } else {
        Ok(item)
    }
}
