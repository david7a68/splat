# Optional character systems in D&D Beyond imports

Research date: 2026-09-16.

## Sources and scope

The three feature names identify specific published systems, not arbitrary
homebrew. Their feature records have `isHomebrew: false` and the identities below.
The D&D Beyond feature URLs redirect unauthenticated readers to the corresponding
official book listings. The cached first-party character API response contains
the individual option rules in `choices.choiceDefinitions`; the short feature
description alone is not the rules catalogue. [1–4]

| Feature | Provider feature ID | Source | Available options |
| --- | --- | --- | --- |
| Dark Bargain | 2048517 | *The Crooked Moon Part One*, Avantris; source 193 | 19 variants of 13 bargains |
| Character Threads | 2331813 | *Northlands Worldbook*, Kobold Press; source 238, page 125 | Seven threads, each in four milestone variants: 28 |
| Runestones | 2331889 | *Northlands Sagas*, Kobold Press; source 239, page 6 | Twelve runes, each in four power variants: 48 |

The catalogue therefore has 95 provider options. These are rules definitions,
not evidence that a character selected all of them. [4]

## Import state

The observed character feature choices have a populated `optionIds` list but
`optionValue: null` and no selected `options`. This means **no selection is
recorded in the source**. It does not establish whether the GM disabled the
system, whether a player has not selected a benefit, or whether a choice exists
outside the sheet. The Dark Bargain snippet even uses past-tense wording while
the description merely permits taking a bargain at the GM's discretion. Neither
template establishes an actual selected benefit. [4]

The program should represent these states separately:

- Unspecified: preserve the source's lack of a selection, with no active effects.
- Inactive: no feature grant on the sheet, or an explicit decision to use no effects
  from this system. This does not mean the system is disabled for the campaign.
- Active: an explicit selected option, plus any required secondary choices.

This state distinction is an application design decision based on the source's
missing selection data. Do not silently turn “unspecified” into “active” or make
all available options affect the character. Do not claim that “inactive” was
confirmed by a null provider field.

For each feature choice, find the choice definition whose ID equals
`{componentTypeId}-{type}`, then restrict its options to that choice's
`optionIds`. The shared definition contains other systems' options. Matching by
display name or copying the whole shared definition would mix unrelated rules.
Preserve the option ID, label, source ID, and source description fingerprint so
later rule changes cannot silently reuse an old conversion. [4]

## Dark Bargain

The parent feature permits taking a bargain at the GM's discretion. The source
offers 13 bargains; three have Intelligence, Wisdom, and Charisma variants,
which expands the provider list to 19. Benefits and costs belong together. A
selection can give spells, resources, passive effects, triggered effects, and
narrative consequences. Disabling the selected bargain must disable both its
benefits and its costs. [1, 4]

The families are Bestial Awakening, Call of the Lonely, Collector of Stories,
Crooked Fortune, Cult of Personality, Hand of Death, Might of the Old,
Self-Serving Sacrifice, Shrouded Spirit, Skin of Coffin Nails, Stilled Hungers,
Unquenchable Fury, and Wings of Whispers. [4, options 5098955–5098973]

Do not reduce these to unconditional stat additions. For example, Might of the
Old has a lasting Strength/Dexterity adjustment, a temporary Strength override,
and a fire vulnerability that lasts longer than that override. Skin of Coffin
Nails requires a selected physical damage type and changes that resistance to
vulnerability when its water condition applies. Crooked Fortune restores its
use after a sequence of consequential saving throws rather than after a rest.
[4, options 5098962, 5098965, 5098970]

There is an unresolved source inconsistency: **Collector of Stories (Charisma)**
(5098961) calls its spellcasting ability **Wisdom** in the rule description.
The sibling Intelligence and Wisdom entries agree with their labels. Preserve
this conflict and require a ruling before claiming an unambiguous casting
ability; the label alone is not sufficient to fix published text. [4]

## Character Threads

Threads are character goals, not automatically granted class features. The
source says they can be added during a character's career and gives benefits
when a milestone is achieved. The option labels use levels 1, 5, 11, and 17,
but the descriptions require achieved milestones. Level alone must not invent
progress or select milestone rewards. [2, 4]

