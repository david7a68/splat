# Splat

A desktop foundation for D&D 2024 encounter planning and simulation.
Built with Rust and egui (eframe). The design system is in [docs/design-system.md](docs/design-system.md).
The sidebar has one flat level: **Run** (while a run is open), **Encounters**, **Sessions**, **Timeline**, **Party**,
**Library**, **Campaign**, and **Settings**. The campaign switcher is always above them. **Lookup** (Ctrl+K) searches every entry and opens
beside any page. While a run is open, other pages show a strip with the turn and **Next turn**. Each page keeps its
state, and the app reopens on the last page or the open run. Drag the edge of the sidebar, a page's list, or a panel to
change its width. The app keeps that width after a restart. Simulation is later work.

## Development

Install Nix and devenv, then run:

```sh
devenv shell
cargo run --locked
```

Alternatively, with direnv installed, run `direnv allow` once.

```sh
devenv shell check
devenv test
```

Rust 1.95.0 and the Rust overlay follow the other local projects.
`devenv.lock` pins the environment; `Cargo.lock` pins application dependencies.
The Linux shell includes the libraries needed for Wayland, X11, and OpenGL.
It selects Nix Mesa for EGL so the app also starts on non-NixOS hosts.

## Visual tests

`cargo test --locked` runs every usage scenario on the real app, headlessly and with no
GPU, and checks ergonomics and task costs against `src/visual_tests/baseline.json`. A
worse count fails the test, and a better one is printed. The tests use temporary folders and
synthetic data. Test builds have no route to your settings, character cache, keyring or
D&D Beyond.

```sh
devenv shell -- visual-test iter-0
devenv shell -- visual-test work --only run-lookup,party
```

This also writes to `artifacts/screenshots/iter-0/`: a PNG for each state at 960x640,
1200x800 and 1920x1080, `contact-sheet.png`, `ergonomics.json`, `ergonomics.md`,
`manifest.txt`, and one `video/<scenario>.mp4` per scenario. Rendering uses Mesa's
software Vulkan driver, so captures repeat between runs. Video and the contact sheet need
the host's `ffmpeg` and `montage`.

- `SPLAT_VISUAL_ENFORCE=strict` fails on any violation. `report` never fails on a check.
- `SPLAT_VISUAL_UPDATE_BASELINE=1` rewrites the baseline after a full run.
- `SPLAT_VISUAL_SCENARIOS=party,settings` runs only those scenarios. `visual-test <label> --only
  party,settings` is the same, and keeps the captures of the label's other scenarios.
- `SPLAT_VISUAL_VIDEO=0` turns video off for `visual-test`.

The `gallery` scenario captures every design token and component. `src/design/lint.rs` fails on raw
colours and text sizes outside `src/design/`, and `src/design/tokens.rs` tests WCAG contrast.

Scenario labels are at the top of each scenario in `src/visual_tests/scenarios.rs`, and the
ones they share are in `steps.rs`. Goals use the test IDs in `docs/ux-intent.md` section 8.

## Local layout

Code lives in `~/Projects/splat`. The vault link is
`~/Documents/vault/D20 D&D/00-09 System/09 Tools/splat`.
Campaign data remains in the vault. A read-only content verifier and the complete
[built-in SRD 5.2.1 source package](docs/srd-import.md) are available; campaign
package selection and local entry editing are available in **Campaign**. The
[execution proof](docs/execution-proof.md) runs converted SRD mechanics from Markdown.

The link is listed in Obsidian's excluded files. This reduces visibility in
search and related views but is not a complete indexing or filesystem exclusion.
Build artifacts stay in the code repository's ignored `target/` directory.

Keep D&D Beyond session cookies and tokens outside Git. Authentication is available
in Settings; character sheets with combat sync are available in **Party**. Characters now have persistent local copies in the shared creature schema.
The encounter builder uses these local character copies. Manual DM runs are available. Automatic combat resolution and simulation integration remain later work.

## D&D Beyond connection

Select **⚙ Settings** at the bottom left. Open the sign-in link and sign in
in your browser. In browser developer tools, open Application (or Storage),
select Cookies for dndbeyond.com, and copy the `CobaltSession` value. Paste
that value into Splat's masked field and select **Connect**.

