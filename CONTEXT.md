# Splat

Splat helps a game master plan encounters for D&D campaigns.

## Language

**Campaign**:
A local campaign managed in Splat, with a name and a folder for its files.
_Avoid_: Project

**D&D Beyond campaign**:
A campaign hosted on D&D Beyond. It can be linked to one local campaign;
a local campaign can have one such link or remain unlinked.

**Active campaign**:
The one local campaign currently selected by the user.

**Shared library**:
The collection of reusable packages available across campaigns. Every shared
entry belongs to a package.

**Campaign copy**:
An independently editable copy of a shared entry owned by one campaign. It
retains shared dependencies and may remember its origin; it does not require
the original entry to remain available.

**Package**:
A collection of reusable game content and rules that a campaign can enable in
whole or in part. Homebrew and externally supplied packages have the same role.
_Avoid_: Source, module

**Package registration**:
The association between a package's identity and its location on this computer,
which makes it available to Splat's shared library.

**Package manifest**:
The identity, description, and collection-level metadata of a package.

**Campaign manifest**:
A campaign's identity, display name, and content configuration, including its
package selections and rule choices, kept with the campaign.

**Source**:
The origin of content or a rule, such as a sourcebook or homebrew author.
A package can include material from multiple sources.

**Package selection**:
The content and rules a campaign enables from a package. Enabling content makes
it available through the package; creating an independent campaign copy is a
separate action.

**Rule**:
A definition of how the game behaves. Rules can be presented for people to read
and used to determine simulation outcomes.

**Rule group**:
A named set of related rules enabled together so that their behavior remains
complete.

**Required dependency**:
Content or rules needed by another enabled entry. A campaign selection includes
these dependencies and is incomplete when one is unavailable.

**Rule conflict**:
Incompatible rules that affect the same behavior. The campaign must explicitly
choose which rule applies; loading order does not decide it.

**My Homebrew**:
The default package for shared content created by the user.

**Copy origin**:
The shared entry from which a campaign copy was made. It records the copy's
history without requiring that original entry to remain available.

**Unsupported rule**:
A rule whose behavior Splat cannot currently simulate. It prevents simulation
that needs that behavior, while the related content remains viewable and editable.

**Core package**:
The package that supplies the campaign's standard game rules. Splat initially
uses the D&D 2024 core package by default.

**Entry**:
An individually identifiable item of package content, such as a creature,
spell, or encounter template. Entries can have the same name without being
the same item.

**Entry reference**:
A reference to a particular entry by UUID or resource key, optionally accompanied
by an Obsidian link for navigation.

**Resource key**:
A readable namespaced identifier such as `dnd2024:spells/bless`, independent of
a note's display name and filesystem location.

**Namespace**:
The name belonging to a package or campaign that qualifies the resource keys
of its content, unique among the owners registered in Splat.

**Identity conflict**:
Two distinct notes claiming the same entry identity, requiring the user to
identify which keeps that identity and which becomes a new entry.

**Draft entry**:
An incomplete or invalid entry retained for editing. Simulations that require
its unresolved content cannot run until the issues are corrected.

**Campaign rule choice**:
A campaign-wide choice of which rule applies when alternatives exist.
An encounter template uses the campaign's choices.

**Mechanic**:
A named game behavior, such as taking a long rest, for which the campaign chooses
the applicable rule. Content using that mechanic follows the campaign's choice.

**Inline control**:
An interactive element within rendered descriptive text that invokes a defined
roll or other supported interaction. Its behavior comes from structured mechanics,
not from interpreting the surrounding prose.

**Encounter participant**:
One particular creature or character taking part in an encounter run, with its
own current combat state. An inline control used during play acts on that participant.

**Combat model**:
The stats, actions, resources, effects, inventory, and consumables used to resolve
combat for both monsters and player characters.

**Encounter template**:
A reusable definition of an encounter's roster, starting setup, and triggered
phases.

**Encounter roster**:
The party members, creatures, or combination of both selected for an encounter,
including participants that enter during a later phase.

