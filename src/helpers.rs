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