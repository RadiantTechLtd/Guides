# Git - Version Control

`Git` is a tool for tracking changes in any set of files, usually used to develop source code during software development.
It is used by both individuals and teams.

## Installation

This triggers `MacOS` to install the `Xcode Command Line Tools`, which includes `git`.

```bash
git --version
```

## Usage

Log in to your `GitHub` account:

```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@domain.com"
```

Clone a repository:

```bash
git clone https://github.com/RadiantTechLtd/Guides.git
```

Create a new branch:

```bash
git checkout -b new_branch
```

Add files to the staging area:

```bash
git add .
```

Commit changes:

```bash
git commit -m "Commit message"
```

Push changes to the remote repository:

```bash
git push origin new_branch
```

## Details

Version control is the management of changes to a file or set of files over time.
It is a system that allows developers to track and manage changes to their code, collaborate with others, and maintain a history of their work.

A version control system (VCS) allows developers to keep track of changes made to a file or set of files over time. It can track who made changes, what changes were made when they were made, and why they were made.
This is useful when multiple developers are working on the same codebase, or when a developer needs to make changes to an existing codebase without losing the original code.

There are several popular version control systems, including `Git`, `Subversion` (SVN), and `Mercurial`.
These systems typically offer features such as branching, merging, and tagging, which allow developers to work on different versions of the same codebase simultaneously, merge changes from different branches, and label specific versions for release.
