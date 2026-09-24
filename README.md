# Splat

A desktop foundation for D&D 2024 encounter planning and simulation.
Built with Rust and egui (eframe). The app has separate Library and Campaign workspaces. Library browsing and local
module authoring and campaign encounter planning are available. Simulation remains a placeholder.

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

## Local layout

Code lives in `~/Projects/splat`. The vault link is
`~/Documents/vault/D20 D&D/00-09 System/09 Tools/splat`.
Campaign data remains in the vault. A read-only content verifier and the complete
[built-in SRD 5.2.1 source package](docs/srd-import.md) are available; campaign
module selection and local entry editing are available in **Campaign → Content**. The
[execution proof](docs/execution-proof.md) runs converted SRD mechanics from Markdown.

The link is listed in Obsidian's excluded files. This reduces visibility in
search and related views but is not a complete indexing or filesystem exclusion.
Build artifacts stay in the code repository's ignored `target/` directory.

Keep D&D Beyond session cookies and tokens outside Git. Authentication is available
in Settings; character sheets with optional combat sync are available in Campaign → Party. Characters now have persistent local copies in the shared creature schema.
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

## Library workspace

Select **Library** to work on reusable modules without changing the active campaign.
The built-in D&D 2024 SRD appears as one read-only catalogue. Browse Creatures,
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

Under **Package actions → Manage packages**, register an existing folder containing exactly one package
manifest, or enter a name, namespace (for example `my_homebrew`), and a new absolute
folder path to create a module. The parent folder must already exist. Module
registrations are stored in user settings; module notes remain in their own folders.
Unregistering a module leaves its files in place. Module and campaign folders must
be separate. Register My Homebrew in your preferred location for shared homebrew.

Local modules support new draft entries and Markdown editing. **Save** checks note
structure, keeps existing UUIDs and kinds, rejects duplicate identities, and checks
for external edits before replacing a file. Validation messages show unresolved
references; a module-only check can report references to other modules as unresolved.
Invalid entry notes remain available for repair. **Discard changes**, then **Reload**,
to load external edits. Switching to Campaign or Settings keeps the draft in memory.
Save before closing the app; drafts are not persisted.

In **Campaign → Content → Modules**, enable a whole module or select individual
entries. Required dependencies are included separately with their reasons, and rule
groups include all members. Whole-module selections include future additions;
individual selections do not. **Save selections** writes `splat_packages` into the
campaign manifest while preserving unrelated fields, comments, and note text. A
campaign without a manifest gets `Campaign.md` on first save, with its existing UUID
and the default SRD selection. Missing modules and unresolved references remain visible.

Use **Copy to…** on a library or campaign entry to make an independent copy in a
registered module or campaign. Review the destination, name, retained dependencies,
and Markdown before **Create copy**. Copies get a new UUID, lose the original resource
key, and retain source attribution and optional origin identity. Existing references
are not rewritten. A copy into a shared module cannot depend on private campaign
entries; share or replace those dependencies first. The app checks source, dependency,
and destination changes before writing. Destination folders must already exist.

Clean destination workspaces refresh after copying; reload after saving any pending draft. **Campaign entries** provides a
reader and Markdown editor for local entries, plus copying back to modules. Pending
campaign edits survive workspace switches and block sidebar campaign changes until
saved or discarded. Invalid campaign notes have a raw repair editor. Repair them before
saving campaign selections. Raw notes can also be edited externally; reload after external changes.

Moving entries between owners, automatic link repair, and running rules from the reader
remain outside this UI. New entries are drafts, not executable rules. D&D Beyond links
remain in Settings.

## Character sheets

Open **Campaign → Party** to read the linked D&D Beyond party. Select a character
and browse Overview, Actions, Spells, Equipment, and Features. **Refresh party**
gets the latest sheets. Computed totals come directly from D&D Beyond. Local
creature records remain available offline, and open sheets use the event stream
with polling fallback. No character changes are sent to D&D Beyond. See [viewer details and streaming findings](docs/character-viewer.md).

## Encounter builder

Open **Campaign → Encounters**, select **New encounter**, and enter a name.
Add cached party members or search the campaign's available creatures. Add or
duplicate a creature for each participant, assign its side, and use **Move up**
to change roster order. Optional starting HP and temporary HP apply to that
participant only. With an override off, the creature's starting state applies.
The default sides, party and enemies, are hostile to each other.

