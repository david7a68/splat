# Packages and campaign content

> Historical decision record. The remaining schema details are now specified in
> [Mechanics](mechanics-schema.md), [State](state-schema.md), and
> [Campaign/storage](campaign-schema.md). Those contracts supersede the older
> illustrative fields and open-detail lists here. No implementation is authorized.

Status: consolidated package design agreed, including optional copy or move when sharing campaign content.
This is a conceptual design, not an implemented schema. No data-model
implementation is authorized.

## Scope

Packages are local-only. Package updates, version management, distribution,
publishing, arbitrary package-supplied code, and an Obsidian interaction plugin
are outside the initial scope. Existing user settings and credential storage
are unchanged by these content-storage decisions.

## Conceptual model

| Concept | Responsibility |
| --- | --- |
| Package | Owns a reusable collection of shared entries and has a Markdown manifest. |
| Entry | Has a stable UUID, kind, name, source attribution, mechanics, descriptive text, and required references. |
| Package selection | Makes a whole package or specific entries available to a campaign by reference. |
| Rule group | Keeps a named set of related rules together as one selection unit. |
| Campaign rule choice | Chooses the applicable rule for a mechanic and resolves incompatible alternatives. |
| Campaign-owned entry | Holds original campaign content or an independently editable copy with an optional origin reference. |
| Inline control | Refers to structured mechanics and uses the viewing context to determine which state it can affect. |

A source identifies where content originated, such as a sourcebook or author.
It is separate from the package that distributes the local collection.

## Packages and selection

Every shared entry belongs to a package. My Homebrew is the default package for
user-created shared content. Homebrew and externally supplied packages have the
same capabilities. A D&D 2024 core package supplies the default game rules using
operations supported by Splat's engine.

The UI uses checkboxes with Select all and individual deselection:

- Entire package enables all entries, including entries added locally later.
- Selected entries records a partial selection of specific entries; later
  additions are not enabled automatically.
- Required dependencies are included and their reasons are shown separately
  from direct user selections. Optional content is not included automatically.
- Individual content entries and named, indivisible rule groups can be selected.

Entries with the same name coexist. Stable UUIDs establish identity; the UI shows
package names to distinguish entries. Names never cause implicit replacement.

## References and simulation eligibility

References to specific creatures, spells, or other content identify exact entries.
References to replaceable mechanics, such as taking a long rest, use the rule
chosen for that mechanic by the campaign.

Shared packages may depend only on shared packages. Campaign content may depend
on shared content or other content in the same campaign.

Disabling or removing required content leaves unresolved references. Existing
encounters remain editable, but affected simulation is blocked until required
content is restored or replaced. Unsupported behavior also blocks affected
simulation by default; it must never be silently treated as implemented.

Rule conflicts require an explicit campaign choice. Declared replacements are
supported; loading order never silently chooses a winner. Compatible additions
can coexist. Rule choices apply campaign-wide. Encounter templates do not silently
override them. Any future experimental or approximate simulation mode must be
explicit and identify changed or omitted behavior.

The combat design now explicitly permits targeting assumptions for simulations
without spatial calculations. Results show those assumptions, and mechanics
that remain unresolved still block affected simulations. This limited exception
is described in [Creatures and encounters](combat.md); other approximations
remain deferred.

## Campaign-owned homebrew and sharing

Campaign-specific homebrew can be created directly in the campaign. It does not
require a package used by only that campaign.

