# opsec-shield

Open source OPSEC tool for your own device. It is not an antivirus: it answers
"what is happening on my machine, who has access, and what has changed?".

## Layout

- `crates/core-device`: core logic (ports, connections, privileges, persistence).
- `crates/cli`: the `opsec` binary, a thin layer on top of the core.

## Roadmap

1. Workspace skeleton (done)
2. `opsec ports`
3. `opsec connections`
4. `opsec privileges` and `opsec startup`
5. `opsec snapshot` and `opsec diff`

## Usage

```powershell
cargo run -p opsec-cli
```

## License

AGPL-3.0-or-later. See [LICENSE](LICENSE).
