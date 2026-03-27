#![deny(missing_docs)]

//! 用于将播放信息同步到系统的媒体控件和/或 Discord RPC 的 Rust crate
//!
//! 目前支持 Windows、Linux 和 MacOS 的媒体控件交互

use anyhow::Result;

mod discord;
pub mod model;
mod sys_media;

pub use sys_media::EventCallback;

use model::{
    DiscordConfigPayload, MetadataPayload, PlayModePayload, PlayStatePayload, TimelinePayload,
};

/// 初始化插件
///
/// ### Errors
///
/// 可能会在媒体控件初始化失败时返回错误
///
/// ### 备注
///
/// 如果其他 API 调用失败，则只会打印日志并静默失败
pub fn initialize() -> Result<()> {
    discord::init();

    sys_media::get_platform_controls().initialize()?;

    Ok(())
}

/// 关闭媒体控件，清理资源
pub fn shutdown() {
    discord::disable();
    let _ = sys_media::get_platform_controls().shutdown();
}

/// 启用媒体控件
///
/// ### Errors
///
/// 会在调用 API 失败时返回错误
pub fn enable_system_media() -> Result<()> {
    sys_media::get_platform_controls().enable()
}

/// 禁用媒体控件
///
/// ### Errors
///
/// 会在调用 API 失败时返回错误
pub fn disable_system_media() -> Result<()> {
    sys_media::get_platform_controls().disable()
}

/// 注册媒体控件的事件回调 (上一首、下一首、暂停、播放等)
///
/// ### 参数
///
/// * `callback` - 事件回调函数，接收 [`SystemMediaEvent`] 参数
///
/// ### Errors
///
/// 如果注册回调失败，会返回错误
pub fn register_event_handler(callback: EventCallback) -> Result<()> {
    sys_media::get_platform_controls().register_event_handler(callback)?;

    Ok(())
}

/// 更新歌曲元数据
///
/// 同时也会更新 Discord 的元数据 (如果启用了 Discord RPC)
///
/// ### 备注
///
/// 更新 Discord RPC 的元数据时，必须提供 `original_cover_url`
pub fn update_metadata(payload: MetadataPayload) {
    discord::update_metadata(payload.clone());
    sys_media::get_platform_controls().update_metadata(payload);
}

/// 更新播放状态 (播放/暂停)
///
/// 同时也会更新 Discord 的播放状态 (如果启用了 Discord RPC)
pub fn update_play_state(payload: PlayStatePayload) {
    discord::update_play_state(payload);
    sys_media::get_platform_controls().update_playback_status(payload);
}

/// 更新播放速率
///
/// ### 备注
///
/// 只会更新媒体控件的信息，不会更新 Discord RPC 上的信息
pub fn update_playback_rate(rate: f64) {
    sys_media::get_platform_controls().update_playback_rate(rate);
}

/// 更新音量
///
/// ### 备注
///
/// 只会更新媒体控件的信息，不会更新 Discord RPC 上的信息
pub fn update_volume(volume: f64) {
    sys_media::get_platform_controls().update_volume(volume);
}

/// 更新进度信息
///
/// 同时也会更新 Discord 的进度信息 (如果启用了 Discord RPC)
///
/// ### 备注
///
/// Discord RPC 实现的进度更新有节流，调用此函数无需担心 Discord RPC 的速率限制
pub fn update_timeline(payload: TimelinePayload) {
    discord::update_timeline(payload);
    sys_media::get_platform_controls().update_timeline(payload);
}

/// 更新播放模式
///
/// ### 备注
///
/// 只会更新媒体控件的信息，不会更新 Discord RPC 上的信息
pub fn update_play_mode(payload: PlayModePayload) {
    sys_media::get_platform_controls().update_play_mode(payload);
}

/// 启用 Discord RPC
///
/// ### 备注
///
/// 启用后会立刻尝试连接，如果 Discord 未启动，或因为其他未知原因连接失败，会每 5 秒尝试连接一次
pub fn enable_discord_rpc() {
    discord::enable();
}

/// 关闭 Discord RPC
pub fn disable_discord_rpc() {
    discord::disable();
}

/// 更新 Discord RPC 的配置
///
/// ### 参数
///
/// * `payload` - 配置信息，可以配置是否在暂停后也显示 Discord Activity 和 状态显示风格。详情请查看
///   [`DiscordConfigPayload`]
pub fn update_discord_config(payload: DiscordConfigPayload) {
    discord::update_config(payload);
}
