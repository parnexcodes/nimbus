use env_logger::Env;

pub fn init_logger(default_level: &str) {
    env_logger::Builder::from_env(Env::default().default_filter_or(default_level)).init();
}