The seven threads are Bloodsworn, Cursemarked, Explorer, Fatebound, Herald,
Legend Hunter, and Sworn Huskarl. Many milestone rewards are alternatives;
some also require a skill, language, spellcasting ability, magic item, patron,
or other campaign choice. Preserve those decisions explicitly instead of
granting every alternative. [4, options 5807809–5807836]

The provider repeats earlier milestone descriptions in higher-tier options.
This is not permission to apply every block independently. In particular,
**Cursemarked replaces all prior milestone benefits** at each higher milestone.
Its later Greater Sacrifice choices can refer to earlier sacrifice effects,
and its triggered features cannot overlap. Preserve these references without
reinstating superseded passive benefits. Other thread variants include their
earlier earned milestones, subject to the player's reward choices. [4, options
5807813–5807816 and the other thread options]

Bloodsworn's Wrath reward names Wrathful Smite without a link or edition. The
[official D&D Beyond spell search](https://www.dndbeyond.com/spells?filter-search=wrathful+smite)
lists a legacy version (2324: Evocation, Psychic damage, Concentration) and a
current version (2619220: Necromancy, Necrotic damage, no Concentration). The
source does not establish which is intended, and neither full spell definition
is present in these character caches. The reward permits one cast per Long Rest,
but does not explicitly waive spell slots or components. Keep the version and
slot-cost decision explicit, preserve normal components, and do not claim a
verified spell conversion from the name alone.

## Runestones

The parent instruction is specific to the beginning of the *Northlands Sagas*
adventure path: each player draws one Runestone randomly. The presence of the
generic feature in a sheet for another campaign does not establish that this
draw happened. The importer should not perform a random draw. [3, 4]

The twelve runes are Algiz, Ansuz, Berkanan, Eiwaz, Gebo, Jera, Kaunen, Laukaz,
Perto, Sowilo, Thurisaz, and Tiwaz. Each has provider variants labelled levels
1, 2, 7, and 12, describing least, lesser, greater, and patron powers. Higher
variants include earlier power text. Import the explicit selected variant;
do not select a rune or assume campaign progression from character level. [4,
options 5807896–5807943]

Most least powers say once per day and need no action to sketch. Later powers
can require an action, Bonus Action, or Reaction, and patron effects are often
conditional on proximity to a named deity. A once-per-day limit is not the
same thing as a Long Rest reset. Higher effects can replace earlier values:
Thurisaz increases its bonus damage die from d4 to d6 to d8, and Perto
increases its die from d6 to d8. They do not grant all those dice together.
Some later effects end the rune. Preserve each option's timing, costs,
duration, replacement rules, and patron conditions. [4]

## Conversion boundary

Supporting state and source retention is separate from interpreting every rule.
Active options need typed rule definitions that preserve benefits, costs,
conditions, progression, and secondary choices. If a selected option lacks a
verified conversion or its source text changes, retain the source and report a
conversion gap. Inactive options do not need engine support to produce no
effects. Unselected catalogue options should not create active-character
requirements. Simulation execution remains a separate implementation step.

## References

1. D&D Beyond, [Dark Bargain feature](https://www.dndbeyond.com/feats/2048517-dark-bargain),
   redirects to [The Crooked Moon Part One](https://marketplace.dndbeyond.com/category/DBCLGUN0E).
   The official listing identifies Avantris Entertainment and 13 Dark Bargains.
2. D&D Beyond, [Character Threads feature](https://www.dndbeyond.com/feats/2331813-character-threads),
   redirects to [Northlands Worldbook](https://marketplace.dndbeyond.com/category/DB86Q6I93).
   The official listing identifies Kobold Press.
3. D&D Beyond, [Runestones feature](https://www.dndbeyond.com/feats/2331889-runestones),
   redirects to [Northlands Sagas](https://marketplace.dndbeyond.com/category/DBYLR645O).
4. First-party D&D Beyond character API snapshots already stored by the local
   application, inspected on the research date. Reproducible source locations:
   feature definitions 2048517, 2331813, and 2331889 (entity type 1088085227),
   the corresponding `choices.feat` records, and
   `choices.choiceDefinitions[id="1088085227-3"].options`, restricted to each
   choice's `optionIds`. Exact option ranges are given above. Player identifiers,
   character details, and full licensed source text are intentionally omitted
   from this research note.
