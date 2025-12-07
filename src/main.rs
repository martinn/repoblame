use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use tabled::{settings::Style, Table};

mod git;
mod output;
mod stats;

/// Aggregate git blame stats across any git repository.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RepoBlameArgs {
    /// Path to a git repository folder (specify a non-root folder if wanting to analyze a
    /// subfolder only).
    #[arg(short, long)]
    path: Option<std::path::PathBuf>,

    /// Include binary files in the blame stats (excluded by default)
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    include_binary: bool,

    /// Optional list of file extension(s) to exclude from the blame stats.
    /// Example: --exclude-by-extension md txt
    #[arg(short, long, num_args(1..))]
    exclude_by_extension: Option<Vec<String>>,

    #[arg(short, long, value_enum, default_value_t)]
    format: OutputFormat,
}

#[derive(clap::ValueEnum, Default, Clone)]
enum OutputFormat {
    /// Whitespace-delimited table
    Plain,
    /// Table with lines delimiting rows, columns, and headers
    #[default]
    Table,
    /// Pretty-printed JSON
    Json,
}

fn main() {
    // Handle SIGPIPE
    // https://stackoverflow.com/questions/65755853/simple-word-count-rust-program-outputs-valid-stdout-but-panicks-when-piped-to-he/65760807
    sigpipe::reset();

    let args = RepoBlameArgs::parse();

    let binding = args.path.unwrap_or(PathBuf::from("."));
    let exclude_binary = !args.include_binary;

    let exclude_by_type = args.exclude_by_extension.unwrap_or_default();
    let repo_path = binding.as_path();

    let mut repo_stats = stats::RepoStats::new();
    let mut git_tree = git::GitTree::new(repo_path);

    git_tree.iter().for_each(|file_path| {
        // Clear and print the current file being processed
        eprint!("\r\x1B[2K");
        eprint!("\r {}", file_path);
        std::io::stdout().flush().unwrap();

        let file_path = Path::new(&file_path);
        let file_extension = file_path.extension().and_then(|ext| ext.to_str());

        if exclude_binary {
            let mut full_path = PathBuf::from(repo_path);
            full_path.push(file_path);
            match is_binary_file(full_path.as_path()) {
                Ok(true) => return,
                Ok(false) => (),
                Err(..) => (),
            }
        }

        if !exclude_by_type.is_empty()
            && exclude_by_type.contains(&file_extension.unwrap_or_default().to_string())
        {
            return;
        }

        let mut git_blame = git::GitBlame::new(repo_path, file_path);
        git_blame
            .iter()
            .filter_map(|line| parse_email(&line))
            .for_each(|author_email| {
                repo_stats.increment_lines(&author_email, file_extension);
            });
    });
    // Clear the line after processing all files
    eprint!("\r\x1B[2K");

    let sorted_authors = repo_stats.sorted_authors();
    let sorted_file_types_by_author = repo_stats.sorted_file_types_by_author();

    let output = output::Output::new(
        repo_path,
        &repo_stats,
        &sorted_authors,
        &sorted_file_types_by_author,
    );
    match args.format {
        OutputFormat::Table => {
            let mut table = Table::from(&output);
            table.with(Style::modern());
            println!("{}", table);
        }
        OutputFormat::Plain => {
            let mut table = Table::from(&output);
            table.with(Style::blank());
            println!("{}", table);
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&output).unwrap();
            println!("{}", json);
        }
    };
}

fn is_binary_file(file_path: &Path) -> Result<bool, std::io::Error> {
    let file = File::open(file_path)?;
    let mut buffer: Vec<u8> = vec![];
    file.take(1024_u64).read_to_end(&mut buffer)?;

    Ok(content_inspector::inspect(&buffer).is_binary())
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
