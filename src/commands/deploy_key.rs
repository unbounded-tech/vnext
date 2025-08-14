//! Deploy key command implementation

use crate::models::error::VNextError;
use crate::models::deploy_key::{DeployKeyResponse, DeployKeyList, SecretList, Secret};
use crate::services::git;
use crate::services::changelog;
use log::info;
use reqwest::blocking::Client;
use serde_json;
use std::fs::{self, create_dir_all};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Prompt user for input with a default value
fn prompt_with_default(prompt: &str, default: &str) -> Result<String, VNextError> {
    print!("{} [{}]: ", prompt, default);
    io::stdout().flush().map_err(|e| VNextError::Other(format!("Failed to flush stdout: {}", e)))?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| VNextError::Other(format!("Failed to read input: {}", e)))?;
    
    let input = input.trim();
    if input.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(input.to_string())
    }
}

/// Prompt user for confirmation (y/n)
fn prompt_for_confirmation(prompt: &str) -> Result<bool, VNextError> {
    print!("{} (y/n): ", prompt);
    io::stdout().flush().map_err(|e| VNextError::Other(format!("Failed to flush stdout: {}", e)))?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| VNextError::Other(format!("Failed to read input: {}", e)))?;
    
    let input = input.trim().to_lowercase();
    Ok(input == "y" || input == "yes")
}

/// Get the ID of a deploy key with the given name if it exists
fn get_deploy_key_id(
    owner: &str,
    repo_name: &str,
    key_name: &str,
) -> Result<Option<u64>, VNextError> {
    // First try using GitHub CLI
    let list_keys_cmd = format!(
        "gh api repos/{}/{}/keys",
        owner,
        repo_name
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&list_keys_cmd)
        .output();
        
    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                match serde_json::from_str::<DeployKeyList>(&stdout) {
                    Ok(keys) => {
                        // Check if any key has the given title
                        for key in keys.0 {
                            if key.title == key_name {
                                return Ok(Some(key.id));
                            }
                        }
                        Ok(None)
                    },
                    Err(e) => {
                        log::warn!("Failed to parse deploy keys response: {}", e);
                        Ok(None)
                    }
                }
            } else {
                log::warn!("Failed to list deploy keys: {}", String::from_utf8_lossy(&output.stderr));
                Ok(None)
            }
        },
        Err(e) => {
            log::warn!("Failed to execute gh api command: {}", e);
            Ok(None)
        }
    }
}

/// Delete a deploy key by ID
fn delete_deploy_key(
    owner: &str,
    repo_name: &str,
    key_id: u64,
) -> Result<(), VNextError> {
    info!("Deleting existing deploy key with ID: {}...", key_id);
    
    // Try using GitHub CLI first
    let delete_key_cmd = format!(
        "gh api -X DELETE repos/{}/{}/keys/{}",
        owner,
        repo_name,
        key_id
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&delete_key_cmd)
        .output();
        
    match output {
        Ok(output) => {
            if output.status.success() {
                info!("Successfully deleted deploy key with ID: {}", key_id);
                Ok(())
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                // If we get a 404, the key might have been deleted already
                if error.contains("404") {
                    info!("Deploy key with ID {} not found (may have been deleted already)", key_id);
                    Ok(())
                } else {
                    Err(VNextError::Other(format!("Failed to delete deploy key: {}", error)))
                }
            }
        },
        Err(e) => {
            Err(VNextError::Other(format!("Failed to execute delete command: {}", e)))
        }
    }
}

/// Check if a deploy key with the given name already exists in the repository
fn check_deploy_key_exists(
    owner: &str,
    repo_name: &str,
    key_name: &str,
) -> Result<bool, VNextError> {
    match get_deploy_key_id(owner, repo_name, key_name)? {
        Some(_) => Ok(true),
        None => Ok(false)
    }
}

/// Check if a secret with the given name already exists in the repository
fn check_secret_exists(
    owner: &str,
    repo_name: &str,
    secret_name: &str,
) -> Result<bool, VNextError> {
    // Try using GitHub CLI to list secrets
    let list_secrets_cmd = format!(
        "gh api repos/{}/{}/actions/secrets",
        owner,
        repo_name
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&list_secrets_cmd)
        .output();
        
    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                match serde_json::from_str::<SecretList>(&stdout) {
                    Ok(secrets) => {
                        // Check if any secret has the given name
                        for secret in secrets.secrets {
                            if secret.name == secret_name {
                                return Ok(true);
                            }
                        }
                        Ok(false)
                    },
                    Err(e) => {
                        log::warn!("Failed to parse secrets response: {}", e);
                        Ok(false)
                    }
                }
            } else {
                log::warn!("Failed to list secrets: {}", String::from_utf8_lossy(&output.stderr));
                Ok(false)
            }
        },
        Err(e) => {
            log::warn!("Failed to execute gh api command: {}", e);
            Ok(false)
        }
    }
}

