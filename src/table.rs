use std::collections::HashMap;
use std::path::Path;

use tabled::settings::peaker::{PriorityMax, PriorityMin};
use tabled::settings::{Panel, Settings, Style, Width};
use tabled::{Table, Tabled};
use thousands::Separable;

use crate::stats;

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

impl TableRow {
    pub fn new(
        number: u32,
        author_email: &str,
        lines: u32,
        lines_percentage: u8,
        lines_by_file_type: &str,
    ) -> Self {
        Self {
            number,
            author_email: author_email.to_string(),
            lines: format!("{} ({}%)", lines.separate_with_commas(), lines_percentage),
            lines_by_file_type: lines_by_file_type.to_string(),
        }
    }
}

pub fn calculate_percentage(part: u32, total: u32) -> u8 {
    (part as f32 / total as f32 * 100.0).round() as u8
}

pub struct TableDisplay {
    pub table: Table,
}

impl TableDisplay {
    // TOOD: Decouple from stats module
    pub fn new(
        repo_path: &Path,
        repo_stats: &stats::RepoStats,
        sorted_authors: &[(&stats::Author, &stats::AuthorStats)],
        sorted_file_types_by_author: &HashMap<
            &stats::Author,
            Vec<(&stats::FileType, &stats::NumberOfLines)>,
        >,
    ) -> Self {
        let mut rows: Vec<TableRow> = vec![];
        for (mut i, (author, author_stats)) in sorted_authors.iter().enumerate() {
            i += 1;
            let lines_percentage =
                calculate_percentage(author_stats.lines.0, repo_stats.total_lines.0);
            let row = TableRow::new(
                i as u32,
                &author.email,
                author_stats.lines.0,
                lines_percentage,
                &sorted_file_types_by_author
                    .get(author)
                    .unwrap()
                    .iter()
                    .take(5)
                    .map(|(file_type, lines)| {
                        format!(
                            "{}: {} ({}%)",
                            file_type,
                            lines.separate_with_commas(),
                            calculate_percentage(lines.0, author_stats.lines.0)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
            rows.push(row);
        }

        let mut table = Table::new(&rows);
        table
            .with(Panel::header(format!(
                "Repository path: {}",
                repo_path.canonicalize().unwrap().display()
            )))
            .with(Panel::footer(format!(
                "Total number of lines blamed: {}",
                repo_stats.total_lines.separate_with_commas()
            )))
            .with(Style::modern())
            .with(Settings::new(
                Width::wrap(100).priority::<PriorityMax>(),
                Width::increase(100).priority::<PriorityMin>(),
            ));
        Self { table }
    }
}

impl std::fmt::Display for TableDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.table)
    }
}
