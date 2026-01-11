# A Threat Intelligence Platform Written in Rust

## WARNING
This is strictly a demo repository for me to learn and collaborate writing rust.

## Usage
Get Docker Desktop running locally, then from CLI:
```
docker run --name my-postgres \
  -e POSTGRES_PASSWORD=mysecretpassword \
  -p 5432:5432 \
  -v pgdata:/var/lib/postgresql/data \
  -d postgres
```
Replace ports, names, password as needed.

You'll then need a `.env` file at the top level of this repo that contains your DB details, looking something like this:
```
DATABASE_URL=postgres://UNAME:PASSWORD@localhost:5432/DB_NAME
```

Currently a `cargo run` will load an IP blocklist from Binary Defense; TODO - Build out a smaller, broader example load.

## TODO:
- [ ] Middlware layer to query DB in rust
- [ ] Layer to compare a PCAP from the rust-network-analyzer-tool with DB
- [ ] UI to drop PCAP & provide alert / review?
- 