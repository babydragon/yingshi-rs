#[macro_use]
extern crate lazy_static;

pub mod client;
pub mod util;
pub mod settings;

pub use client::Client;
pub use settings::Settings;

use serde::Deserialize;
use serde_repr::Deserialize_repr;
use enumflags2::BitFlags;
use reqwest::Error;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct APIError {
    code: String,
    message: String
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "request api error, code: {}, message {}", self.code, self.message)
    }
}

impl StdError for APIError {

}

impl From<reqwest::Error> for APIError {
    fn from(e: Error) -> Self {
        APIError{
            code: "800".to_string(),
            message: format!("{}", e)
        }
    }
}

type Result<T> = std::result::Result<T, APIError>;

#[derive(Deserialize)]
pub struct Page {
    total: u32,
    page: u32,
    size: u32
}

#[derive(Deserialize)]
pub struct APIResponse<T> {
    code: String,
    msg: String,
    page: Option<Page>,
    data: Option<T>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokenData {
    access_token: String,
    expire_time: i64
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum DeviceStatus {
    Offline = 0,
    Online = 1
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    device_serial: String,
    device_name: String,
    device_type: String,
    status: DeviceStatus,
    defence: u8,
    device_version: String,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum VideoLevel {
    Smooth = 0,
    Balance = 1,
    HD = 2,
    UltraClear = 3
}

#[derive(BitFlags, Copy, Clone, Debug, Deserialize)]
#[repr(u8)]
pub enum Permission {
    Preview = 1 << 0,
    Replay = 1 << 1,
    Alert = 1 << 2,
    VoiceTalk = 1 << 3
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CameraInfo {
    device_serial: String,
    channel_no: u8,
    channel_name: String,
    status: DeviceStatus,
    pic_url: String,
    is_encrypt: u8,
    video_level: VideoLevel,
    permission: Permission
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStatusInfo {
    privacy_status: i8,
    pir_status: i8,
    alarm_sound_mode: i8,
    battry_status: i8,
    lock_signal: i8,
    disk_num: i8,
    disk_state: String,
    cloud_status: i8,
    nvr_disk_num: i8,
    nvr_disk_state: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Alarm {
    alarm_id: String,
    alarm_name: String,
    alarm_type: i32,
    alarm_time: u64,
    channel_no: u8,
    is_encrypt: u8,
    is_checked: u8,
    rec_state: u8,
    pre_time: u32,
    delay_time: u32,
    device_serial: String,
    alarm_pic_url: String,
}

pub enum AlarmStatus {
    ALL = 2,
    READ = 1,
    UNREAD = 0,
}