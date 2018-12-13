# Remote Storage server

## Usage

### Installation

```bash
sudo apt-get install libsqlite3-dev
```

### Initialize database

```bash
cargo install diesel_cli --no-default-features --features sqlite
echo "DATABASE_URL=test.db" > .env
diesel migration run
```

### Start

```bash
cargo run
```
