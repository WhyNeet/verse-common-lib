use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::json::JsonEncoder;

pub fn init_logger() -> anyhow::Result<()> {
    let file_stdout = FileAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build("./logs/server.log")?;
    let log_config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(file_stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))?;
    log4rs::init_config(log_config)?;

    Ok(())
}
