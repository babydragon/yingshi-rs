use clap::Clap;

#[derive(Clap)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCmd
}

#[derive(Clap)]
pub enum SubCmd {
    Config(Config),
    Device(DeviceOpt),
    Alarm(AlarmOpt),
    Decrypt(DecryptOpt),
}

#[derive(Clap)]
pub struct Config {
    #[clap(short, long)]
    pub key: String,
    #[clap(short, long)]
    pub secret: String,
    #[clap(short, long)]
    pub default_device: Option<String>
}

#[derive(Clap, Debug)]
pub enum DeviceAction {
    List,
    Info,
}

#[derive(Clap)]
pub struct DeviceOpt {
    #[clap(short, long)]
    pub serial: Option<String>,
    #[clap(arg_enum)]
    pub action: DeviceAction
}

#[derive(Clap)]
pub struct AlarmOpt {
    #[clap(short, long)]
    pub serial: Option<String>,
    #[clap(long, about="alarm start time, format yyyy-mm-dd HH:MM:ss")]
    pub start: Option<String>,
    #[clap(long, about="alarm start time, format yyyy-mm-dd HH:MM:ss")]
    pub end: Option<String>,
}

#[derive(Clap)]
pub struct DecryptOpt {
    #[clap(short, long)]
    pub file: String,
    #[clap(short, long)]
    pub code: String,
}