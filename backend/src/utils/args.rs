use crate::{database, utils::config, web};
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
    #[command(subcommand, about = "User commands", long_about = None)]
    User(User),
    #[command(subcommand, about = "Category commands", long_about = None)]
    Category(Category),
    #[command(subcommand, about = "Web commands", long_about = None)]
    Web(Web),
    #[command(subcommand, about = "Import a dataset", long_about = None)]
    Import(Import),
    #[command(subcommand, about = "Export a dataset", long_about = None)]
    Export(Export),
    #[command(subcommand, about = "Set an expense reset point.", long_about = None)]
    Reset(Reset),
    // #[command(subcommand, about = "Program utilities", long_about = None)]
    // Utils(Utils),
    // #[command(subcommand, about = "Expense commands", long_about = None)]
    // Expenses(Expenses),
    // #[command(subcommand, about = "CLI commands", long_about = None)]
    // Cli(Cli),
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
        #[arg(short, long)]
        description: String,
    },
    #[command(about = "Delete a category", long_about = None)]
    Delete {
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Debug, Subcommand)]
enum Web {
    #[command(about = "Start the web server", long_about = None)]
    Start,
}

#[derive(Debug, Subcommand)]
enum Import {
    #[command(about = "Import a csv dataset", long_about = None)]
    OldCsvFormat {
        #[arg(short, long)]
        path: String,
    },
}

#[derive(Debug, Subcommand)]
enum Export {
    #[command(about = "Export the database as a json.", long_about = None)]
    Json {
        #[arg(short, long)]
        path: String,
    },
}

#[derive(Debug, Subcommand)]
enum Reset {
    #[command(about = "This will add the current datetime into the database and can be queried by the website in order to only provide the relevant expenses in calculations.", long_about = None)]
    Expenses,
}

pub async fn run(config: config::Config, db_pool: &sqlx::PgPool) -> Result<()> {
    let entry_point = EntryPoint::parse();

    match &entry_point.command {

        Commands::User(user) => match user {
            User::List => {
                info!("Listing users");
                let users = database::user::get_users(&db_pool).await?;
                println!("=== Users:");
                for u in &users {
                    println!("  - {}", u);
                }
            }
            User::Create { name } => {
                info!("Creating user {}", name);
                database::user::insert_user(db_pool, name.to_string()).await?;
            }
            User::Delete { name } => {
                info!("Deleting user {}", name);
                let users = database::user::get_users(&db_pool).await?;
                let count = users.iter().filter(|&n| n.username() == name).count();
                let mut input = String::new();
                if count == 0 {
                    println!("User `{}` does not exist.", name);
                    return Ok(());
                } else if count == 1 {
                    println!("User `{}` will be deleted. [y/n]", name);
                    std::io::stdin().read_line(&mut input)?;
                    if input.trim() != "y" {
                        println!("Aborting.");
                        return Ok(());
                    }
                    let id = users.iter().find(|&n| n.username() == name).unwrap().id();
                    database::user::delete_user(db_pool, id).await?;
                } else {
                    println!("Multiple users with the name `{}` exist.", name);
                    println!("Please resolve in database manually.");
                    return Err(anyhow::anyhow!("Multiple users with the name `{}` exist.", name));
                }
            }
        },
        Commands::Category(category) => match category {
            Category::List => {
                info!("Listing categories");
                let categories = database::category::get_categories(&db_pool).await?;
                println!("=== Categories:");
                for c in &categories {
                    println!("  - {}", c);
                }
            }
            Category::Create { name, description } => {
                info!("Creating category {}", name);
                database::category::insert_category(db_pool, name.to_string(), description).await?;
            }
            Category::Delete { name } => {
                info!("Deleting category {}", name);
                let categories = database::category::get_categories(&db_pool).await?;
                let count = categories.iter().filter(|&n| n.name() == name).count();
                let mut input = String::new();
                if count == 0 {
                    println!("Category `{}` does not exist.", name);
                    return Ok(());
                } else if count == 1 {
                    println!("Category `{}` will be deleted. [y/n]", name);
                    std::io::stdin().read_line(&mut input)?;
                    if input.trim() != "y" {
                        println!("Aborting.");
                        return Ok(());
                    }
                    let id = categories.iter().find(|&n| n.name() == name).unwrap().id();
                    database::category::delete_category(db_pool, id).await?;
                } else {
                    println!("Multiple categories with the name `{}` exist.", name);
                    println!("Please resolve in database manually.");
                    return Err(anyhow::anyhow!("Multiple categories with the name `{}` exist.", name));
                }
            }
        },


        Commands::Web(web) => match web {
            Web::Start => {
                info!("Starting web server");
                web::run(config, db_pool.clone()).await;
            }
        },
        Commands::Import(import) => match import {
            Import::OldCsvFormat { path } => {
                info!("Importing dataset from {}", path);
                super::import::old_csv_format_import(db_pool, path).await?;
            }
        },
        Commands::Export(export) => match export {
            Export::Json { path } => {
                info!("Exporting dataset to {}", path);
                super::export::export_json(db_pool, path).await?;
            }
        }
        Commands::Reset(reset) => match reset {
            Reset::Expenses => {
                info!("Adding reset point");
                database::expense::insert_last_reset(db_pool).await?;
            }
        },
    }
    Ok(())
}


// Commands::Utils(utils) => match utils {
// // Utils::Initialize => {}
// _ => unreachable!(),
// },

// Commands::Expenses(expenses) => match expenses {
// Expenses::List => {
// info!("Listing expenses");
// unimplemented!()
// // let exps = expenses::list_all(&db).await?;
// // for (i, expense) in exps.iter().enumerate() {
// //     println!("Expense #{} ====================================", i + 1);
// //     println!("{expense}",);
// // }
// }
// },
// Commands::Cli(cli) => match cli {
// Cli::Start => {
// info!("Starting CLI");
// unimplemented!()
// // cli::run(&db_pool).await?;
// }
// },


/*

#[derive(Debug, Subcommand)]
enum Utils {
    // #[command(about = "Initialize the database", long_about = None)]
    // Initialize,
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

 */