**Encounter side**:
A group of participants assigned to the same side of an encounter, independently
of their creature type or membership in a campaign's party.

**Encounter group**:
A reusable, named list of creatures with counts, a default side, and notes, owned by
a campaign. It is for any encounter, or it belongs to one encounter, and then only runs
of that encounter offer it. The DM adds it to a running encounter when it arrives. Each
addition creates new participants. It is not a side, which sorts participants within one
encounter; not a roster, whose participants one encounter always starts with; not a rule group, which enables
rules together; and not an initiative group, which shares a turn order position.
_Avoid_: Team, wave, spawn group

**Made-up creature**:
A creature the DM names and gives HP while a run is open, with no note behind it. It joins
as an encounter group's members do, and it lives only in that run's saved state. It is not a
campaign creature, and it does not become one.
_Avoid_: Temporary creature, custom monster

**Override**:
An explicit change an encounter roster slot or encounter group member builds in to
one creature, such as a custom name or extra HP. It belongs to that slot or member,
is applied when the participant is created, and is always shown beside the value it
replaces. It never changes the creature's entry. To change a creature everywhere,
use a campaign copy.
_Avoid_: Variant, scaling

**Loot**:
What the party can take after a fight. An encounter has its own, in its note's `## Loot` list, and a roster slot
or encounter group member can give its one creature some. A line is words, or a roll on a roll table by its key.
A roll's result stays with its line in the run, and the log has it too. Loot is not an override, because it changes nothing about the creature,
but it is kept beside a slot's overrides and frozen into a run in the same way.
_Avoid_: Treasure, drops

**Encounter phase**:
A part of an encounter governed by triggers that can cause participants to enter
or leave combat. Phases have independent triggers and can overlap.

**Encounter trigger**:
An event or condition that causes a planned change in an encounter, once by
default or repeatedly when explicitly configured.

**World**:
The fictional setting of a campaign, as notes and entries.
_Avoid_: Setting, which is confused with app settings.

**World note**:
A plain Markdown file in a campaign that Splat reads and does not own. Its path is its identity.

**World entry**:
A note with a Splat identity that represents something in the world.

**NPC**:
A world entry for a person the DM plays. It can point at a stat block, and it is not a creature entry.

**NPC generator**:
A Lookup action that rolls a name and three attributes (look, manner, want) from the campaign's marked tables or built-in lists. Nothing is written until the DM saves the result as an NPC.

**Location**:
A world entry for a place. Locations can be inside other locations.

**Roll table**:
An existing Markdown table that a marker makes rollable.

**Campaign clock**:
The one count of standard hours, computed from the timeline, whatever world the party is on.

**Session**:
One meeting of the table, with its notes in a Markdown file in the campaign's folder.

**Session plan**:
The encounters a session's note links to under its plan, with what has happened so far. The note holds the
plan, so that Obsidian sees it too.

**Carried state**:
Each player's HP and temporary HP from the last fight of the session being played, which the next run starts
from. A long rest the DM marks starts everyone at full instead.

**Session template**:
Markdown a new session's notes start from, built in or owned by the campaign.

**Timeline**:
The campaign's record of what happened in the story, in order. It stands apart from any session.

**Timeline event**:
One encounter, rest, travel interval, day or other major event on the timeline, with a stable
ID that a note can refer to. It can belong to a session.

**Timeline reference**:
A link to one timeline event, `[[Timeline#^id|words]]`. Splat and Obsidian both follow it.

**Recording**:
Audio of a session, captured by the host's recorder and kept beside the campaign's notes.

**Raw transcript**:
The words of a recording as the speech service heard them. Nothing changes it later.

**Clean transcript**:
A raw transcript with its errors corrected and its events marked by an LLM, then checked against
the raw words by a second pass.

**Event marker**:
A suggestion in a transcript, written by an LLM, that a timeline event starts there. It reaches
the timeline only when the DM adds it.

**Encounter sequence**:
A series of encounters with explicit elapsed time and rests, across which
participants retain their HP, resources, inventory, and ongoing effects.

