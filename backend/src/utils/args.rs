use crate::{utils::config, web};
use anyhow::Result;

use clap::{Parser, Subcommand};
use log::info;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct EntryPoint {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(subcommand, about = "Program utilities", long_about = None)]
    Utils(Utils),
    #[command(subcommand, about = "User commands", long_about = None)]
    User(User),
    #[command(subcommand, about = "Category commands", long_about = None)]
    Category(Category),
    #[command(subcommand, about = "Expense commands", long_about = None)]
    Expenses(Expenses),
    #[command(subcommand, about = "CLI commands", long_about = None)]
    Cli(Cli),
    #[command(subcommand, about = "Web commands", long_about = None)]
    Web(Web),
}

#[derive(Debug, Subcommand)]
enum Utils {
    // #[command(about = "Initialize the database", long_about = None)]
    // Initialize,
}

#[derive(Debug, Subcommand)]
enum User {
    #[command(about = "List all users", long_about = None)]
    List,
    #[command(about = "Create a user", long_about = None)]
    Create {
        #[arg(short, long)]
        name: String,
    },
    #[command(about = "Delete a user", long_about = None)]
    Delete {
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Debug, Subcommand)]
enum Category {
    #[command(about = "List all categories", long_about = None)]
    List,
    #[command(about = "Create a category", long_about = None)]
    Create {
        #[arg(short, long)]
        name: String,
    },
    #[command(about = "Delete a category", long_about = None)]
    Delete {
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Debug, Subcommand)]
enum Expenses {
    #[command(about = "List all categories", long_about = None)]
    List,
}

#[derive(Debug, Subcommand)]
enum Cli {
    #[command(about = "Start the command line interface", long_about = None)]
    Start,
}

#[derive(Debug, Subcommand)]
enum Web {
    #[command(about = "Start the web server", long_about = None)]
    Start,
}

pub async fn run(config: config::Config, db_pool: &sqlx::PgPool) -> Result<()> {
    let entry_point = EntryPoint::parse();

    match &entry_point.command {
        Commands::Utils(utils) => match utils {
            // Utils::Initialize => {}
            _ => unreachable!(),
        },
        Commands::User(user) => match user {
            User::List => {
                info!("Listing users");
                unimplemented!()
                // let usrs = users::list(&db).await?;
                // println!("Users:");
                // for user in usrs {
                //     println!("\t{}", user?);
                // }
            }
            User::Create { name } => {
                info!("Creating user {}", name);
                unimplemented!()
                // users::create(name, &db).await?;
            }
            User::Delete { name } => {
                info!("Deleting user {}", name);
                unimplemented!()
                // users::delete_via_name(name, &db).await?;
            }
        },
        Commands::Category(category) => match category {
            Category::List => {
                info!("Listing categories");
                unimplemented!()
                // let cats = categories::list(&db).await?;
                // println!("Categories:");
                // for category in cats {
                //     println!("\t{}", category?);
                // }
            }
            Category::Create { name } => {
                info!("Creating category {}", name);
                unimplemented!()
                // categories::create(name, &db).await?;
            }
            Category::Delete { name } => {
                info!("Deleting category {}", name);
                unimplemented!()
                // categories::delete_via_name(name, &db).await?;
            }
        },
        Commands::Expenses(expenses) => match expenses {
            Expenses::List => {
                info!("Listing expenses");
                unimplemented!()
                // let exps = expenses::list_all(&db).await?;
                // for (i, expense) in exps.iter().enumerate() {
                //     println!("Expense #{} ====================================", i + 1);
                //     println!("{expense}",);
                // }
            }
        },
        Commands::Cli(cli) => match cli {
            Cli::Start => {
                info!("Starting CLI");
                unimplemented!()
                // cli::run(&db_pool).await?;
            }
        },
        Commands::Web(web) => match web {
            Web::Start => {
                info!("Starting web server");
                web::run(config, db_pool.clone()).await;
            }
        },
    }
    Ok(())
}
