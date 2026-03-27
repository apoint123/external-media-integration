#![allow(missing_docs)]
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemMediaEventType {
    Play,
    Pause,
    Stop,
    NextSong,
    PreviousSong,
    ToggleShuffle,
    ToggleRepeat,
    SetRate,
    SetVolume,
    /// 绝对位置，毫秒
    Seek,
}

#[derive(Clone, Debug)]
pub struct SystemMediaEvent {
    pub type_: SystemMediaEventType,
    pub position_ms: Option<f64>,
    pub rate: Option<f64>,
    pub volume: Option<f64>,
}

impl SystemMediaEvent {
    #[must_use]
    pub const fn new(t: SystemMediaEventType) -> Self {
        Self {
            type_: t,
            position_ms: None,
            rate: None,
            volume: None,
        }
    }
    #[must_use]
    pub const fn seek(pos: f64) -> Self {
        Self {
            type_: SystemMediaEventType::Seek,
            position_ms: Some(pos),
            rate: None,
            volume: None,
        }
    }
    #[must_use]
    pub const fn set_rate(rate: f64) -> Self {
        Self {
            type_: SystemMediaEventType::SetRate,
            position_ms: None,
            rate: Some(rate),
            volume: None,
        }
    }
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    #[must_use]
    pub const fn set_volume(volume: f64) -> Self {
        Self {
            type_: SystemMediaEventType::SetVolume,
            position_ms: None,
            rate: None,
            volume: Some(volume),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MetadataPayload {
    pub song_name: String,
    pub author_name: String,
    pub album_name: String,

    /// 封面的原始字节数据，适用于除 Discord RPC 之外的其他平台
    pub cover_data: Option<Vec<u8>>,

    /// 封面的 HTTP URL，更新 Discord RPC 时必传，其他平台可不传
    ///
    /// Linux 平台在没有提供 `cover_data` 时会使用它
    pub original_cover_url: Option<String>,

    /// 网易云音乐中对应的曲目 ID
    ///
    /// ### 用途
    /// - 以 "NCM-{ID}" 的格式上传到 SMTC 的 "流派" 字段
    /// - 生成 Discord RPC 的按钮链接
    /// - MacOS 和 Linux 会使用此值来填充唯一的曲目 ID
    pub ncm_id: Option<i64>,

    /// 当前歌曲时长，单位是毫秒
    ///
    /// 用于 Linux、MacOS、Discord RPC 的元数据更新。Windows 使用 [`TimelinePayload`] 的
    /// `total_time` 字段。
    pub duration: Option<f64>,
}

impl fmt::Debug for MetadataPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MetadataPayload")
            .field("song_name", &self.song_name)
            .field("author_name", &self.author_name)
            .field("album_name", &self.album_name)
            .field(
                "cover_data",
                &self.cover_data.as_ref().map_or_else(
                    || "None".to_string(),
                    |bytes| format!("Some({} bytes)", bytes.len()),
                ),
            )
            .field("original_cover_url", &self.original_cover_url)
            .field("ncm_id", &self.ncm_id)
            .field("duration", &self.duration)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    None,
    Track,
    List,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayStatePayload {
    pub status: PlaybackStatus,
}

#[derive(Debug, Clone, Copy)]
pub struct TimelinePayload {
    /// 单位是毫秒
    pub current_time: f64,

    /// 单位是毫秒
    pub total_time: f64,

    /// 是否为 seek 操作触发的更新
    pub seeked: Option<bool>,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayModePayload {
    pub is_shuffling: bool,
    pub repeat_mode: RepeatMode,
}

/// Discord 显示模式枚举
///
/// 不打开详细信息面板时，在用户名下方显示的小字
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscordDisplayMode {
    /// Listening to SPlayer
    Name,
    /// Listening to Rick Astley
    State,
    /// Listening to Never Gonna Give You Up
    Details,
}

/// Discord 配置参数
#[derive(Debug, Clone)]
pub struct DiscordConfigPayload {
    /// 暂停时是否显示
    ///
    /// 注意暂停时进度会固定为 0
    pub show_when_paused: bool,

    /// 显示模式，参考 [`DiscordDisplayMode`]
    pub display_mode: Option<DiscordDisplayMode>,
}
