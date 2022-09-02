use std::env;
use nih_plug::nih_log;

pub(crate) fn init(name: String, version: String) {

    let key = "NIH_LOG";
    env::set_var(key, "~/cw_log.txt");

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

            nih_log!("Logger initialized");
        }
    }
}
