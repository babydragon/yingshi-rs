use super::{APIResponse, TokenData, Result};
use crate::{DeviceInfo, CameraInfo, DeviceStatusInfo, AlarmStatus, Alarm, APIError};
use chrono::Utc;
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use crate::settings::Settings;

static SERVER: &'static str = "https://open.ys7.com";
static TOKEN: &'static str = "/api/lapp/token/get";
static DEVICE_LIST: &'static str = "/api/lapp/device/list";
static CAMERA_LIST: &'static str = "/api/lapp/camera/list";
static DEVICE_INFO: &'static str = "/api/lapp/device/status/get";
static DEVICE_ALARM: &'static str = "/api/lapp/alarm/device/list";

const EXPIRE_DELTA : i64 = 1000;

/// 萤石客户端，通过http服务获取萤石设备信息
pub struct Client {
    http: reqwest::Client,
    setting: Settings,
}

impl Client {
    /// 创建客户端
    pub async fn new(setting: Settings) -> Result<Client> {
        let client = reqwest::Client::new();

        let mut c = Client {
            http: client,
            setting
        };

        c.fetch_token().await?;

        Ok(c)
    }

    async fn fetch_token(& mut self) -> Result<()> {
        let url = SERVER.to_owned() + TOKEN;

        let params = [("appKey", &self.setting.key), ("appSecret", &self.setting.secret)];
        let response = self.http.post(&url).form(&params).send().await?
            .json::<APIResponse<TokenData>>().await?;
        if response.code == "200" {
            let t = response.data.unwrap();
            println!("success get token {}", t.access_token);
            self.setting.token = Some(t.access_token);
            self.setting.expire_time = Some(t.expire_time);

            Ok(())
        } else {
            Err(APIError{
                code: response.code,
                message: response.msg
            })
        }
    }

    async fn token(&mut self) -> Result<String> {
        match &self.setting.token {
            Some(token) => {
                let expire = self.setting.expire_time.unwrap_or_default();
                let current = Utc::now().timestamp_millis();
                // expired
                if expire - current < EXPIRE_DELTA {
                    // fetch again
                    self.fetch_token().await?;
                    let new_token = self.setting.token.as_ref().unwrap();
                    Ok(new_token.clone())
                } else {
                    Ok(token.clone())
                }
            }
            None => {
                // fetch token
                self.fetch_token().await?;
                let new_token = self.setting.token.as_ref().unwrap();
                Ok(new_token.clone())
            }
        }
    }

    async fn request_server<ResponseType: DeserializeOwned>(&mut self, url: String, mut params: HashMap<&str, String>) -> Result<ResponseType> {
        let token = self.token().await?;
        params.insert("accessToken", token);

        let response = self.http.post(&url).form(&params).send().await?
            .json::<APIResponse<ResponseType>>().await?;
        if response.code == "200" {
            Ok(response.data.unwrap())
        } else {
            Err(APIError{
                code: response.code,
                message: format!("fail to request url [{}], error message: {}", url, response.msg)
            })
        }
    }

    /// 列出账号下所有设备
    pub async fn list_device(&mut self) -> Result<Vec<DeviceInfo>> {
        let url = SERVER.to_owned() + DEVICE_LIST;
        let params = HashMap::new();

        let result = self.request_server::<Vec<DeviceInfo>>(url, params).await?;
        Ok(result)
    }

    /// 列出账号下所有摄像头
    pub async fn list_cameras(&mut self) -> Result<Vec<CameraInfo>> {
        let url = SERVER.to_owned() + CAMERA_LIST;
        let params = HashMap::new();

        let response = self.request_server::<Vec<CameraInfo>>(url, params).await?;
        Ok(response)
    }

    /// 获取设备状态，可以通过这个接口获取到电量什么的
    pub async fn device_info(&mut self, device_serial: &str, channel: Option<i8>) -> Result<DeviceStatusInfo> {
        let url = SERVER.to_owned() + DEVICE_INFO;
        let mut params = HashMap::new();
        params.insert("deviceSerial", device_serial.to_owned());

        if let Some(c) = channel {
            params.insert("channel", c.to_string());
        }

        let response = self.request_server::<DeviceStatusInfo>(url, params).await?;
        Ok(response)
    }

    /// 获取设备报警信息
    /// 可选参数：
    /// start_time: 开始时间，默认为当日0点时间戳，单位毫秒
    /// end_time: 结束时间，默认为当前时间戳，单位毫秒
    /// status：报警状态
    pub async fn device_alarm(&mut self, device_serial: &str,
                              start_time: Option<i64>, end_time: Option<i64>, status: Option<AlarmStatus>) -> Result<Vec<Alarm>> {
        let url = SERVER.to_owned() + DEVICE_ALARM;
        let mut params = HashMap::new();
        params.insert("deviceSerial", device_serial.to_owned());

        if let Some(start) = start_time {
            params.insert("startTime", start.to_string());
        }

        if let Some(end) = end_time {
            params.insert("endTime", end.to_string());
        }

        if let Some(s) = status {
            let i = s as i32;
            params.insert("status", i.to_string());
        }

        let response = self.request_server::<Vec<Alarm>>(url, params).await?;
        Ok(response)
    }
}