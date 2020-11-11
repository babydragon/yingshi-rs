use clap::Clap;
use opt::Opts;
use crate::opt::{SubCmd, DeviceAction};
use crate::opt::DeviceAction::Info;
use chrono::NaiveDateTime;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use yingshi_client::{Settings, Client, util, AlarmStatus};

mod opt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opts = Opts::parse();

    match opt.subcmd {
        SubCmd::Config(config) => {
            let mut s = Settings::new(config.key, config.secret);
            if let Some(_) = config.default_device {
                s.default_device_serial = config.default_device
            }

            s.save().await?;
        }
        SubCmd::Device(device_opt) => {
            match device_opt.action {
                DeviceAction::List => {
                    // list all device
                    let settings = Settings::load().expect("fail to load settings");
                    let mut client = Client::new(settings).await?;

                    let devices = client.list_device().await?;

                    println!("I have these devices: {:?}", devices);
                }
                DeviceAction::Info => {
                    let settings = Settings::load().expect("fail to load settings");
                    let mut serial: Option<String> = None;
                    if settings.default_device_serial != None {
                        serial = settings.default_device_serial.clone()
                    }

                    if device_opt.serial != None {
                        serial = device_opt.serial
                    }
                    if serial == None {
                        panic!("serial must provided when query device info")
                    }
                    let mut client = Client::new(settings).await?;

                    let device_info = client.device_info(serial.unwrap().as_str(), None).await?;

                    println!("{:?}", device_info);
                }
            }
        }
        SubCmd::Alarm(alarm_opt) => {
            let settings = Settings::load().expect("fail to load settings");
            let mut serial: Option<String> = None;
            if settings.default_device_serial != None {
                serial = settings.default_device_serial.clone()
            }

            if alarm_opt.serial != None {
                serial = alarm_opt.serial
            }
            if serial == None {
                panic!("serial must provided when query device info")
            }

            let mut client = Client::new(settings).await?;

            let mut start_time: Option<i64> = None;
            let mut end_time: Option<i64> = None;

            if let Some(start) = alarm_opt.start {
                let result = NaiveDateTime::parse_from_str(start.as_str(), "%Y-%m-%d %H:%M:%S");
                if let Ok(t) = result {
                    start_time = Some(t.timestamp_millis());
                }
            }

            if let Some(end) = alarm_opt.end {
                let result = NaiveDateTime::parse_from_str(end.as_str(), "%Y-%m-%d %H:%M:%S");
                if let Ok(t) = result {
                    end_time = Some(t.timestamp_millis());
                }
            }

            let alarms = client.device_alarm(serial.unwrap().as_str(), start_time, end_time, Some(AlarmStatus::ALL)).await?;
            println!("{:?}", alarms);
        }
        SubCmd::Decrypt(decrypt_opt) => {
            let file_name = decrypt_opt.file;

            let mut f = File::open(file_name).await?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).await?;

            let content = util::decrypt(buffer.as_mut_slice(), decrypt_opt.code.as_str())?;

            let mut result_file = File::create("decrypt_file.jpg").await?;
            result_file.write_all(content).await?;
        }
    }

    Ok(())
}
