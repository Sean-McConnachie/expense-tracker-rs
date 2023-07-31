use expenses_backend::{utils, database};

#[tokio::main]
async fn main() {
    let config: utils::config::Config =
        utils::config::read_config("Config.toml").expect("Failed to read config.");

    utils::logger::init_logger(config.log_level());

    let db_pool = database::connect_db(config.db_config())
        .await
        .expect("Failed to connect to database.");


    utils::startup(&config, &db_pool).await.expect("Failed to run startup tasks.");

    utils::args::run(config, &db_pool)
        .await
        .expect("Failed to run application.");
}
