I wanted a better way to view KVs so this lets me download all of them (can be really really slow) or target a specific group based on pattern matching. 

What you'll need
- An API key with KV read permission
- Your KV namespace id
- Your cloudflare account ID

![Alt text](./assets/screenshot.png?raw=true "Title")

To download all kv's with a pattern match

```bash
cargo run kv-name-pattern
```

To download all kv's from your cloudflare

```bash
cargo run
```