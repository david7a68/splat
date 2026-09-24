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

This is an internal D&D Beyond endpoint, not a supported public API. The Party viewer fetches and caches character sheets. Campaign names and links are stored in the campaign folder. Authentication tests use a local mock server;
a successful live login requires a real cookie entered in the application.

## User settings and campaigns

In **Settings**, connect to D&D Beyond, then use **Add from D&D Beyond**:

1. Select a campaign from your account. Splat fills in its name and campaign ID.
2. Select its data folder with **Browse…**, or enter an absolute folder path.
3. Select **Open campaign**.

Choose an existing folder (the system folder picker can create one). Splat discovers
an existing campaign manifest anywhere in that folder, or creates `Campaign.md`.
Cancel does not create files. A folder linked to a different remote campaign,
multiple manifests, invalid settings, or duplicate registrations must be repaired
before opening. Campaign and package folders cannot overlap.

Use **Open existing campaign folder** to restore a campaign without connecting to
D&D Beyond. Its manifest supplies its UUID, name, link, custom paths, content
selections, and rules. To relocate a campaign, move its whole folder, then open
that folder in Splat. Missing folders stay registered. **Remove from Splat** removes
only the registration; files and the saved link stay in the folder.

The campaign manifest is the source of campaign settings. Its `splat_name` is the
local display name; `splat_campaign` contains version 1, `dndbeyond_campaign_id`,
and `dndbeyond_campaign_name`. Credentials stay in the desktop keyring and memory.
Optional paths can be edited in the manifest:

```yaml
splat_storage:
  characters: "Characters/D&D Beyond"
  character_cache: ".splat/characters"
splat_run_storage:
  directory: "Encounter Runs"
```

Character paths must be relative folders inside the campaign, without `..` or
symbolic links. Run storage retains support for existing absolute paths. Relative
paths work after a folder move. Changing a path does not move old files. Character
JSON records, including provider responses, are stored in the campaign cache;
its default location is `.splat/characters/<local UUID>/<remote ID>/`. The old app
cache is copied on first use if no campaign cache exists, and is kept for recovery.

App settings use `$XDG_CONFIG_HOME/splat/settings.json`, or
`~/.config/splat/settings.json` on Linux. They retain folder registrations, cached
campaign metadata, shared package registrations, and the active selection. On
startup or **Reload**, existing manifests take precedence over cached metadata.
Legacy campaigns gain manifest settings while retaining their UUIDs, links,
custom fields, and note text. Invalid or unsupported files are kept unchanged.
External edits require a reload before a conflicting save.

Each campaign can have one D&D Beyond link. A remote campaign cannot be linked
to two registered campaigns. The existing link control can replace or remove a
link. Remote name refreshes preserve a custom local display name; DM names and
player counts stay in memory. Failed requests keep the cached remote name, and
late responses cannot apply a name to a replaced link.

## Library workspace

Select **Library** to work on reusable modules without changing the active campaign.
The built-in D&D 2024 SRD appears as one read-only catalogue. Search entries by name
or path, filter by kind, and select an entry to read its Markdown or inspect its source.
Engine support remains separate from structural validity.

Under **Manage modules**, register an existing folder containing exactly one package
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
campaign without a manifest gets `Campaign.md` during setup, with its existing UUID
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
file or participant identities. A new campaign gets a manifest during setup.
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
