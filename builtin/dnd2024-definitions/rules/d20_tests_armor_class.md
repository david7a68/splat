---
splat_schema: 1
splat_id: 0c74fdb5-106a-509f-a01c-2496a34593be
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_armor_class
splat_name: Armor Class
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
---

# Armor Class

## Source text

<!-- SRD page 7 -->

A creature’s Armor Class represents how well the\
creature avoids being wounded in combat. The AC\
of a character is determined at character creation\
(see “Character Creation”), whereas the AC of a\
monster appears in its stat block.\
Calculating AC. All creatures start with the same\
base AC calculation:\
Base AC = 10 + the creature’s Dexterity modifier\
A creature’s AC can then be modified by armor,\
magic items, spells, and more.\
Only One Base AC. Some spells and class features\
give characters a different way to calculate their AC.\
A character with multiple features that give differ-\
ent ways to calculate AC must choose which one to\
use; only one base calculation can be in effect for a\
creature.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_armor_class ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_armor_class ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` maximum_simultaneous_base_formulas ` | ` 1 ` |
| ` modifiers_from ` | ` ["armor", "magic_items", "spells", "other_rules"] ` |

#### Default base ac

<!-- splat:values path=splat_rule/parameters/default_base_ac -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |
| ` left ` | ` 10 ` |
| ` right ` | ` dexterity_modifier ` |

#### Base formula choice

<!-- splat:values path=splat_rule/parameters/base_formula_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` from ` | ` available_ac_formulas ` |
