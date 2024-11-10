# pg_vizurl

CLI frontend for https://explain.dalibo.com, a PostgreSQL execution plan visualizer.

With a query in your clipboard and `$DATABASE_URL` configured in your shell, run this command to open the visualization in a browser.

```
pbpaste | pg_vizurl
```

## Installation

Homebrew:

```
brew tap jelder/homebrew-tap
brew install pg_vizurl
```

Cargo:

```
cargo binstall pg_vizurl
```