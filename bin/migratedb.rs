use ed_migratedb::EdMigrationOptions;
use simplelog::{Config, LevelFilter, SimpleLogger};
use tern::App;

#[tokio::main]
async fn main() {
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    let app = App::new(EdMigrationOptions);

    if let Err(e) = app.run().await {
        log::error!("{e}");
        std::process::exit(1);
    }
}