**Encounter run**:
One playthrough or simulation of an encounter, with its own participants,
current HP, spent resources, and active effects.

**Retired encounter**:
An encounter template with an ended run, unless it is marked repeatable. It is not a state in its
note. It leaves the ready list and stays reachable from its ended runs in History.
_Avoid_: Archived

**Completion condition**:
An objective or state that ends an encounter automatically when satisfied.
An encounter can also be ended manually.

**End reason**:
The reason an encounter run stopped, such as meeting a completion condition,
a manual ending, or reaching a simulation limit.

**Encounter outcome**:
The result of an encounter run for its sides or objectives, recorded separately
from why the run stopped.

**Simulation trial**:
One automatic evaluation of an encounter or sequence from its selected starting
state, contributing one result to a simulation report.

**Simulation seed**:
The value that initializes a trial's random choices. Reproducing the trial also
requires the same inputs and compatible simulation behavior.

**Seed record**:
Saved metadata for regenerating a simulation trial, including its seed, setup
reference, engine identification, and input fingerprints. It does not preserve
the original inputs or results.

**Initiative method**:
The campaign's rule for establishing turn order, including individual or grouped
initiative.

**Initiative group**:
A set of participants that share an initiative position while retaining their
individual identities and combat state.

**Roster order**:
The explicit order of participants in an encounter roster, used as a stable
fallback when initiative does not decide their order.

**Simulation strategy**:
The policy that selects a participant's combat choices during simulation.
The applicable rules determine which choices are allowed and their outcomes.

**Combat sync**:
The background exchange of a linked character's HP, death saves, conditions, spell slots and limited
uses with D&D Beyond. It runs from the time Splat connects. An encounter can turn it off for its run.

**Controller**:
The person or strategy that makes a participant's permitted choices. A control
effect can change the controller without changing which creature it controls.

**Party binding**:
The association between an encounter's party slot and a campaign's party member.
It lets a shared encounter use different parties without owning their characters.

**Creature relationship**:
A source-defined connection between particular creatures, such as a grapple,
summoning, control or possession. Its rules determine the effects and how it ends.

**Form profile**:
The rules for which statistics, health, equipment and effects a creature retains
or changes in another form, including how that form ends.

**Readied spell**:
A spell already cast whose energy is held for a declared trigger. Its remaining
opportunity, Concentration and release follow its rules and the controller's
permitted choices.

**Item definition**:
The reusable description and rules of an item, separate from particular items
carried by creatures or kept in containers.

**Inventory item**:
An item or quantity of identical items held in an inventory, with its own
equipment state. Items with different individual state are tracked separately.

**Container**:
An item that can hold other items, such as a barrel, backpack, or Bag of Holding.
It has its own storage rules and can exist without a creature carrying it.

**Container setup**:
An authored starting configuration for a container and its contents, which can
be used when preparing an encounter or a creature's inventory.
Each placement creates an independent container and contents. Moving an
existing container preserves that container and its contents.

**Container capacity**:
The limits on a container's contents, such as weight, volume, or item count,
as specified by its rules.

**Carried weight**:
The weight an item contributes to its holder, including any effect of its
contents according to the item's rules.

**Inventory row key**:
A stable local identifier for an inventory item or stack within an inventory,
independent of row order and item display name.

**Equipment requirement**:
A condition an inventory item must meet to grant an action or effect, such as
being worn, wielded, or attuned to its bearer.

**Supplied stat**:
A stat given as a stated value, with information about the contributions already
included in that value when known.

**Calculated stat**:
A stat whose value is determined from identified contributions using the
applicable rules.

**Resource pool**:
A named supply spent by one or more abilities, with a current amount, a limit,
and one or more mechanisms for recovery. A creature or item can have several
distinct pools.

**Active effect**:
An ongoing game effect on a participant, with an origin and an end condition.
It can change the participant's effective stats without changing their base stats.

**Effect application**:
A group of related effects produced by one spell casting or ability use.
Its effects can affect different participants and have different end conditions.

**Roll scope**:
The set of targets or resolutions that use one roll result, as specified by
the applicable rule.

