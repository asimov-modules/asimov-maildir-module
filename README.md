# ASIMOV Maildir Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-maildir-module)](https://crates.io/crates/asimov-maildir-module)
[![Documentation](https://docs.rs/asimov-maildir-module/badge.svg)](https://docs.rs/asimov-maildir-module)

[ASIMOV] module for [Maildir] email import.

## ✨ Features

- Parses email messages from Maildir folders and outputs them as [JSON-LD].
- Constructs a semantic knowledge graph based on the [KNOW] ontology.
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with the [ASIMOV CLI]

```bash
asimov module install maildir -v
```

### Installation from Source Code

```bash
cargo install asimov-maildir-module
```

## 👉 Examples

### Email Import from a Maildir Folder

#### Cataloging email messages in the maildir

```bash
asimov-maildir-cataloger file:/path/to/maildir/
```

#### Exporting email messages in the maildir as JSON

```bash
asimov-maildir-cataloger file:/path/to/maildir/ -o json
```

#### Fetching a specific email message

```bash
asimov-maildir-fetcher file:/path/to/maildir/#mid
```

## ⚙ Configuration

This module requires no configuration.

## 📚 Reference

### `asimov-maildir-cataloger`

```
asimov-maildir-cataloger

Usage: asimov-maildir-cataloger [OPTIONS] <MAILDIR-FOLDER-URL>

Arguments:
  <MAILDIR-FOLDER-URL>  A `file:/path/to/maildir/` URL to the folder to catalog

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -n, --limit <COUNT>    Limit the number of messages to catalog
  -o, --output <FORMAT>  Set the output format [default: cli] [possible values: cli, json, jsonld, jsonl]
  -h, --help             Print help
```

### `asimov-maildir-fetcher`

```
asimov-maildir-fetcher

Usage: asimov-maildir-fetcher [OPTIONS] <MAILDIR-MESSAGE-URL>

Arguments:
  <MAILDIR-MESSAGE-URL>  A `file:/path/to/maildir/#mid` URL to the message to fetch

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -o, --output <FORMAT>  The output format
  -h, --help             Print help
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-maildir-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-maildir-module&text=asimov-maildir-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-maildir-module&title=asimov-maildir-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-maildir-module&t=asimov-maildir-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-maildir-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-maildir-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[Maildir]: https://en.wikipedia.org/wiki/Maildir
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Rust]: https://rust-lang.org
