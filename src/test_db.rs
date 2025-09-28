// Simple test to verify database setup works
use crate::data::database::establish_connection;
use crate::data::repository::TestRepository;
use crate::data::db_models::*;

pub fn test_database_setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing database setup...");
    
    // Establish connection
    let connection = establish_connection()?;
    println!("✓ Database connection established");
    
    // Create repository
    let repo = TestRepository::new(connection);
    println!("✓ Repository created");
    
    // Test creating a motorcycle
    let new_motorcycle = NewMotorcycle {
        brand: "Honda".to_string(),
        model: "CBR250RR".to_string(),
        year: 2023,
        engine_cc: 250,
        vin: Some("TEST123456789".to_string()),
        license_plate: Some("B1234TEST".to_string()),
        notes: Some("Test motorcycle for database setup".to_string()),
    };
    
    match repo.create_motorcycle(new_motorcycle) {
        Ok(motorcycle) => {
            println!("✓ Motorcycle created with ID: {}", motorcycle.id);
            println!("  Brand: {}, Model: {}, Year: {}, Engine: {}cc", 
                motorcycle.brand, motorcycle.model, motorcycle.year, motorcycle.engine_cc);
        }
        Err(e) => {
            println!("✗ Failed to create motorcycle: {}", e);
            return Err(Box::new(e));
        }
    }
    
    // Test getting motorcycles
    match repo.get_motorcycles() {
        Ok(motorcycles) => {
            println!("✓ Retrieved {} motorcycles from database", motorcycles.len());
            for moto in motorcycles {
                println!("  - {} {} {} ({}cc)", moto.brand, moto.model, moto.year, moto.engine_cc);
            }
        }
        Err(e) => {
            println!("✗ Failed to get motorcycles: {}", e);
            return Err(Box::new(e));
        }
    }
    
    println!("✓ Database setup test completed successfully!");
    Ok(())
}