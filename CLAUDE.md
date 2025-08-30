# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal website and technical blog repository built with multiple tools:
- **Zola**: Static site generator for the main website (`mysite/`) 
- **mdBook**: Documentation generator for multiple books (`books/` directory)

## Development Commands

### Main Website (Zola)
```bash
# Serve the website locally with auto-reload
cd mysite && zola serve -O

# Build the static site
cd mysite && zola build
```

### Books (mdBook)
Each book in the `books/` directory can be built independently:

```bash
# Build wjson book
mdbook build ./books/wjson/ -d $(pwd)/mysite/static/wjson --open

# Serve wjson book for development
mdbook serve -d $(pwd)/mysite/static/wjson --open ./books/wjson

# Build inside-duckdb book
mdbook build ./books/inside-duckdb/ -d $(pwd)/mysite/static/inside-duckdb --open

# Serve inside-duckdb book for development
mdbook serve -d $(pwd)/mysite/static/inside-duckdb --open ./books/inside-duckdb
```

## Architecture

### Repository Structure
- `mysite/`: Zola-based main website
  - `content/`: Markdown content for blog posts, docs, and monthly updates
    - `blog/`: Technical blog posts (2016-2025) 
    - `monthly/`: Monthly technical summaries
    - `docs/`: Documentation pages
  - `static/`: Static assets and generated books
  - `config.toml`: Zola configuration
- `books/`: Multiple mdBook projects
  - `wjson/`: JSON API for Scala documentation
  - `inside-duckdb/`: DuckDB source code analysis notes
  - `learning/`: Technical learning notes (C++, Rust, SIMD, LLVM)
  - `paper-readings/`: Academic paper reading notes
  - `simd/`, `olympics/`: Additional specialized books

### Content Organization
- Blog posts follow date-based naming: `YYYY-MM-DD-title/`
- Monthly summaries track technical developments and learnings
- Books are specialized technical references built as standalone mdBook projects
- All books are built into `mysite/static/` for unified hosting

### Languages and Topics
Primary focus areas include:
- Systems programming (Rust, C++, Zig)
- Database internals (DuckDB, DataFusion)
- SIMD and performance optimization
- Compiler technologies (LLVM, QBE)
- Web technologies and visualization

## Notes
- Chinese content mixed with English technical terms
- Books use mermaid diagrams (some have mermaid preprocessor configured)
- Static site is designed for GitHub Pages hosting
- No package.json or traditional JavaScript build setup