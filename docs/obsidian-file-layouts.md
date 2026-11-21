# Obsidian file layouts

Splat reads `splat_schema: 2` and `timeline_version: 2`. It leaves files with
other versions on disk and reports the path and reason. The reader hides
front matter. A flagged row stays in its file with a line warning.

## Check files

From the Splat repository, build Splat and run its file checker. Use the path
to a vault, campaign or package.

```sh
devenv shell -- cargo build --locked --bin splat
devenv shell -- ./target/debug/splat check /path/to/folder
```

The checker prints flagged and unreadable files with line numbers and
reasons, followed by totals. Exit status 1 means it found a flagged or
unreadable file. Exit status 2 means the arguments were invalid or it could
not open the folder.

## Stat blocks, spells and items

| Entry | Heading | Table columns |
| --- | --- | --- |
| Creature | `Stat block` | `Stat \| Value` and `Ability \| Score \| Mod \| Save` |
| Spell | `Spell` | `Field \| Value` |
| Item | `Item` | `Field \| Value` |

Edit the table to change a stat. Splat preserves formulas, spacing and other
text. It keeps printed stats in a folded quote callout for reference. It
checks printed modifiers and passive initiative. A bad cell keeps its text
and gets a line warning. The value stays unset. Rejected vitals also show a
warning in the reader.

Table shape hints preserve compound identities and text ranges. Explicit
ritual and concentration rows preserve both true and false values.

## Campaign and World notes

- Campaign files use flat properties, package task items and a Rule/Choice
  table. Clear a package box in Obsidian to disable the package. Splat accepts
  task bullets `-`, `*` and `+`, tab indents, and commas within links.
- A rule choice uses the visible name or link. The next save updates its
  stored ID. An unknown choice gets a line warning.
- World type names are case-insensitive. Unique stat block names and
  vault requirement links appear in Properties. Character payloads and
  package routes go in folded Splat data callouts.

## Sessions and runs

Write plan tasks such as `- [ ] [[Bridge Ambush]]`. Splat leaves out the title
property when a session title is empty. Obsidian's core Templates plugin can
fill `{{date}}` and `{{title}}`.

Splat saves runs as JSON. In Obsidian Sync, enable **other file types** to sync
these files.

## Timeline

Splat stores days and events in the Markdown body. The day ID goes in a
`%% splat id=… %%` comment below its heading. The event ID goes at the end of
its list item.

## Encounters and groups

Saved encounters use a roster table. Encounter groups use a member table.
Splat names new files after their entries and preserves other tags, properties
and prose when it saves.

- Enter a unique name or `[[Name]]` in the Creature cell. Saving writes a
  named link. If a link contains a pipe, use `\|` in the table. Splat reads both
  escaped and unescaped pipes. The visible link takes precedence over the
  stored ID, which helps resolve moved files.
- Slot keys go in comments in the Note cell. Splat keeps text on both sides
  of the comments. Copying a roster row copies its visible values to a new
  slot. Splat flags a copied party member.
- Use `Side rivals` for a group member's side. In roster Changes, use
  `Start HP 12` for starting HP. Speed accepts `40`, `40 ft` or `40 ft.` and
  saves as `Speed 40 ft.`.
- If a group has an unread row, **Groups**, **Arrivals** and the plan show how
  many members can join. **Edit** opens the regular change form and keeps
  the name and note.

## Character copies and duplicate files

A character copy keeps its identity after a rename. It uses the character's
name once and keeps imported data in a folded section at the end. If Splat
cannot read a copy, a refresh can rewrite it and keep the personal notes if
the generated text still matches its checksum. If the text changed, choose
**Replace with D&D Beyond sheet** to archive the previous file before replacing it.

If you copy a file, the repair names the file that will become a separate
entry. Use **Undo separate entry** beside the result to reverse the repair.
