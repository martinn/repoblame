# Repo Blame

Aggregate `git blame` stats across any git repository.

Showing top contributors by current lines of code, and their most popular file types. In contrast to most other tools which mainly work based on commits.

## Quick demo

Cropped output of `repoblame` on the [django](https://github.com/django/django) repo:

```console
❯ repoblame --path ~/Projects/django/ --exclude-by-extension po mo lock txt
┌──────┬────────────────────────────────┬────────────────────────┬─────────────────────────┐
│ Repository path: ~/Projects/django                                                       │
├──────┼────────────────────────────────┼────────────────────────┼─────────────────────────┤
│ #    │ Author email                   │ Total LoC (Percentage) │ Top 5 file types by LoC │
├──────┼────────────────────────────────┼────────────────────────┼─────────────────────────┤
│ 1    │ ops@djangoproject.com          │ 130,636 (25%)          │ py: 130,636 (100%)      │
├──────┼────────────────────────────────┼────────────────────────┼─────────────────────────┤
│ 2    │ timograham@gmail.com           │ 24,685 (5%)            │ py: 16,725 (68%)        │
│      │                                │                        │ js: 6,378 (26%)         │
│      │                                │                        │ 1: 721 (3%)             │
│      │                                │                        │ json: 554 (2%)          │
│      │                                │                        │ Unknown: 127 (1%)       │
├──────┼────────────────────────────────┼────────────────────────┼─────────────────────────┤
│ 3    │ russell@keith-magee.com        │ 23,574 (4%)            │ py: 22,724 (96%)        │
│      │                                │                        │ json: 424 (2%)          │
│      │                                │                        │ xml: 164 (1%)           │
│      │                                │                        │ Unknown: 123 (1%)       │
│      │                                │                        │ html: 120 (1%)          │
├──────┼────────────────────────────────┼────────────────────────┼─────────────────────────┤
│ 4    │ felisiak.mariusz@gmail.com     │ 21,723 (4%)            │ py: 18,222 (84%)        │
│      │                                │                        │ js: 2,505 (12%)         │
│      │                                │                        │ svg: 281 (1%)           │
│      │                                │                        │ yml: 205 (1%)           │
│      │                                │                        │ 1: 199 (1%)             │
├──────┼────────────────────────────────┼────────────────────────┼─────────────────────────┤
│ 5    │ claude@2xlibre.net             │ 18,993 (4%)            │ py: 14,793 (78%)        │
│      │                                │                        │ js: 2,361 (12%)         │
│      │                                │                        │ html: 1,494 (8%)        │
│      │                                │                        │ json: 144 (1%)          │
│      │                                │                        │ toml: 64 (0%)           │
├──────┼────────────────────────────────┼────────────────────────┼─────────────────────────┤
...
├──────┼────────────────────────────────┼────────────────────────┼─────────────────────────┤
│ 2092 │ ccsapp.tux@gmail.com           │ 1 (0%)                 │ py: 1 (100%)            │
├──────┼────────────────────────────────┼────────────────────────┼─────────────────────────┤
│ Total number of lines blamed: 525,678                                                    │
└──────┴────────────────────────────────┴────────────────────────┴─────────────────────────┘
```

## Features

- [x] Get aggregate blame stats across any git repo
- [x] Show top contributors by Lines of Code (LoC) and percentage (%) of repo LoC
- [x] Show authors top file types (file extension) by LoC
- [x] Support for `.git-blame-ignore-revs` (to ignore particular commits like formatting commits)
- [x] Support analyzing a subfolder of a repo only
- [x] Exclude binary files by default (and option to include them back in)
- [x] Option to exclude files by file type
- [ ] Automated tests
- [ ] Option to exclude specific file paths
- [ ] CI/CD and release packaging
- [ ] Progress report
- [ ] Concurrency and other performance optimizations (`git blame` on a big repo with lots of commits can get slow)
- [ ] Use `git2-rs` instead of `git` CLI (The initial version used `git2-rs` but performance was an order of magnitude worse for some reason)

## How to run it

At the moment this is not packaged anywhere, but you can checkout the repo and run it with `cargo`.

You will need to have [rust installed](https://www.rust-lang.org/tools/install).

To run `repoblame` on a git repo at `../some_path/` (assuming that path points to the root of a git repo):

```console
cargo run --release -- --path ../some_path/
```

You can also run it only against a particular sub-folder of a git repo:

```console
cargo run --release -- --path ../some_path/some_sub_path/
```

Full CLI options below:

```console
Aggregate git blame stats across any git repository

Usage: repoblame [OPTIONS]

Options:
  -p, --path <PATH>
          Path to a git repository folder (specify a non-root folder if wanting to analyze a subfolder only)
      --include-binary
          Include binary files in the blame stats (excluded by default)
  -e, --exclude-by-extension <EXCLUDE_BY_EXTENSION>...
          Optional list of file extension(s) to exclude from the blame stats. Example: --exclude-by-extension lock json
  -h, --help
          Print help
  -V, --version
          Print version
```

## Disclaimer

I made this tool as a project for learning rust.

It's intended solely for providing information out of curiosity and should not be relied upon for any other purposes. Due to various factors, results are not a reliable indicator of actual top contributors to any project.

The stats provided are also devoid of any historical context; they reflect data at a specific point in time only.