Splat exchanges the cookie directly with
`https://auth-service.dndbeyond.com/v1/cobalt-token`. It uses HTTPS, disables
redirects, and keeps the bearer token in memory. With **Remember session in
desktop keyring** selected, it saves the cookie through Linux Secret Service
(for example, KWallet or GNOME Keyring). Startup restores that cookie and obtains
a fresh token. If the keyring is unavailable, you can still connect for the
current run; Splat reports that the cookie was not saved.

Tokens refresh automatically; a failed refresh clears the live connection and
displays an error. Disconnect clears local credentials and removes the saved
cookie. If removal fails, Splat reports it and lets you retry. It does not
invalidate the browser session. Network and keyring operations run in a worker;
credential writes and removals run in order.

This is an internal D&D Beyond endpoint, not a supported public API. The Party viewer fetches and caches character sheets. Campaign details are fetched into memory, with linked names cached in the campaign manifest. Authentication tests use a local mock server;
a successful live login requires a real cookie entered in the application.

## User settings and campaigns

The sidebar dropdown selects one active local campaign. Manage campaigns and
configure their D&D Beyond links in **Settings**. Each local campaign has a UUID,
name, and absolute folder path. Renaming or relocating it preserves its UUID.
Removing its entry does not delete files.

Settings uses `$XDG_CONFIG_HOME/splat/settings.json`, or
`~/.config/splat/settings.json` on Linux. Schema version 3 contains
`campaigns` and `active_campaign_id`. Each campaign has one optional
`dndbeyond_campaign_id`. The same D&D Beyond campaign cannot be linked to two
local campaigns. Select **Not linked** to remove a link, or choose another
campaign to replace it.

After connecting, expand **Add from D&D Beyond** and select a campaign. Its
name and ID fill the campaign form. Then choose its data folder with **Browse…**
or enter a path, and select **Open campaign**. No local campaign is required first.
You can also choose an existing folder and select **Open campaign** without connecting.

Splat creates or discovers the campaign manifest in that folder. It stores the
name and D&D Beyond link there, alongside existing custom paths and content
settings. Opening a folder preserves its UUID, settings, and notes. Existing
registrations load their manifest settings on startup or **Reload**. App settings
retain the folder registration and cached metadata. Credentials stay in the keyring.
The remote link uses `splat_dndbeyond_campaign_id` and
`splat_dndbeyond_campaign_name`; character cache storage is unchanged.

Versions 1 and 2 migrate old project records without changing UUIDs, names, paths,
or unique links. If an old record has multiple remote IDs, or two records share
one remote ID, loading reports the conflict and blocks writes. Keep at most one
unique remote ID per old entry in settings.json and reload. The original file
remains untouched until a successful edit saves version 3.

Settings save with atomic replacement. Invalid or unsupported files are preserved.
External edits detected before a save require a reload. Session credentials stay
in the desktop keyring and memory, never in settings JSON.

The linked campaign name refreshes once whenever its link control becomes visible,
including returning to Settings, scrolling it back into view, switching campaigns,
or reconnecting. UI repaints do not repeatedly request data. The last successful
name is cached as `splat_dndbeyond_campaign_name` and shown while offline or refreshing.
Failed requests retain the cache. Replacing or removing a link clears its name;
late responses cannot apply an old name to a new link.

## Sessions and the timeline

**Sessions** holds session notes and session templates, as Markdown in the campaign's session folder (`Sessions`
by default, and you can change it there). **Timeline** holds every encounter, rest, travel interval, day and other
major event in story order, in `Timeline.md` in the same folder. The list in its frontmatter is the record, and Splat
writes the body from it. An encounter run adds its own event when combat begins. New events and runs go to the newest
session unless you choose another on the Timeline page. **Notes** (Ctrl+J) opens that session's note beside any page.

Each event has a reference such as `[[Timeline#^t-1a2b3c4d5e|Rest at the mill]]`. Paste it in any note: it is a block
link in Obsidian, and Splat lists the notes that link to an event.

### Recording

**Record** on a session uses the host's recorder (`pw-record`, then `parecord`, then `arecord`) to write 16 kHz mono WAV
under `Sessions/Recordings/<session id>/`. Tell the table first. The recorder is a child process that stops if Splat
dies, and the file stays if it does.

### Processing a recording

Nothing leaves the computer until you select **Process** on a recording. Choose the services in **Settings**.

