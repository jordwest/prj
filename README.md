# prj

`prj` is a command line tool that helps you jump between your local `git` repositories.

## Demo

[![screenshot of demo](./doc/asciinema-screen.png)](https://asciinema.org/a/296118)

## Features

- Fast fuzzy find your project thanks to [fuzzy-matcher](https://github.com/lotabout/fuzzy-matcher)
- Displays git information from each project:
  - Last commit summary
  - Currently checked out branch
  - Pending/uncommitted changes

## Installation

### MacOS (With Homebrew)

```sh
brew tap jordwest/homebrew-tools
brew install prj
```

### Other platforms

This hasn't yet been tested on other platforms, but it's theoretically compatible with Linux and Windows.

First install [Rust](https://www.rust-lang.org/tools/install), then run:

```
cargo install prj
```

## Setup

Set the root to search projects with:

```sh
prj configure
```

Running `prj list` will show the interactive search and send the selected project to `stdout`.

The recommended way to jump to projects is to add a function to your `.bashrc` or `.profile` to send the output of `prj list` to the `cd` command:

```sh
function p() {
	local dir
	dir=$(prj list) && cd $dir
}
```

Once you've added this function, reopen your terminal and run `p` from anywhere.

# Roadmap

## [v1.0 milestone](https://github.com/jordwest/prj/milestone/1)
