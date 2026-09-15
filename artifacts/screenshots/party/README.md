# Live character viewer screenshots

Captured from the running Rust/egui app on an isolated X11 display on 2026-09-15.
These use the configured Heralds of the End campaign and the existing desktop
keyring session. Unlike the sample content screenshots in the parent folder,
these images contain real party names and visible character information. They
were captured and passed to Claude Fable at the user's request. No raw API
responses or credentials were saved.

- `morana-overview.png`: calculated stats, saves, and skills.
- `rohan-spells.png`: spell list, slots, and basic spellcasting values.
- `tiny-equipment.png`: equipment and coin balances.
- `tiny-overview.png`: unarmored defense with shield and HP bonuses.
- `minimum-size.png`: viewer at its 960 × 640 minimum window size.

`claude-fable-review.md` is Claude Fable's review of the initial four images.
The screenshots above include the subsequent fixes. Applied findings: stronger
text contrast, ability modifier emphasis, proficiency and expertise labels,
three-column checks, basic initiative and passive perception, base speed, total
hit dice, death saves, class spell slots, basic spellcasting values, spell timing
and range metadata, filter hint and clear control, and a ticking fetch age.
The full character name “Rohan of Unc” was verified against the API; it is not
truncated.

Remaining suggestions include a party summary screen, more distinct navigation
styles, spell-level grouping, richer description formatting, and a denser header.
These are follow-ups rather than claimed completed work. Final rule calculations
and item/conditional adjustments need a dedicated character rules adapter.

`claude-fable-final-review.md` records a second review of the revised images.
It confirmed the main fixes and flagged the hidden scrollbar at minimum size.
The final viewer uses a solid, permanently visible scrollbar. The additional
`minimum-size-scrolled.png` confirms that the lower sections can be reached.