Imported D&D Beyond definitions differ: they go to an enabled package or to the
[campaign package](character-content.md#routes-and-the-campaign-package), not to loose
campaign-owned notes.

Sharing campaign content to a package offers two explicit operations:

- Copy to package creates a shared copy with a new UUID. The campaign original
  and existing references remain unchanged.
- Move to package changes ownership of the existing entry, retaining its stable
  identity and updating references or selection as needed so existing campaign
  uses resolve to the shared entry. The campaign-local original no longer exists
  as a separate entry.

Before either operation completes, campaign-local required dependencies must also
be shared or replaced with shared dependencies. Sharing cannot leave a package
referencing private campaign content. The operation must show these dependencies;
it does not silently move unrelated content.

## Campaign copies

Making a copy duplicates only the selected entry. The copy is independently
editable and owned by the campaign. Dependencies remain shared references until
the user copies those separately.

Copies retain an optional reference to their origin. Removing the original does
not invalidate the copy itself; required dependencies are handled separately.
Creating a copy never rewrites existing encounter references automatically.
The user explicitly chooses where to use it.

## Markdown storage

Packages are ordinary folders. Each package has a Markdown manifest note for
identity, description, and package-level metadata. Entries are separate
Obsidian-flavoured Markdown files with YAML frontmatter. Campaign-owned entries
live within the campaign's configured folder structure. Placement must preserve
the user's Johnny.Decimal organization and allow related notes to stay together.

Package folders can be registered from any chosen location. Their paths stay
in app settings; campaigns select packages by UUID. A campaign manifest note
keeps its package selections and rule choices with the campaign folder.
All top-level Splat frontmatter fields use the `splat_` prefix, separately from
ordinary Obsidian metadata. Common fields are flat; complex properties use
nested YAML where required. Package and campaign note placement uses the same
configurable conventions, preserving existing folders when defaults change.

- YAML frontmatter is authoritative for stats and executable rules. Inventory
  is an explicit exception: marked Markdown tables hold the inventory list
  and item references without a duplicate YAML inventory list.
- The Markdown body contains descriptions, notes, examples, inline controls,
  and authoritative marked inventory tables. Referenced item mechanics stay
  in YAML.
- Nested YAML is acceptable. Obsidian's Properties UI cannot edit nested
  properties; use Source mode or Splat's editor for those structures.
- Source prose is not interpreted as executable behavior.

Files use readable names, such as `Goblin.md`, with stable UUIDs in frontmatter.
Renaming or moving a note preserves its identity. Duplicate entry names can use
different folders or distinguishing filenames.

An entry reference uses either `id` for a UUID or `key` for a namespaced resource
key, plus optional `link` for Obsidian navigation. Each package or campaign has
its own namespace; optional entry keys are independent of note filenames and
resolve to the entry's stable UUID. Duplicate keys are reported as conflicts.
Navigation links can be repaired after a note moves or changes name.

These decisions are consolidated in [Markdown storage](storage.md). Directory
layout, exact field names, and the full entry schema remain to be reviewed.

## Interactive rendering

Use namespaced inline code spans, such as `splat:roll bite.attack`, to reference
stable local keys in the structured mechanics. Splat renders a suitable control
whose label and behavior come from that definition. The same effect is never
independently defined in both YAML and prose.

Ordinary Obsidian displays readable inline code. Links retain their navigation
meaning. This follows established plugin conventions but is Splat-specific
notation, not native Obsidian behavior or automatic plugin compatibility.
The full grammar and control vocabulary will be specified with actions/effects.

When browsing library content, controls offer preview rolls without changing
shared definitions or encounter participants. During play, controls operate on
the selected encounter participant. They never implicitly write to D&D Beyond.
An Obsidian plugin for the interactive behavior is deferred.

## Design tree and remaining work

- Package concepts, ownership, selection, references, and copy behavior: agreed.
- Campaign-wide rule choices and dependency failure behavior: agreed.
- Markdown/YAML storage and inline-code control convention: agreed.
- Consolidated package model, including campaign homebrew and sharing: agreed.
- Common combat model, including inventory and consumables, template/run
  separation, local actions, and choices separate from rules: agreed in
  [Creatures and encounters](combat.md).
- Detailed entry schemas, actions/effects, encounters, and simulation state:
  active design branch, not implemented.
- Readable filenames and UUID references with optional Obsidian navigation
  links: agreed in [Markdown storage](storage.md).
- Exact file layout, reference field names, and full control grammar:
  pending the relevant model details.
- Updates, version management, distribution, publishing, arbitrary package code,
  and an Obsidian interaction plugin: deferred.

## References

Foundry modules can contain compendium packs. Enabling content and importing
an independent copy are distinct operations. This is a conceptual reference,
not a requirement to reproduce Foundry's runtime architecture.

- https://foundryvtt.com/article/modules/
- https://foundryvtt.com/article/compendium/
- https://foundryvtt.com/article/packaging-guide/

Obsidian supports YAML frontmatter but not nested properties in its Properties UI:

- https://help.obsidian.md/properties

Existing plugins use inline-code spans for controls:

- Dice Roller, `dice: 1d20 + 5`: https://github.com/javalent/dice-roller#usage
- Meta Bind, `BUTTON[docs-button]`: https://github.com/mProjectsCode/obsidian-meta-bind-plugin/blob/master/exampleVault/Buttons/Button%20Example.md

Executable-document precedents considered during the interview:

- Inform 7: https://ganelson.github.io/inform-website/
- Quarto inline code: https://quarto.org/docs/computations/inline-code.html
- Cucumber/Gherkin: https://github.com/cucumber/docs/blob/main/content/docs/gherkin/reference.md
