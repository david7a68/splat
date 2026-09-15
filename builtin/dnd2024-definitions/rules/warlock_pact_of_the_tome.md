---
splat_schema: 1
splat_id: 11ffd42c-6b9b-59d6-8c80-dc1af3c23326
splat_kind: rule
splat_key: dnd2024:rules/warlock_pact_of_the_tome
splat_name: Pact of the Tome
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
---

# Pact of the Tome

## Source text

<!-- SRD page 74 -->

Stitching together strands of shadow, you conjure\
forth a book in your hand at the end of a Short or\
Long Rest. This Book of Shadows (you determine its\
appearance) contains eldritch magic that only you\
can access, granting you the benefits below. The\
book disappears if you conjure another book with\
this feature or if you die.\
Cantrips and Rituals. When the book appears,\
choose three cantrips, and choose two level 1 spells\
that have the Ritual tag. The spells can be from any\
class’s spell list, and they must be spells you don’t\
already have prepared. While the book is on your\
person, you have the chosen spells prepared, and\
they function as Warlock spells for you.\
Spellcasting Focus. You can use the book as a\
Spellcasting Focus.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_pact_of_the_tome ` |
| ` implementation ` | ` dnd2024:rules_warlock_pact_of_the_tome ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` book_disappears ` | ` ["replacement_book_conjured", "owner_dies"] ` |
| ` spellcasting_focus ` | ` true ` |

#### Conjure

<!-- splat:values path=splat_rule/parameters/conjure -->

| Field | Value |
| --- | --- |
| ` timing ` | ` ["short_rest_finished", "long_rest_finished"] ` |
| ` location ` | ` hand ` |
| ` appearance ` | ` owner_choice ` |
| ` exclusive_magic_access ` | ` owner ` |

#### On conjure choices

<!-- splat:table
path: splat_rule/parameters/on_conjure_choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Count:
    path: count
    type: integer
  Spell level:
    path: spell_level
    type: integer
  Source:
    path: source
    type: symbol
  Exclude already prepared:
    path: exclude_already_prepared
    type: boolean
  Ritual:
    path: ritual
    type: boolean
    optional: true
-->

| Key | Count | Spell level | Source | Exclude already prepared | Ritual |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 3 ` | ` 0 ` | ` Any class spell list ` | true |  |
| ` row_2 ` | ` 2 ` | ` 1 ` | ` Any class spell list ` | true | true |

#### While book on person

<!-- splat:values path=splat_rule/parameters/while_book_on_person -->

| Field | Value |
| --- | --- |
| ` chosen_spells_prepared ` | ` true ` |
| ` chosen_spells_class ` | ` warlock ` |
