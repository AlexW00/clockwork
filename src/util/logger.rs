use simplelog::{Config, ConfigBuilder};

pub(crate) fn init(name: String, version: i32) {
    if let Some(local_dir) = dirs::data_local_dir() {
        // check if clockwork-vst folder exists, if not create it
        let mut path = local_dir.clone();
        path.push("clockwork-vst");
        path.push("logs");
        if !path.exists() {
            std::fs::create_dir_all(&path).ok();
        }

        let logging_dir = path.clone();
        if logging_dir.exists() {
            let filename = format!("{name}-{version}-log.txt");
            let log_file_path = logging_dir.join(filename);

            if let Ok(log_file) = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&log_file_path)
            {
                simplelog::WriteLogger::init(simplelog::LevelFilter::Info, Config::default(), log_file)
                    .ok();
                log_panics::init();
                log::info!("Starting VST");

            } else {
                panic!("Could not open log file");
            }
        }
    }
}
