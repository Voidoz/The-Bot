use serenity::model::id::ChannelId;

// Set constant so we know whether the binary was compiled in release mode or not
#[cfg(debug_assertions)]
pub const IS_DEBUG: bool = true;
#[cfg(not(debug_assertions))]
pub const IS_DEBUG: bool = false;

pub fn should_handle(channel_id: &ChannelId, dev_channel: &str) -> bool {
    let is_dev_channel = dev_channel == &channel_id.to_string();

    IS_DEBUG == is_dev_channel
}

pub fn search_string(source: &String, query: &String, is_case_sensitive: bool) -> Option<String> {
    let source_str = if is_case_sensitive { source.to_owned() } else { source.to_lowercase() };
    let query_str = if is_case_sensitive { query.to_owned() } else { query.to_lowercase() };

    if source_str == query_str { return Some(query_str); }

    let mut query_pad_start = " ".to_owned();
    query_pad_start.push_str(&query_str);

    let mut query_pad_all = query_pad_start.to_owned();
    query_pad_all.push_str(" ");

    let mut query_pad_end = query_str;
    query_pad_end.push_str(" ");

    if      source_str.starts_with(&query_pad_end) { return Some(query_pad_end) }
    else if source_str.contains(&query_pad_all)      { return Some(query_pad_all) }
    else if source_str.ends_with(&query_pad_start)     { return Some(query_pad_start) }
    else                                                 { return None };
}