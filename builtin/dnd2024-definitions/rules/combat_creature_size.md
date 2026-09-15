---
splat_schema: 1
splat_id: 4fab2519-088c-5319-b1f3-ba8b5640e688
splat_kind: rule
splat_key: dnd2024:rules/combat_creature_size
splat_name: Creature Size
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
---

# Creature Size

## Source text

<!-- SRD page 14 -->

A creature belongs to a size category, which deter-\
mines the width of the square space the creature\
occupies on a map, as shown on the Creature Size\
and Space table. That table lists the sizes from\
smallest (Tiny) to largest (Gargantuan). A creature’s\
space is the area that it effectively controls in com-\
bat and the area it needs to fight effectively.\
A character’s size is determined by species, and\
a monster’s size is specified in the monster’s stat\
block.\
Creature Size and Space\
Size Space (Feet) Space (Squares)\
Tiny 2½ by 2½ feet 4 per square\
Small 5 by 5 feet 1 square\
Medium 5 by 5 feet 1 square\
Large 10 by 10 feet 4 squares (2 by 2)\
Huge 15 by 15 feet 9 squares (3 by 3)\
Gargantuan 20 by 20 feet 16 squares (4 by 4)\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_creature_size ` |
| ` implementation ` | ` dnd2024:rule_combat_creature_size ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` player_size_source ` | ` species ` |
| ` monster_size_source ` | ` stat_block ` |

#### Sizes

<!-- splat:rows path=splat_rule/parameters/sizes -->

| Row | size | space_width_feet | occupied_squares |
| --- | --- | --- | --- |
| ` 0 ` | ` tiny ` | ` 2.5 ` | ` 0.25 ` |
| ` 1 ` | ` small ` | ` 5 ` | ` 1 ` |
| ` 2 ` | ` medium ` | ` 5 ` | ` 1 ` |
| ` 3 ` | ` large ` | ` 10 ` | ` 4 ` |
| ` 4 ` | ` huge ` | ` 15 ` | ` 9 ` |
| ` 5 ` | ` gargantuan ` | ` 20 ` | ` 16 ` |
