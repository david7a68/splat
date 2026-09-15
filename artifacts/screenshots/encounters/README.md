# Encounter builder and DM run review

Captured from the real Rust/egui app on an isolated X11 display, 2026-09-15.
All data is synthetic. The app used a temporary settings directory and no
D&D Beyond session or user campaign.

- `setup.png`: 1200 × 800 initiative preparation.
- `setup-minimum.png`: 960 × 640 after Roll NPC initiative.
- `combat-minimum.png`: 960 × 640 running combat and participant controls.
- `saved-runs.png`: saved in-progress run ready to resume.
- `settings-minimum.png`: run folder override and campaign initiative settings.

Review fixes: replaced unsupported arrow glyphs with Up/Down, separated participant
rows into groups, increased text contrast, shortened the save-location status
with the full path on hover, and kept the scrollbar visible. At minimum size,
turn controls remain visible, health controls fit, and lower details/log entries
can be reached by scrolling.

Live checks: resumed the synthetic run, rolled identical NPCs together without
changing the player's 17, began combat, applied 1 damage (temporary HP 3 to 2),
advanced to the next participant, and saved/closed. The JSON run file retained
the HP and current-turn changes. No template or source creature was changed.

The ignored `create_screenshot_fixture` test can create fresh sample settings:
set `SPLAT_UI_FIXTURE` to a temporary directory and run that test explicitly.