/// Generate a deploy key for a GitHub repository
pub fn generate_deploy_key(
    owner: Option<String>,
    name: Option<String>,
    key_name: Option<String>,
    overwrite: bool,
) -> Result<(), VNextError> {
    // Try to detect current repository information
    let (detected_owner, detected_name) = match git::open_repository() {
        Ok(repo) => {
            let repo_info = changelog::get_repo_info(&repo);
            if repo_info.is_github_repo && !repo_info.owner.is_empty() && !repo_info.name.is_empty() {
                info!("Detected GitHub repository: {}/{}", repo_info.owner, repo_info.name);
                (Some(repo_info.owner), Some(repo_info.name))
            } else {
                (None, None)
            }
        },
        Err(_) => (None, None),
    };

    // Get repository owner
    let owner = match (owner, detected_owner) {
        (Some(o), _) => o,  // Use provided owner if specified
        (None, Some(detected)) => {
            // Ask if user wants to use detected owner
            if prompt_for_confirmation(&format!("Use detected repository owner '{}'?", detected))? {
                detected
            } else {
                // Prompt for owner
                prompt_with_default("Enter repository owner (e.g., unbounded-tech)", "")?
            }
        },
        (None, None) => {
            // Prompt for owner with no default
            print!("Enter repository owner (e.g., unbounded-tech): ");
            io::stdout().flush().map_err(|e| VNextError::Other(format!("Failed to flush stdout: {}", e)))?;
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| VNextError::Other(format!("Failed to read input: {}", e)))?;
            input.trim().to_string()
        }
    };

    // Get repository name
    let name = match (name, detected_name) {
        (Some(n), _) => n,  // Use provided name if specified
        (None, Some(detected)) => {
            // Ask if user wants to use detected name
            if prompt_for_confirmation(&format!("Use detected repository name '{}'?", detected))? {
                detected
            } else {
                // Prompt for name
                prompt_with_default("Enter repository name", "")?
            }
        },
        (None, None) => {
            // Prompt for name with no default
            print!("Enter repository name: ");
            io::stdout().flush().map_err(|e| VNextError::Other(format!("Failed to flush stdout: {}", e)))?;
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| VNextError::Other(format!("Failed to read input: {}", e)))?;
            input.trim().to_string()
        }
    };

    let key_name = key_name.unwrap_or_else(|| "DEPLOY_KEY".to_string());
    
    // Check if both deploy key and secret already exist
    let deploy_key_exists = check_deploy_key_exists(&owner, &name, &key_name)?;
    let secret_exists = check_secret_exists(&owner, &name, &key_name)?;
    
    // Determine if we should overwrite existing keys/secrets
    let mut should_overwrite = overwrite;
    
    if (deploy_key_exists || secret_exists) && !should_overwrite {
        // If either exists and overwrite wasn't specified, ask the user
        let prompt = format!(
            "Deploy key or secret '{}' already exists for repository {}/{}. Overwrite?", 
            key_name, owner, name
        );
        should_overwrite = prompt_for_confirmation(&prompt)?;
        
        if !should_overwrite {
            info!("Skipping creation as overwrite was not confirmed.");
            return Ok(());
        }
    }

    // Create .tmp directory if it doesn't exist
    let tmp_dir_path = Path::new(".tmp");
    if !tmp_dir_path.exists() {
        create_dir_all(tmp_dir_path).map_err(|e| VNextError::Other(format!("Failed to create .tmp directory: {}", e)))?;
    }

    let private_key_path = tmp_dir_path.join("deploy_key");
    let public_key_path = tmp_dir_path.join("deploy_key.pub");

    // Generate SSH key pair if it doesn't exist or we're overwriting
    if !private_key_path.exists() || should_overwrite {
        // Generate SSH key pair using ssh-keygen
        info!("Generating SSH key pair...");
        let keygen_output = Command::new("ssh-keygen")
            .arg("-t")
            .arg("ed25519")
            .arg("-f")
            .arg(&private_key_path)
            .arg("-N")
            .arg("")
            .arg("-q")
            .output()
            .map_err(|e| VNextError::Other(format!("Failed to execute ssh-keygen: {}", e)))?;

        if !keygen_output.status.success() {
            let stderr = String::from_utf8_lossy(&keygen_output.stderr);
            let stdout = String::from_utf8_lossy(&keygen_output.stdout);
            
            // Combine stdout and stderr for a more complete error message
            let error_msg = if stderr.trim().is_empty() {
                if stdout.trim().is_empty() {
                    "Unknown error (no output from ssh-keygen)".to_string()
                } else {
                    format!("Output: {}", stdout.trim())
                }
            } else {
                format!("Error: {}", stderr.trim())
            };
            
            return Err(VNextError::Other(format!("Failed to generate SSH key: {}", error_msg)));
        }
        
        // Verify the key files were created
        if !private_key_path.exists() || !public_key_path.exists() {
            return Err(VNextError::Other(
                "SSH key files were not created. Please check if ssh-keygen is installed and working properly.".to_string()
            ));
        }
    } else {
        info!("Using existing SSH key pair...");
    }

    // Set GitHub secret with private key if it doesn't exist or we're overwriting
    if !secret_exists || should_overwrite {
        info!("Creating repository secret {}...", key_name);
        let secret_cmd = format!(
            "gh secret set \"{}\" --body \"$(cat {})\" --repo \"{}/{}\" --app actions",
            key_name,
            private_key_path.display(),
            owner,
            name
        );
        
        let secret_output = Command::new("sh")
            .arg("-c")
            .arg(&secret_cmd)
            .output()
            .map_err(|e| VNextError::Other(format!("Failed to execute gh secret set command: {}", e)))?;
        
        if !secret_output.status.success() {
            let error = String::from_utf8_lossy(&secret_output.stderr);
            return Err(VNextError::Other(format!("Failed to set GitHub secret: {}", error)));
        }
        info!("Repository secret created successfully.");
    } else {
        info!("Repository secret '{}' already exists. Skipping creation.", key_name);
    }
    
    // Add public key as deploy key if it doesn't exist or we're overwriting
    if !deploy_key_exists || should_overwrite {
        // If we're overwriting and the key exists, delete it first
        if should_overwrite && deploy_key_exists {
            if let Some(key_id) = get_deploy_key_id(&owner, &name, &key_name)? {
                delete_deploy_key(&owner, &name, key_id)?;
            }
        }
        
        info!("Adding deploy key to the repository...");
        let public_key_content = fs::read_to_string(&public_key_path)
            .map_err(|e| VNextError::Other(format!("Failed to read public key: {}", e)))?;
        
        // Check for GITHUB_TOKEN environment variable
        let token = match std::env::var("GITHUB_TOKEN") {
            Ok(t) => t,
            Err(_) => {
                // Use gh api command if GITHUB_TOKEN is not available
                let deploy_key_cmd = format!(
                    "gh api repos/{}/{}/keys --field title=\"{}\" --field key=\"$(cat {})\"",
                    owner,
                    name,
                    key_name,
                    public_key_path.display()
                );
                
                let deploy_key_output = Command::new("sh")
                    .arg("-c")
                    .arg(&deploy_key_cmd)
                    .output()
                    .map_err(|e| VNextError::Other(format!("Failed to execute gh api command: {}", e)))?;
                
                if !deploy_key_output.status.success() {
                    let error = String::from_utf8_lossy(&deploy_key_output.stderr);
                    return Err(VNextError::Other(format!("Failed to add deploy key: {}", error)));
                }
                
                info!("Deploy key setup completed.");
                
                // Clean up
                fs::remove_file(&private_key_path)
                    .map_err(|e| VNextError::Other(format!("Failed to remove private key: {}", e)))?;
                
                fs::remove_file(&public_key_path)
                    .map_err(|e| VNextError::Other(format!("Failed to remove public key: {}", e)))?;
                
                return Ok(());
            }
        };
        
        // Use GitHub API directly if GITHUB_TOKEN is available
        let client = Client::new();
        let url = format!("https://api.github.com/repos/{}/{}/keys", owner, name);
        
        let response = client
            .post(&url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "vnext-cli")
            .json(&serde_json::json!({
                "title": key_name,
                "key": public_key_content.trim(),
                "read_only": true
            }))
            .send()
            .map_err(|e| VNextError::Other(format!("Failed to send request to GitHub API: {}", e)))?;
        
        if !response.status().is_success() {
            let error = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(VNextError::Other(format!("Failed to add deploy key: {}", error)));
        }
        
        let deploy_key: DeployKeyResponse = response.json()
            .map_err(|e| VNextError::Other(format!("Failed to parse response: {}", e)))?;
        
        info!("Deploy key setup completed.");
        info!("Deploy key ID: {}", deploy_key.id);
    } else {
        info!("Deploy key '{}' already exists. Skipping creation.", key_name);
    }
    
    // Clean up
    fs::remove_file(&private_key_path)
        .map_err(|e| VNextError::Other(format!("Failed to remove private key: {}", e)))?;
    
    fs::remove_file(&public_key_path)
        .map_err(|e| VNextError::Other(format!("Failed to remove public key: {}", e)))?;
    
    Ok(())
}