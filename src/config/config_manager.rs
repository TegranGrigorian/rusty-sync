use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinioConfig {
    pub endpoint_url: String,
    pub access_key: String,
    pub secret_key: String,
    pub alias: Option<String>, // Optional alias for the server
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RustySyncConfig {
    pub minio_servers: Vec<MinioConfig>,
    pub current_server: Option<usize>, // Index of currently active server
}

impl RustySyncConfig {
    /// Get the config file path (in user's home directory)
    fn get_config_path() -> Result<PathBuf, String> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| "Could not find home directory".to_string())?;
        
        let config_dir = home_dir.join(".rusty-sync");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
        
        Ok(config_dir.join("config.json"))
    }

    /// Load configuration from file, or create default if not exists
    pub fn load() -> Result<Self, String> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let config_content = fs::read_to_string(&config_path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            
            let config: RustySyncConfig = serde_json::from_str(&config_content)
                .map_err(|e| format!("Failed to parse config file: {}", e))?;
            
            Ok(config)
        } else {
            // Create default config
            let default_config = RustySyncConfig::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::get_config_path()?;
        
        let config_json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(&config_path, config_json)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        
        Ok(())
    }

    /// Add a new MinIO server configuration
    pub fn add_server(&mut self, config: MinioConfig) -> Result<(), String> {
        self.minio_servers.push(config);
        self.save()?;
        Ok(())
    }

    /// Get the current active server configuration
    pub fn get_current_server(&self) -> Result<&MinioConfig, String> {
        if self.minio_servers.is_empty() {
            return Err("No MinIO servers configured. Run 'rusty-sync config' to add one.".to_string());
        }

        let index = self.current_server.unwrap_or(0);
        if index >= self.minio_servers.len() {
            return Err("Invalid server index in configuration".to_string());
        }

        Ok(&self.minio_servers[index])
    }

    /// Set the current active server
    pub fn set_current_server(&mut self, index: usize) -> Result<(), String> {
        if index >= self.minio_servers.len() {
            return Err(format!("Server index {} is out of range", index));
        }
        
        self.current_server = Some(index);
        self.save()?;
        Ok(())
    }

    /// Interactive configuration setup
    pub fn interactive_setup() -> Result<(), String> {
        println!("🔧 Rusty Sync Configuration Setup");
        println!("==================================");
        
        let mut config = Self::load()?;
        
        loop {
            println!("\nCurrent servers:");
            if config.minio_servers.is_empty() {
                println!("  (No servers configured)");
            } else {
                for (i, server) in config.minio_servers.iter().enumerate() {
                    let active = if Some(i) == config.current_server { " (active)" } else { "" };
                    let alias = server.alias.as_deref().unwrap_or("unnamed");
                    println!("  {}. {} - {}{}", i + 1, alias, server.endpoint_url, active);
                }
            }

            println!("\nOptions:");
            println!("  1. Add new MinIO server");
            println!("  2. Set active server");
            println!("  3. Remove server");
            println!("  4. Test connection");
            println!("  5. Exit");

            print!("\nChoose an option (1-5): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .map_err(|e| format!("Failed to read input: {}", e))?;

            match input.trim() {
                "1" => {
                    let server_config = Self::prompt_server_config()?;
                    config.add_server(server_config)?;
                    println!("✅ Server added successfully!");
                }
                "2" => {
                    if config.minio_servers.is_empty() {
                        println!("❌ No servers configured");
                        continue;
                    }
                    
                    print!("Enter server number (1-{}): ", config.minio_servers.len());
                    io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    
                    match input.trim().parse::<usize>() {
                        Ok(num) if num >= 1 && num <= config.minio_servers.len() => {
                            config.set_current_server(num - 1)?;
                            println!("✅ Active server set!");
                        }
                        _ => println!("❌ Invalid server number"),
                    }
                }
                "3" => {
                    if config.minio_servers.is_empty() {
                        println!("❌ No servers configured");
                        continue;
                    }
                    
                    print!("Enter server number to remove (1-{}): ", config.minio_servers.len());
                    io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    
                    match input.trim().parse::<usize>() {
                        Ok(num) if num >= 1 && num <= config.minio_servers.len() => {
                            config.minio_servers.remove(num - 1);
                            
                            // Reset current server if needed
                            if config.current_server.is_some() && config.current_server.unwrap() >= config.minio_servers.len() {
                                config.current_server = if config.minio_servers.is_empty() { None } else { Some(0) };
                            }
                            
                            config.save()?;
                            println!("✅ Server removed!");
                        }
                        _ => println!("❌ Invalid server number"),
                    }
                }
                "4" => {
                    match config.get_current_server() {
                        Ok(server) => {
                            println!("🔄 Testing connection to {}...", server.endpoint_url);
                            // We'll implement connection testing later
                            println!("✅ Connection test would be implemented here");
                        }
                        Err(e) => println!("❌ {}", e),
                    }
                }
                "5" => {
                    println!("👋 Configuration saved. Happy syncing!");
                    break;
                }
                _ => {
                    println!("❌ Invalid option. Please choose 1-5.");
                }
            }
        }

        Ok(())
    }

    /// Prompt user for server configuration details
    fn prompt_server_config() -> Result<MinioConfig, String> {
        println!("\n📝 Enter MinIO server details:");
        
        print!("Server alias (e.g., 'my-server'): ");
        io::stdout().flush().unwrap();
        let mut alias = String::new();
        io::stdin().read_line(&mut alias)
            .map_err(|e| format!("Failed to read alias: {}", e))?;
        let alias = alias.trim().to_string();

        print!("Endpoint URL (e.g., 'http://localhost:9000'): ");
        io::stdout().flush().unwrap();
        let mut endpoint = String::new();
        io::stdin().read_line(&mut endpoint)
            .map_err(|e| format!("Failed to read endpoint: {}", e))?;
        let endpoint = endpoint.trim().to_string();

        print!("Access Key: ");
        io::stdout().flush().unwrap();
        let mut access_key = String::new();
        io::stdin().read_line(&mut access_key)
            .map_err(|e| format!("Failed to read access key: {}", e))?;
        let access_key = access_key.trim().to_string();

        print!("Secret Key: ");
        io::stdout().flush().unwrap();
        let mut secret_key = String::new();
        io::stdin().read_line(&mut secret_key)
            .map_err(|e| format!("Failed to read secret key: {}", e))?;
        let secret_key = secret_key.trim().to_string();

        Ok(MinioConfig {
            endpoint_url: endpoint,
            access_key,
            secret_key,
            alias: if alias.is_empty() { None } else { Some(alias) },
        })
    }

    /// Create environment variables from current server config for MinIO utility
    pub fn export_to_env(&self) -> Result<(), String> {
        let server = self.get_current_server()?;
        
        unsafe {
            std::env::set_var("MINIO_ENDPOINT_URL", &server.endpoint_url);
            std::env::set_var("MINIO_ACCESS_KEY", &server.access_key);
            std::env::set_var("MINIO_SECRET_KEY", &server.secret_key);
        }
        
        Ok(())
    }
}
