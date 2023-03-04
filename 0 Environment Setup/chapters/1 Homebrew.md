# Homebrew - Package Manager

Homebrew is a package manager for macOS. It allows you to install and manage software packages from the command line.

## Installation

Navigate to https://brew.sh and copy the code into your terminal to install `Homebrew` package manager.

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Then add the `brew` command to your PATH:

```bash
(echo; echo 'eval "$(/opt/homebrew/bin/brew shellenv)"') >> /Users/freddy/.zprofile
eval "$(/opt/homebrew/bin/brew shellenv)"
```

## Details

`Homebrew` is a popular package manager for macOS that allows users to easily install and manage a variety of software packages, libraries, and utilities on their Mac computers.

With `Homebrew`, users can install, update, and uninstall software packages and dependencies from the command line, without the need for manual downloading, compiling, or configuration. `Homebrew` uses a formula system, where packages are defined as "formulae" in `Ruby` scripts, which are then downloaded, built, and installed from source code or precompiled binaries.

`Homebrew` also allows users to create and maintain their own formulae, contribute to the community-maintained repository of formulae, and use "casks" to install graphical applications and other binary packages. It provides a convenient and streamlined way for developers and users to manage the software dependencies and tools needed for their projects or workflows.
