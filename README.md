# Repo Blame

Aggregate `git blame` stats across any git repository.

Showing top contributors by current lines of code, and their most popular file types. In contrast to most other tools which mainly work based on commits.

## Quick demo

Cropped output of `repoblame` on the [django](https://github.com/django/django) repo:

```console
❯ repoblame --path ~/Projects/django/
┌──────┬────────────────────────────────┬────────────────────────────────┬─────────────────────────┐
│ Repository path: /home/martin/Projects/django                                                    │
├──────┼────────────────────────────────┼────────────────────────────────┼─────────────────────────┤
│ #    │ Author email                   │ Total LoC (Percentage of repo) │ Top 5 file types by LoC │
├──────┼────────────────────────────────┼────────────────────────────────┼─────────────────────────┤
│ 1    │ claude@2xlibre.net             │ 200,197 (19%)                  │ po: 166,703 (83%)       │
│      │                                │                                │ py: 14,817 (7%)         │
│      │                                │                                │ mo: 10,107 (5%)         │
│      │                                │                                │ txt: 4,345 (2%)         │
│      │                                │                                │ js: 2,382 (1%)          │
├──────┼────────────────────────────────┼────────────────────────────────┼─────────────────────────┤
│ 2    │ jannis@leidel.info             │ 139,045 (13%)                  │ po: 121,794 (88%)       │
│      │                                │                                │ py: 6,947 (5%)          │
│      │                                │                                │ mo: 4,485 (3%)          │
│      │                                │                                │ txt: 4,310 (3%)         │
│      │                                │                                │ js: 923 (1%)            │
├──────┼────────────────────────────────┼────────────────────────────────┼─────────────────────────┤
│ 3    │ ops@djangoproject.com          │ 136,734 (13%)                  │ py: 131,051 (96%)       │
│      │                                │                                │ txt: 5,683 (4%)         │
├──────┼────────────────────────────────┼────────────────────────────────┼─────────────────────────┤
│ 4    │ felisiak.mariusz@gmail.com     │ 53,671 (5%)                    │ py: 18,154 (34%)        │
│      │                                │                                │ po: 18,074 (34%)        │
│      │                                │                                │ txt: 6,769 (13%)        │
│      │                                │                                │ mo: 3,867 (7%)          │
│      │                                │                                │ js: 2,505 (5%)          │
├──────┼────────────────────────────────┼────────────────────────────────┼─────────────────────────┤
│ 5    │ timograham@gmail.com           │ 44,246 (4%)                    │ txt: 19,584 (44%)       │
│      │                                │                                │ py: 16,694 (38%)        │
│      │                                │                                │ js: 6,378 (14%)         │
│      │                                │                                │ 1: 721 (2%)             │
│      │                                │                                │ json: 554 (1%)          │
├──────┼────────────────────────────────┼────────────────────────────────┼─────────────────────────┤
...
├──────┼────────────────────────────────┼────────────────────────────────┼─────────────────────────┤
│ 2677 │ jwhitlock@mozilla.com          │ 1 (0%)                         │ txt: 1 (100%)           │
├──────┼────────────────────────────────┼────────────────────────────────┼─────────────────────────┤
│ Total number of lines blamed: 1,066,074                                                          │
└──────┴────────────────────────────────┴────────────────────────────────┴─────────────────────────┘
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
  -p, --path <PATH>  Path to a git repository folder (specify a non-root folder if wanting to analyze a subfolder only)
  -h, --help         Print help
  -V, --version      Print version
```

## Disclaimer

I made this tool as a project for learning rust.

It's intended solely for providing information out of curiosity and should not be relied upon for any other purposes. Due to various factors, results are not a reliable indicator of actual top contributors to any project.

The stats provided are also devoid of any historical context; they reflect data at a specific point in time only.
