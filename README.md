# logshear

Fast log filtering and field-extraction tool with a minimal query DSL for structured and unstructured logs.

---

## Installation

**From source (requires Rust 1.70+):**

```bash
cargo install logshear
```

Or clone and build manually:

```bash
git clone https://github.com/yourname/logshear
cd logshear && cargo build --release
```

---

## Usage

```bash
# Filter lines where status is 500
logshear 'status == 500' app.log

# Extract specific fields from JSON logs
logshear --fields timestamp,level,message 'level == "error"' app.log

# Pipe from stdin
cat app.log | logshear 'method == "POST" and latency > 200'

# Match unstructured logs with a pattern
logshear 'message contains "timeout"' app.log
```

### Query DSL

| Operator | Example |
|----------|---------|
| `==` | `status == 404` |
| `!=` | `level != "debug"` |
| `>`, `<`, `>=`, `<=` | `latency > 500` |
| `contains` | `message contains "error"` |
| `and`, `or`, `not` | `status == 500 and method == "GET"` |

---

## Output

By default, logshear outputs matching log lines as-is. Use `--fields` to project specific fields into tab-separated or JSON output (`--format json`).

---

## License

MIT © 2024 — see [LICENSE](LICENSE) for details.