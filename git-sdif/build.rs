use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Only install completion during release builds or when explicitly requested
    let profile = env::var("PROFILE").unwrap_or_default();
    let install_completion = env::var("INSTALL_COMPLETION").is_ok() || profile == "release";
    
    if !install_completion {
        return;
    }

    // Get the completion script path (relative to this package directory)
    let completion_script = "git-sdif-completion.bash";
    
    // Check if completion script exists
    if !Path::new(completion_script).exists() {
        eprintln!("Warning: git-sdif-completion.bash not found, skipping completion installation");
        return;
    }

    // Try to install to user's bash completion directory
    if let Some(home) = env::var_os("HOME") {
        let completion_dir = Path::new(&home).join(".bash_completion.d");
        
        // Create completion directory if it doesn't exist
        if let Err(_) = fs::create_dir_all(&completion_dir) {
            eprintln!("Warning: Could not create ~/.bash_completion.d directory");
            return;
        }
        
        let target_path = completion_dir.join("git-sdif-completion.bash");
        
        // Copy completion script
        if let Err(e) = fs::copy(completion_script, &target_path) {
            eprintln!("Warning: Could not install completion script: {}", e);
            return;
        }
        
        println!("cargo:warning=Installed git-sdif completion to {}", target_path.display());
        
        // Check if bashrc needs updating
        let bashrc_path = Path::new(&home).join(".bashrc");
        if bashrc_path.exists() {
            if let Ok(bashrc_content) = fs::read_to_string(&bashrc_path) {
                if !bashrc_content.contains(".bash_completion.d") {
                    println!("cargo:warning=Add this to your ~/.bashrc to enable completions:");
                    println!("cargo:warning=");
                    println!("cargo:warning=# Load custom bash completions");
                    println!("cargo:warning=if [ -d ~/.bash_completion.d ]; then");
                    println!("cargo:warning=    for file in ~/.bash_completion.d/*; do");
                    println!("cargo:warning=        [ -r \"$file\" ] && source \"$file\"");
                    println!("cargo:warning=    done");
                    println!("cargo:warning=fi");
                }
            }
        }
    }
}