1. A **speech service** writes `<recording>.transcript.md`. The default is OpenAI's `gpt-transcribe`, and any service
   with the OpenAI `audio/transcriptions` API works. It needs a key, kept in the desktop keyring. Audio goes up in parts
   of at most 10 minutes (the limit is 25 MB), each cut at the quietest moment nearby, and each with the end of the last
   part as its `prompt`. Processing resumes at the next part after a failure.
2. An **LLM** goes over the transcript twice. The default is `gpt-6.1-sol` through the `codex` command, with your Codex
   sign-in, in a read-only sandbox. Anthropic and any OpenAI-compatible API are options. The Codex CLI takes no audio, so
   step 1 cannot use it.
   - The first pass cleans the raw words and marks events, in `<recording>.cleaned.md`.
   - The second pass checks that against the raw transcript and corrects it. It writes `<recording>.marked.md`, the result,
     `<recording>.review.md`, what it changed and could not verify, and `<recording>.summary.md`.

   Splat then checks the result itself: every time heading must be there, and it must not be far shorter than the raw
   words. The raw transcript never changes. A failed second pass keeps the first, and only the second runs again.

The default instructions for each pass, which you can edit in Settings, ask for one marker before each event:

```
<!-- splat:event kind="long_rest" at="01:12:00" title="Rest at the mill" place="Old mill" duration="8 hours" -->
```

A marker is a suggestion. The recording's row lists them, and only what you add goes on the timeline.

## Library workspace

Select **Library** to work on reusable packages without changing the active campaign.
The built-in D&D 2024 SRD, your packages, and the active campaign's own entries are one
catalogue. It opens on what this campaign can use, and the menu at the top shows everything,
one package, or the campaign's own entries. The built-in SRD is read-only. Browse Creatures,
Spells, Items, Character options, Rules, Glossary, and Encounters in the section
sidebar. Sections use subject metadata as well as entry kinds, so spell references
appear with spells. Other entries includes package notes and invalid notes for repair.
Search covers all sections by name, path, or section; clear it to return to the
selected section. Entries share stat cards, headings, and compact detail rows.
Structured details, engine support, and source identifiers remain expandable;
Markdown source and local editing remain available. Engine support is separate
from structural validity.

The built-in reader reflows printed PDF lines into paragraphs and keeps page
references under **Source and identifiers**. Raw definitions stay under collapsed
**Technical details**. Classes show separate traits, equipment choices, and
feature links in level order. Display changes do not alter saved content.

A D&D Beyond definition goes to the enabled package note that lists its ID in
`splat_dndbeyond_ids`. Other definitions go to the campaign's **Campaign Package** folder. A
definition keeps its place until you approve a move: **Campaign → Packages** lists what can move
when packages are added or removed, suggests notes with the same words, and adds the ID when you
apply a change. The sidebar shows how many changes wait. See
[Imported character content](docs/design/character-content.md).

Under **Packages → Manage packages…**, register an existing folder containing exactly one package
manifest, or enter a name, namespace (for example `my_homebrew`), and a new absolute
folder path to create a package. The parent folder must already exist. Package
registrations are stored in user settings; package notes remain in their own folders.
Unregistering a package leaves its files in place. Package and campaign folders must
be separate. Register My Homebrew in your preferred location for shared homebrew.

Local packages support new draft entries and Markdown editing. **Save** checks note
structure, keeps existing UUIDs and kinds, rejects duplicate identities, and checks
for external edits before replacing a file. Validation messages show unresolved
references; a package-only check can report references to other packages as unresolved.
Invalid entry notes remain available for repair. If the file changed on disk, the save stops
and offers **Overwrite with my draft**, **Show disk version**, and **Reload and lose my draft**.
**Discard changes** takes one press and offers **Undo discard** until the next edit. Changing
pages keeps the draft in memory.
Save before closing the app; drafts are not persisted.

In **Campaign → Packages**, enable a whole package or select individual
entries. Required dependencies are included separately with their reasons, and rule
groups include all members. Whole-package selections include future additions;
individual selections do not. **Save selections** writes `splat_packages` into the
campaign manifest while preserving unrelated fields, comments, and note text. A
campaign without a manifest gets `Campaign.md` on first save, with its existing UUID
and the default SRD selection. Missing packages and unresolved references remain visible.
A package that other entries need but that is not registered shows once, as **Package not
found**. The issue uses the package's name when its folder, with a `Package.md`, is beside a
registered package, and its namespace otherwise. An entry missing from a loaded package also
shows once, with the entries that need it.

