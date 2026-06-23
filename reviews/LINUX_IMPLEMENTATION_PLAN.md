# gnotes-linux — implementation checklist

Derived from code audit + fork plan. Full Nemotron review: `../gnotes/reviews/NEMOTRON_LINUX_NATIVE_REVIEW.md` (when complete).

## Done (2026-06-23)

- [x] Fork scaffold at `05_apps_and_extensions/gnotes-linux/`
- [x] GitHub repo: https://github.com/davidthegnomad/gnotes-linux
- [x] `src-tauri/src/window/mod.rs` — shared note window builder
- [x] Restore `float_on_top` on session startup (Rust + TS `useEffect`)
- [x] Logical position/size in `update_note` (HiDPI)
- [x] Linux tray: New Note + Quit (`tray-icon` feature)
- [x] Hide main controller window on Linux
- [x] Bundling enabled: AppImage + deb
- [x] `LINUX.md`, `UPSTREAM.md`, fork README
- [x] `nim-code-review.py --focus` flag for targeted NIM reviews

## Next (P1)

- [ ] Manual compositor test matrix on Nobara (fill table in `LINUX.md`)
- [ ] `cargo tauri build` release smoke test
- [ ] Tray tooltip/icon polish (PNG sizes for 22px panel)
- [ ] User-visible error when `setAlwaysOnTop` fails on Wayland
- [ ] Clamp restored window positions to visible monitor bounds

## Later (P2)

- [ ] Flatpak manifest + Flathub submission
- [ ] GitHub Actions `ubuntu-latest` CI build
- [ ] Single-instance plugin (relaunch focuses tray)
- [ ] Opaque fallback mode for broken transparency
- [ ] `.desktop` Categories and keywords review

## Upstream cherry-pick candidates

Watch `davidthegnomad/gnotes` for:

- Editor / TipTap fixes
- SQLite migration changes
- Generic security / capability updates

Do **not** merge macOS controller UX or iCloud work — see [UPSTREAM.md](./UPSTREAM.md).
