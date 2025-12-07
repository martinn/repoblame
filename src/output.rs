use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
};

use serde::Serialize;
use tabled::{
    settings::{
        peaker::{PriorityMax, PriorityMin},
        Panel,
        Settings,
        Width,
    },
    Table,
    Tabled,
};
use thousands::Separable;

use crate::stats::{self};

#[derive(Serialize)]
pub struct Output {
    repo_path: PathBuf,
    total_lines: u32,
    authors: Vec<AuthorStats>,
}

impl Output {
    pub fn new(
        repo_path: &Path,
        repo_stats: &stats::RepoStats,
        sorted_authors: &[(&stats::Author, &stats::AuthorStats)],
        sorted_file_types_by_author: &HashMap<
            &stats::Author,
            Vec<(&stats::FileType, &stats::NumberOfLines)>,
        >,
    ) -> Self {
        let authors = sorted_authors
            .iter()
            .enumerate()
            .map(|(i, (author, author_stats))| {
                let rank = (i + 1) as u32;
                let percentage =
                    calculate_percentage(author_stats.lines.0, repo_stats.total_lines.0);

                let top_file_types = sorted_file_types_by_author[author]
                    .iter()
                    .take(5)
                    .map(|(file_type, lines)| FileTypeStats {
                        file_type: file_type.to_string(),
                        lines: lines.0,
                        percentage: calculate_percentage(lines.0, author_stats.lines.0),
                    })
                    .collect();

                AuthorStats {
                    rank,
                    author_email: author.email.clone(),
                    total_lines: author_stats.lines.0,
                    percentage,
                    top_file_types,
                }
            })
            .collect();

        Self {
            repo_path: repo_path.canonicalize().unwrap(),
            total_lines: repo_stats.total_lines.0,
            authors,
        }
    }
}

impl From<&Output> for Table {
    fn from(value: &Output) -> Self {
        let rows: Vec<TableRow> = value
            .authors
            .iter()
            .map(<&AuthorStats as Into<TableRow>>::into)
            .collect();
        let mut table = Table::new(&rows);
        table
            .with(Panel::header(format!(
                "Repository path: {:?}",
                value.repo_path.display()
            )))
            .with(Panel::footer(format!(
                "Total number of lines blamed: {}",
                value.total_lines.separate_with_commas()
            )))
            .with(Settings::new(
                Width::wrap(100).priority::<PriorityMax>(),
                Width::increase(100).priority::<PriorityMin>(),
            ));
        table
    }
}

#[derive(Serialize)]
pub struct AuthorStats {
    rank: u32,
    author_email: String,
    total_lines: u32,
    percentage: u8,
    top_file_types: Vec<FileTypeStats>,
}

#[derive(Tabled)]
pub struct TableRow {
    #[tabled(rename = "#")]
    pub number: u32,

    #[tabled(rename = "Author email")]
    pub author_email: String,

    #[tabled(rename = "Total LoC (Percentage of repo)")]
    pub lines: String,

    #[tabled(rename = "Top 5 file types by LoC")]
    pub lines_by_file_type: String,
}

impl From<&AuthorStats> for TableRow {
    fn from(value: &AuthorStats) -> Self {
        Self {
            number: value.rank,
            author_email: value.author_email.clone(),
            lines: format!(
                "{} ({}%)",
                value.total_lines.separate_with_commas(),
                value.percentage
            ),
            lines_by_file_type: value
                .top_file_types
                .iter()
                .map(FileTypeStats::to_string)
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }
}

#[derive(Serialize)]
pub struct FileTypeStats {
    file_type: String,
    lines: u32,
    percentage: u8, // percentage of author's total lines
}

impl Display for FileTypeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} ({}%)",
            self.file_type,
            self.lines.separate_with_commas(),
            self.percentage
        )
    }
}

pub fn calculate_percentage(part: u32, total: u32) -> u8 {
    (part as f32 / total as f32 * 100.0).round() as u8
}
