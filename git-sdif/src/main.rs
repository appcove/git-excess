use clap::Parser;
use std::process::Command;

/// Compare two git branches or commits to see what the symmetric difference is.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The first branch to compare
    branch1: String,

    /// The second branch to compare
    #[clap(default_value = "HEAD")]
    branch2: String,
}

fn main(){
    let args = Args::parse();

    // Get the merge base between the two arguments 
    let merge_base_vec = Command::new("git")
        .arg("merge-base")
        .arg(&args.branch1)
        .arg(&args.branch2)
        .output()
        .expect("Failed to run git merge-base")
        .stdout
    ;

    // Decode the utf-8 string
    let merge_base = String::from_utf8(merge_base_vec).unwrap();
    let merge_base = merge_base.trim();

    match merge_base {
        "" => {
            eprintln!("No merge base found between branches:");
            eprintln!(" 1: {}", args.branch1);
            eprintln!(" 2: {}", args.branch2);
            std::process::exit(1);
        },
        _ => {
            println!("Found Merge base: {}", merge_base);
        },
    }

    println!("");
    println!("");
    println!("-------------------------------------------------------------------------------");
    println!("Commits unique to `{}`", args.branch1);
    println!("");
    
    // Show me commits on the first branch that are not on the second branch
    let _cmd1 = Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("--pretty=format:%Cgreen%h%Creset %Cred(%an)%Creset [%ad] %Cblue%s%Creset ")
        .arg(&args.branch1)
        .arg(format!("^{}", &args.branch2))
        .status()
    ;

    println!("");
    println!("");
    println!("-------------------------------------------------------------------------------");
    println!("Commits unique to `{}`", args.branch2);
    println!("");


    // Show me commits on the second branch that are not on the first branch
    let _cmd2 = Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("--pretty=format:%Cgreen%h%Creset %Cred(%an)%Creset [%ad] %Cblue%s%Creset ")
        .arg(&args.branch2)
        .arg(format!("^{}", &args.branch1))
        .status()
    ;

    println!("");
    println!("");
    println!("-------------------------------------------------------------------------------");
    println!("Common ancestor of `{}` and `{}`", args.branch1, args.branch2);
    println!("");


    // Show me the common commit
    let _cmd3 = Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("-1")
        .arg(&merge_base)
        .status()
    ;
}