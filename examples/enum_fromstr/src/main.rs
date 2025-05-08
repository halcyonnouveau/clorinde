use enum_fromstr_codegen::{
    client::Params,
    queries::{
        get_all_items, get_items_by_color, get_items_by_status, create_item
    },
    types::{Color, Status},
};
use std::str::FromStr;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create database pool
    let pool = create_pool().await?;
    let client = pool.get().await?;

    println!("=== Using Enum FromStr Implementation ===\n");

    // Parse color from string using FromStr trait
    let color_str = "red";
    let color = Color::from_str(color_str)?;
    println!("Parsed color '{}' into {:?}", color_str, color);

    // Parse status from string using FromStr trait
    let status_str = "active";
    let status = Status::from_str(status_str)?;
    println!("Parsed status '{}' into {:?}", status_str, status);

    // Use parsed enum to filter query
    let items = get_items_by_color().bind(&client, &color).all().await?;
    println!("\nItems with color {color_str}:");
    for item in items {
        println!("- Item: {} (Color: {:?}, Status: {:?})", item.name, item.color, item.status);
    }

    // Try parsing an invalid color
    let invalid_color = "purple";
    match Color::from_str(invalid_color) {
        Ok(c) => println!("\nSuccessfully parsed '{invalid_color}' to {:?}", c),
        Err(e) => println!("\nError parsing '{invalid_color}': {}", e),
    }

    // Parse command line arguments example
    println!("\n=== Simulated Command Line Argument Example ===");
    // Simulate command line arguments for color and status
    let args = ["program", "--color", "blue", "--status", "pending"];
    
    // Parse flags
    let mut parsed_color = None;
    let mut parsed_status = None;
    
    for i in 1..args.len() {
        match args[i] {
            "--color" => {
                if i + 1 < args.len() {
                    parsed_color = Some(Color::from_str(args[i + 1]).unwrap_or_else(|e| {
                        eprintln!("Error parsing color: {}", e);
                        std::process::exit(1);
                    }));
                }
            },
            "--status" => {
                if i + 1 < args.len() {
                    parsed_status = Some(Status::from_str(args[i + 1]).unwrap_or_else(|e| {
                        eprintln!("Error parsing status: {}", e);
                        std::process::exit(1);
                    }));
                }
            },
            _ => {}
        }
    }
    
    // Query with the parsed arguments if available
    if let Some(color) = parsed_color {
        let items = get_items_by_color().bind(&client, &color).all().await?;
        println!("\nItems with command line specified color {:?}:", color);
        for item in items {
            println!("- Item: {} (Color: {:?}, Status: {:?})", item.name, item.color, item.status);
        }
    }
    
    if let Some(status) = parsed_status {
        let items = get_items_by_status().bind(&client, &status).all().await?;
        println!("\nItems with command line specified status {:?}:", status);
        for item in items {
            println!("- Item: {} (Color: {:?}, Status: {:?})", item.name, item.color, item.status);
        }
    }

    Ok(())
}

/// Connection pool configuration.
///
/// This is just a simple example config, please look at
/// `tokio_postgres` and `deadpool_postgres` for details.
use enum_fromstr_codegen::deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use enum_fromstr_codegen::postgres_types::NoTls;

async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5435);
    cfg.dbname = Some(String::from("postgres"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}