**Save encounter** writes a Markdown template under `Encounters/` in the campaign
folder. Select a saved encounter to reopen it. Names can change without changing
file or participant identities. A new campaign gets a manifest on first save.
Existing notes and unexposed encounter data are retained. External file changes
block saving; discard the draft and reload to read them. Unsaved encounters
survive tab changes and block campaign switching, with a warning on app close.

Party entries are local character copies from the linked D&D Beyond campaign;
refresh **Party**, then **Reload** in Encounters to see new copies. This first
editor saves direct creature references in campaign-local templates. Shared
`party_slot` bindings still need the Markdown editor. Phases, assumptions,
completion rules, and advanced starting state also remain Markdown fields.
Saving a template does not execute rules or start a combat run.

### Running an encounter as DM

Save and select an encounter, then choose **Run encounter as DM**. The run takes
an independent copy of each creature's stats and starting state. Refreshing Party
or editing content does not change an existing run. Repeated creatures have
numbered names and separate HP, conditions, and notes.

Enter initiative totals or roll each participant using its supplied bonus.
Use matching totals for groups and the arrow buttons to resolve ties. Fill in
unknown HP values under **Adjust HP and initiative bonus**, then choose
**Begin combat**. **Next turn** advances through present participants and counts
rounds. Absent participants keep their state and initiative when they return.
Participants at zero HP stay in turn order until you mark them absent.

Select a participant to apply final damage, heal, replace temporary HP, record
conditions or Concentration, adjust death saves, and add DM notes. Damage uses
temporary HP first; healing stops at maximum HP. You resolve attacks, damage
modifiers, condition effects, Concentration checks, death and recovery, phases,
and completion conditions. Creature details and encounter notes remain available
for reference. The dice control accepts expressions such as `2d6 + 3`.

The combat log records changes. **Undo** restores up to 100 prior actions,
including turn changes, dice rolls, and manual endings in the current session.
**End encounter** records a manual ending and your outcome. Runs save automatically
after changes; **Save and close run** returns to the builder. Open **Saved runs**
and choose **Resume** to continue after restarting the app. Run changes never
write to templates or character copies.

### Run storage and campaign initiative

**Campaign run settings** sets the run folder. It defaults to `Encounter Runs`
inside the campaign, and accepts another relative folder or an absolute path.
Saved JSON files contain the run state, combat log, initiative rule, random state,
and frozen creature definitions. Saves use atomic replacement and detect external
edits. Failed saves keep the current run in memory and show an error. Undo history
is kept for the current session only. Changing the folder does not move old runs.

The same settings select the campaign initiative rule. **Roll NPC initiative**
uses that rule and leaves player totals alone. Choices are identical NPC groups
(the D&D default), individual NPCs, one group per side, or explicit encounter
initiative groups. Side and explicit groups use the first NPC in roster order
for their bonus and roll mode. Normal, advantage, and disadvantage rolls are
available; use disadvantage for surprise. Ties use the campaign's roster-order
setting or the DM/player ordering controls. NPC status can be changed before
combat begins. Run folder settings use `splat_run_storage.directory`; initiative
is a campaign-local rule selected through `splat_rule_choices`.

### D&D Beyond combat sync

Enable **Combat stats · two-way sync** in Party, or **D&D Beyond combat sync**
for a linked player in a DM run. Each connection first reads the live sheet.
Saved runs do not automatically push old values. The session must have edit
permission for the character.

Sync covers current and temporary HP, death saves, official conditions,
ordinary and pact spell slots spent, and supported action/spell limited uses.
Concentration, custom effects, inventory charges, initiative, and notes stay local.
Changes are sent after a short pause; incoming values are checked every 15 seconds.
Each write is followed by a read to verify the result. Conflicts keep local edits
visible and offer a choice of local or remote values. Unconfirmed changes are not
retried without review. The service has no verified atomic compare-and-set API,
so another client can still change values between a read and a write.

Incoming health or condition changes clear the run's undo history. Wait for sync
or disable it before closing a run with pending changes. Disabling sync cancels
queued work but cannot recall a request already sent. Party sync stops when its
view closes; run sync remains active while that run is open.

Transport and conflict handling are tested with simulated responses. Live writes
have not been verified with an authenticated D&D Beyond character. Endpoint
sources and limits are recorded in [the research notes](docs/research/dndbeyond-combat-writes.md).