Use **Copy to…** on a library or campaign entry to make an independent copy in a
registered package or campaign. Review the destination, name, retained dependencies,
and Markdown before **Create copy**. Copies get a new UUID, lose the original resource
key, and retain source attribution and optional origin identity. Existing references
are not rewritten. A copy into a shared package cannot depend on private campaign
entries; share or replace those dependencies first. The app checks source, dependency,
and destination changes before writing. Destination folders must already exist.

Clean destinations refresh after copying, and an open encounter draft sees the copy at once. **Campaign → Campaign entries** provides a
search, the shared reader and a Markdown editor for local entries, plus copying back to packages. Pending
campaign edits survive workspace switches and block sidebar campaign changes until
saved or discarded. Invalid campaign notes have a raw repair editor. Repair them before
saving campaign selections. Raw notes can also be edited externally; reload after external changes.

Moving entries between owners, automatic link repair, and running rules from the reader
remain outside this UI. New entries are drafts, not executable rules. D&D Beyond links
are in Settings and in **Campaign → Link and folder**.

## Character sheets

Open **Party** to read the linked D&D Beyond party. Select a character
and browse Overview, Actions, Spells, Equipment, and Features. **Refresh party**
gets the latest sheets. Computed totals come directly from D&D Beyond. Local
creature records remain available offline, and open sheets use the event stream
with polling fallback. No character changes are sent to D&D Beyond. See [viewer details and streaming findings](docs/character-viewer.md).

If you edit a generated campaign note, Splat keeps your edit and stops updating that character's notes. To
get updates again, move the note aside or use **Force read** on the warning. Force read reads the sheet again and
replaces each note that has edits. Your version stays beside the note in a `.splat-archive` file, and the page lists
those files. Personal notes carry over when Splat can still find them.

## Encounter builder

Open **Encounters**, select **New encounter**, and type a name.
Use **Add party**, or search the campaign's available creatures. Each result shows
the CR, HP and AC it has, and **Read** opens its stat block in Lookup, beside the draft.
Add or duplicate a creature for each participant, assign its side, and use the arrows
to change roster order. Optional starting HP and temporary HP apply to that
participant only. With a value off, the creature's starting state applies.
The default sides, party and enemies, are hostile to each other.

