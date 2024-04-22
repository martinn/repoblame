use std::collections::HashMap;
use std::fmt;
use std::ops::AddAssign;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Author {
    pub email: String,
    // TODO: Add other details
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FileType(pub String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberOfLines(pub u32);

#[derive(Debug)]
pub struct AuthorStats {
    pub lines: NumberOfLines,
    pub file_types: HashMap<FileType, NumberOfLines>,
}

#[derive(Debug)]
pub struct RepoStats {
    pub author_stats: HashMap<Author, AuthorStats>,
    pub total_lines: NumberOfLines,
}

impl fmt::Display for NumberOfLines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AddAssign for NumberOfLines {
    fn add_assign(&mut self, other: Self) {
        *self = NumberOfLines(self.0 + other.0);
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl RepoStats {
    pub fn new() -> Self {
        Self {
            author_stats: HashMap::new(),
            total_lines: NumberOfLines(0),
        }
    }

    pub fn increment_lines(&mut self, author_email: &str, file_extension: Option<&str>) {
        let author = Author {
            email: author_email.to_string(),
        };
        let file_type = FileType(file_extension.unwrap_or("Unknown").to_string());

        let author_stats = self
            .author_stats
            .entry(author.clone())
            .or_insert(AuthorStats {
                lines: NumberOfLines(0),
                file_types: HashMap::new(),
            });
        author_stats.lines += NumberOfLines(1);

        *author_stats
            .file_types
            .entry(file_type.clone())
            .or_insert(NumberOfLines(0)) += NumberOfLines(1);

        self.total_lines += NumberOfLines(1);
    }

    pub fn sorted_authors(&self) -> Vec<(&Author, &AuthorStats)> {
        let mut sorted_authors: Vec<(&Author, &AuthorStats)> = self.author_stats.iter().collect();
        sorted_authors.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));
        sorted_authors
    }

    pub fn sorted_file_types_by_author(
        &self,
    ) -> HashMap<&Author, Vec<(&FileType, &NumberOfLines)>> {
        self.author_stats
            .iter()
            .map(|(author, author_blame)| {
                let mut file_types_vec: Vec<(&FileType, &NumberOfLines)> =
                    author_blame.file_types.iter().collect();
                file_types_vec.sort_by(|a, b| b.1.cmp(a.1));
                (author, file_types_vec)
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn sorted_authors_by_file_type(
        &self,
    ) -> HashMap<&FileType, Vec<(&Author, &NumberOfLines)>> {
        self.author_stats
            .iter()
            .flat_map(|(author, author_blame)| {
                author_blame
                    .file_types
                    .iter()
                    .map(move |(file_type, lines)| (file_type, (author, lines)))
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<&FileType, Vec<(&Author, &NumberOfLines)>>,
                 (file_type, (author, lines))| {
                    acc.entry(file_type).or_default().push((author, lines));
                    acc
                },
            )
            .iter()
            .map(|(file_type, author_lines)| {
                let mut sorted_author_lines: Vec<(&Author, &NumberOfLines)> = author_lines.to_vec();
                sorted_author_lines.sort_by(|a, b| b.1.cmp(a.1));
                (file_type, sorted_author_lines)
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<&FileType, Vec<(&Author, &NumberOfLines)>>,
                 (file_type, sorted_author_lines)| {
                    acc.entry(file_type)
                        .or_default()
                        .extend(sorted_author_lines);
                    acc
                },
            )
    }
}
