# pg_vizurl

CLI frontend for https://explain.dalibo.com, a PostgreSQL execution plan visualizer.

With a query in your clipboard and `$DATABASE_URL` configured in your shell, run this command to open the visualization in a browser.

```
pbpaste | pg_vizurl
```

## Installation

### Install prebuilt binaries via shell script

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/jelder/pg_vizurl/releases/download/v0.1.1/pg_vizurl-installer.sh | sh
```

### Install prebuilt binaries via Homebrew

```sh
brew install jelder/tap/pg_vizurl
```