**Action resolution**:
The stages through which an ability use selects targets, spends resources,
makes rolls, and applies effects, as required by its rules.

**Action use**:
One attempt to perform an ability by a particular participant, with its selected
targets, costs, outcomes, and any item that grants it. One use can contain
several separate attack resolutions.

**Action budget**:
A limited permission to act, with its own eligible uses and refresh timing.
An extra reaction restricted to opportunity attacks is a separate permission
from an ordinary reaction.

**Damage component**:
One typed contribution to damage, such as the fire part of a hit that also
deals piercing damage. Several components can belong to one damage event.

**Damage event**:
One instance of damage against a participant, with its cause and applicable
components. Its rules determine adjustments and the effects it triggers.

**Standalone roll**:
A roll of an ability's dice expression that displays a result without using
the ability, spending resources, or changing a participant's state. A later
ability use does not automatically reuse this result.

**Reaction opportunity**:
A point during play at which a qualifying participant can use a reaction,
as specified by that reaction's trigger and timing.

**Nested reaction**:
A reaction triggered during the resolution of another reaction. Its timing
rules determine when it resolves before the interrupted resolution continues.

**Target eligibility**:
The conditions that determine whether a participant can be affected by an
ability or effect, including when those conditions must be checked.

**Targeting assumption**:
An explicit simplification used to determine reachability or affected targets
in a simulation without spatial calculations.

**Stacking key**:
The identity used by rules to match effects that count as the same mechanic
when deciding how their contributions combine.

**Effect tag**:
A category used to describe an effect, such as a spell effect or an AC bonus.
Sharing a tag does not by itself make effects conflict.

**Suppressed effect**:
An ongoing effect whose contribution is temporarily excluded by a stacking rule.
Its duration continues, and it can contribute again if the rule permits.

**Preview roll**:
A roll made while browsing a definition, without changing an encounter
participant or the shared definition.

**Campaign-owned entry**:
Content owned directly by one campaign, either created there or copied from
shared content. It does not need to belong to a package.

**Share to package**:
Make campaign-owned content available in a shared package. The user chooses
whether to create a shared copy or move the existing entry into the package.

**Campaign package**:
The package that holds a campaign's imported definitions when no enabled package
note lists their D&D Beyond ID. It stays in the campaign's folder and is private to that campaign.
_Avoid_: Campaign-owned entry, which the DM creates or copies.

**D&D Beyond ID**:
The provider's identity for a definition, such as `feature:8429477`. A package note
lists the definitions it holds in `splat_dndbeyond_ids`. One note can list many.

**Import route**:
The recorded place of one imported source definition: a note in an enabled
package, or the campaign package. A route stands until the DM approves a change.

**Package reconciliation**:
The check of import routes against the packages that a campaign enables. It lists
definitions that can move to a package, must return to the campaign package, or no
longer have a use. Nothing moves until the DM applies the change.

**Conversion gap**:
A source rule that still needs a complete typed definition or an explicit
interpretation. Extracting its numbers or keeping its prose does not close it.

**Engine gap**:
A specified rule contract for which the engine has no implementation. The list of
implemented contracts is in Rust code, so content cannot close a gap by itself.

**Character check**:
A comparison of a D&D Beyond character with what the SRD rules give for its classes,
levels and choices. It reports differences and changes nothing: D&D Beyond stays the
source of truth.

**Class link**:
A reviewed, source-pinned match between a D&D Beyond class and an SRD class. A class
with no link is not compared, because a name alone does not prove the edition.

**Execution profile**:
A bounded, executable view of a definition, with explicit limits on the rules
and situations it supports. It does not prove support for the full definition.

**Old layout**:
A file with `splat_schema: 1` or `timeline_version: 1`. Splat no longer loads it.
Library and Campaign flag it with its path and the Convert files action. Conversion
is explicit, through Convert files or `splat convert`; normal saves never convert.
The old parser exists only for conversion. Built-in notes use schema 2. `splat check`
uses the normal readers without opening a window or changing files.