A roster slot can also build in changes to its creature: a display name, maximum HP,
AC, initiative bonus, attack bonus, damage bonus, speed, starting conditions, and
a note. The slot's row sets its side, and a note can also override it. Open them with the slot's **Edit** button. Each one appears under the slot with the
value it replaces, and nothing is written back to the creature, a package or a campaign
copy. An invalid change is refused with a message that names the slot. See
[Overrides](docs/design/campaign-schema.md#overrides).

**Save encounter** writes a Markdown template under `Encounters/` in the campaign
folder. Select a saved encounter to reopen it. Names can change without changing
file or participant identities. A new campaign gets a manifest on first save.
Existing notes and unexposed encounter data are retained. External file changes
block saving; discard the draft and reload to read them. Unsaved encounters
survive page changes and content refreshes, and block campaign switching, with a warning on app close.

Party entries are local character copies from the linked D&D Beyond campaign;
refresh **Party**, then **Reload** on Encounters to see new copies. This first
editor saves direct creature references in campaign-local templates. Shared
`party_slot` bindings still need the Markdown editor. Phases, assumptions,
completion rules, and advanced starting state also remain Markdown fields.
Saving a template does not execute rules or start a combat run.

### Encounter groups

An encounter group is a reusable, named set of creatures that you prepare once and add to a
running encounter when it arrives. It is a note with `splat_kind: encounter_group`, saved under
`Encounter Groups/` in the campaign folder, with its members and counts, a default side, and your
notes as its text. **New group** on Encounters opens its editor, which searches creatures as the
encounter editor does. Any creature from this campaign or a package it uses can be a member, and
each member can carry the same changes as a roster slot. An empty group or an unavailable
creature is refused with a message. See [the schema](docs/design/campaign-schema.md#encounter-groups).

### Running an encounter as DM

Choose **Run encounter** beside a saved encounter. The run takes
an independent copy of each creature's stats and starting state. Refreshing Party
or editing content does not change an existing run. Repeated creatures have
numbered names and separate HP, conditions, and notes.

Setup, live play and the ended view are separate. In setup, type initiative totals or
roll each participant using its supplied bonus. Rows hold still while you type, and
**Turn order** beside them shows the result, with arrows to resolve ties. Fill in
unknown HP values under **Record**, then choose **Begin combat**. **Next turn** advances through present participants and counts
rounds. Absent participants keep their state and initiative when they return.
Participants at zero HP stay in turn order until you mark them absent.

Click a row, type an amount, and press Enter to apply final damage. **Heal** and
**Temp HP** are beside it. **Conditions** lists the official conditions, one click
each. The side panel holds the stat block or sheet, **Record** (HP corrections, death
saves, notes, sync), **Log**, encounter **Notes**, and **Groups**. A spell named in a stat block opens
in Lookup. Damage uses temporary HP first; healing stops at maximum HP. You resolve
attacks, damage modifiers, condition effects, Concentration checks, death and recovery,
phases, and completion conditions. **Log** has a dice control that accepts expressions
such as `2d6 + 3`.

| Key | Command |
|---|---|
| `N` | Next turn |
| `Enter`, `Shift+Enter` | Damage or heal by the typed amount |
| `U`, `Ctrl+Z` | Undo |
| `↑`, `↓` | Select another row |
| `A` | Focus the amount |
| `C` | Conditions |
| `Ctrl+K` | Open or close Lookup. `Escape` also closes it |

**Groups** lists the campaign's encounter groups, with their members and total XP. Drag a group's
name onto the initiative list, or press its **Add** button. Type a total beside it to use that for every
member, as players give theirs. Otherwise the campaign's rule decides: one shared roll with the first
member's bonus (identical NPCs, or the encounter's initiative groups), a roll for each member
(individually), or the initiative its side already has (one group per side). This works in setup and in
combat, and the same group can be added again. Each member is a new participant at full HP, with its
own key and numbered name. In combat the group takes its place on that total and the current turn stays
current. It joins the saved run and the log, and **Undo** takes back the whole group until later
steps stand in the way.

The row and card of a creature with changes say `Changed`, and the card lists each base and changed
value. The run applies name, maximum HP, AC, initiative bonus, side and conditions. Attacks and speed
are stat block text, so their changes stay on the card and on the row during combat, for you to add. These changes are saved with the run, so editing the file later does not
change it.

The combat log records changes. **Undo** names the last action and restores up to 100
prior actions, including turn changes, dice rolls, and manual endings in the current session.
**End encounter** records a manual ending and your outcome. Runs save automatically
after changes; **Close run** asks once, then returns to Encounters. The app reopens an open
run after a restart, on the current turn, and turns its sync back on. **Saved runs** on Encounters,
grouped by encounter and with dates, has **Resume** for the others. Run changes never
write to templates or character copies.

### Run storage and campaign initiative

**Campaign → Rules and runs** sets the run folder. It defaults to `Encounter Runs`
inside the campaign, and accepts another relative folder or an absolute path.
Saved JSON files contain the run state, combat log, initiative rule, random state,
and frozen creature definitions. Saves use atomic replacement and detect external
edits. If the file changed, saving stops until you choose **Keep this run** or **Save as a new
run**. Other failed saves keep the current run in memory and show an error. Undo history
is kept for the current session only. Changing the folder does not move old runs.

The same settings select how the campaign rolls NPC initiative: in groups of
identical NPCs (the D&D default), individually, in one group per side, or in the
encounter's initiative groups. **Roll NPC initiative** in run setup uses that
choice and leaves player totals alone. An encounter group added during setup stays
one roll under the identical-NPC rule, and joins neither its like nor another addition. The arrow beside it uses another choice for
one roll, and the campaign's choice stays the default. An arrow choice for the
encounter's initiative groups shows only when an NPC has a group. Side and initiative groups
use the first NPC in roster order for their bonus and roll mode. Normal, advantage, and disadvantage rolls are
available; use disadvantage for surprise. Ties use the campaign's roster-order
setting or the DM/player ordering controls. NPC status can be changed before
combat begins. Run folder settings use `splat_run_storage.directory`; initiative
is a campaign-local rule selected through `splat_rule_choices`.

### D&D Beyond combat sync

Combat sync starts when Splat connects to D&D Beyond and the active campaign is linked. It then
runs in the background on every page. Splat reads the party's sheets when the campaign's event stream
reports a change, and reads each sheet about once a minute. Party's panel can turn off its own sync of
the selected character. The background reads continue.

An encounter has the option **Sync party with D&D Beyond**, on by default. It shows only in a linked
campaign, in the roster of the encounter editor. With the option on:

- The roster shows what the party has now: HP, temporary HP, AC, conditions and spell slots for each
  row, and the party's total, so that you can plan the encounter around them.
- The party starts the run with the values on their sheets. Starting HP overrides for those rows wait,
  unused, in case you turn the option off.
- The run starts sync by itself, and **your changes to HP, death saves and conditions go to the
  player's sheet on D&D Beyond** with no other step.

With the option off, the party starts from their saved copies and the starting values you set, and
nothing in that encounter reaches D&D Beyond unless you select **Sync party** in the run. Use it for a flashback, a training fight, or a test of an
idea. The note records `combat_sync: false`, and a note without it has sync on.

**Sync party** in a run starts or stops sync for every linked player, and **Record** has the panel for
one. A stop lasts for that run, and the run saves it. A run saved before this option does not start by
itself. A finished run does not sync, so a resumed one keeps its results.

If D&D Beyond reports that this session cannot edit a character, the row says `Sync read-only`. Your
changes for that character stay in the run and nothing is sent. Changes on the sheet still come in, and
if you both changed a value, you choose which to keep. Each synced row shows its state, and a conflict shows on the row and the strip without stopping
the table. Each connection first reads the live sheet. Saved runs do not automatically push old values.
The session must have edit permission for the character.

A value you recorded is not replaced without your say. If a session ends during a run, keep
recording: a row with a change that was not sent says `Not sent`, the others say `Sync stopped`, and
closing the run tells you who has unsent changes. When you connect again, sync starts again by itself
if **Sync party** was on. On a first read, by **Sync party**, after a reconnect or after a
restart, Splat compares each value with the last one D&D Beyond confirmed, or with what the participant
began the run with if there is none. A value you changed stays in the run, and if the sheet differs
that is a conflict, settled from the panel: use the sheet's value, or keep yours and send it. An open
conflict is found again after a close, an app close or a crash. A value you did not change takes
the sheet's.

While sync reads a sheet, that sheet decides Maximum HP, and the field says so. Stop sync to change it
in the run. A maximum set while sync was off stays until the next first read, where the notice and the
log say what it became. A message from sync stays under the list until you dismiss it.

A save from an earlier version can hold unsent values as its last match, so on its first read every
difference asks, once. Its new `started` field is optional, and earlier saves load. Spell slots and
limited uses are not in the saved run: an unsent one asks at the next first read in the same session,
and closing says it will be lost. Undo stops wherever a value reached the run from the sheet or the
sync panel, and the notice and the log say what changed, from what to what.

Sync covers current and temporary HP, death saves, official conditions,
ordinary and pact spell slots spent, and supported action/spell limited uses.
Concentration, custom effects, inventory charges, initiative, and notes stay local.
Changes are sent after a short pause; incoming values are checked every 15 seconds.
Each write is followed by a read to verify the result. Conflicts keep local edits
visible and offer a choice of local or remote values. Unconfirmed changes are not
retried without review. The service has no verified atomic compare-and-set API,
so another client can still change values between a read and a write.

Health or condition values taken from the sheet clear the run's undo history, and a message says so. In a small
window with a full roster, the list can leave no room for that message, so only the Log records the change. Wait for sync
or disable it before closing a run with pending changes. Disabling sync cancels
queued work but cannot recall a request already sent. A change made on Party is still sent after you
leave its page. Party sync stops only when the connection ends, and the sidebar says so until it starts
again. Run sync remains active while that run is open, until the encounter ends.

Transport and conflict handling are tested with simulated responses. Live writes
have not been verified with an authenticated D&D Beyond character. Endpoint
sources and limits are recorded in [the research notes](docs/research/dndbeyond-combat-writes.md).
