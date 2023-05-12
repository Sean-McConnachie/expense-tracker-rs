use expenses_backend::database;
use expenses_backend::utils::{args, config, logger};

#[tokio::main]
async fn main() {
    let config: config::Config =
        config::read_config("Config.toml").expect("Failed to read config.");

    logger::init_logger(config.log_level());

    let db_pool = database::connect_db(config.db_config())
        .await
        .expect("Failed to connect to database.");

    database::initialize::initialize_db(&db_pool)
        .await
        .expect("Failed to initialize database.");

    args::run(config, &db_pool)
        .await
        .expect("Failed to run application.");
}
