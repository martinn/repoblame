use clap::Parser;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

mod git;
mod stats;
mod table;

/// Aggregate git blame stats for a whole repo.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RepoBlameArgs {
    /// Path to the git repository (default: current directory)
    #[arg(short, long)]
    path: Option<std::path::PathBuf>,
}

fn main() {
    let args = RepoBlameArgs::parse();

    let binding = args.path.unwrap_or(PathBuf::from("."));
    let repo_path = binding.as_path();

    let mut repo_stats = stats::RepoStats::new();
    let mut git_tree = git::GitTree::new(repo_path);

    git_tree.iter().for_each(|file_path| {
        // TODO: Fix overlap
        print!("\r {}", file_path);
        std::io::stdout().flush().unwrap();

        // TODO: Use file_format detection instead?
        // TODO: Skip binary and other non-text files?
        let file_path = Path::new(&file_path);
        let file_extension = file_path.extension().and_then(|ext| ext.to_str());
        let mut git_blame = git::GitBlame::new(repo_path, file_path);

        git_blame
            .iter()
            .filter_map(|line| parse_email(&line))
            .for_each(|author_email| {
                repo_stats.increment_lines(&author_email, file_extension);
            });
    });
    println!("\r");

    let sorted_authors = repo_stats.sorted_authors();
    let sorted_file_types_by_author = repo_stats.sorted_file_types_by_author();

    let table = table::TableDisplay::new(
        repo_path,
        &repo_stats,
        &sorted_authors,
        &sorted_file_types_by_author,
    );
    println!("{}", table);
}

fn parse_email(line: &str) -> Option<String> {
    let author_email = "author-mail";
    if line.contains(author_email) {
        let email = line
            .split(' ')
            .last()
            .unwrap()
            .trim_matches(&['<', '>'])
            .to_string();
        Some(email)
    } else {
        None
    }
}
