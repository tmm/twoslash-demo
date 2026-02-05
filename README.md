# twoslash-demo

Demo crate for [rustdoc-twoslash](https://github.com/tmm/rustdoc-twoslash) — rustdoc with IDE-style type hover annotations on code blocks.

**Live demo**: https://twoslash-rustdoc.vercel.app

## Build & Deploy

Requires the [rustdoc-twoslash](https://github.com/tmm/rustdoc-twoslash) fork built locally.

```bash
# Build docs with twoslash annotations
RUSTDOC_TWOSLASH=1 \
  RUSTDOC=/path/to/rustdoc-twoslash/build/host/stage1/bin/rustdoc \
  RUSTC=/path/to/rustdoc-twoslash/build/host/stage1/bin/rustc \
  cargo doc --no-deps

# Copy built docs and push (auto-deploys to Vercel)
rsync -a --delete --exclude='.vercel' target/doc/ docs/
git add docs/ && git commit -m "docs: update" && git push
```

## Related

- [tmm/rustdoc-twoslash](https://github.com/tmm/rustdoc-twoslash) — modified `src/librustdoc/` files and patch
- [wevm/vocs/twoslash-rust](https://github.com/wevm/vocs/tree/next/twoslash-rust) — rust-analyzer integration library
