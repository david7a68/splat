---
splat_schema: 1
splat_id: 9c89020a-0301-503f-a7b5-d4783212276d
splat_kind: rule
splat_key: dnd2024:rules/combat_movement_and_position
splat_name: Movement and Position
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
---

# Movement and Position

## Source text

<!-- SRD page 14 -->

On your turn, you can move a distance equal to your\
Speed or less. Or you can decide not to move.\
Your movement can include climbing, crawling,\
jumping, and swimming (each explained in “Rules\
Glossary”). These different modes of movement can\
be combined with your regular movement, or they\
can constitute your entire move.\
However you’re moving with your Speed, you\
deduct the distance of each part of your move from\
it until it is used up or until you are done moving,\
whichever comes first.\
A character’s Speed is determined during char-\
acter creation. A monster’s Speed is noted in the\
monster’s stat block. See “Rules Glossary” for more\
about Speed as well as about special speeds, such as\
a Climb Speed, Fly Speed, or Swim Speed.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_movement_and_position ` |
| ` implementation ` | ` dnd2024:rules_combat_movement_and_position ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` on_own_turn ` | ` true ` |
| ` minimum_distance ` | ` 0 ` |
| ` mixed_modes_allowed ` | ` ["walk", "climb", "crawl", "jump", "swim"] ` |
| ` spend_budget_per_segment ` | ` true ` |
| ` special_speed_modes ` | ` ["climb", "fly", "swim"] ` |

#### Movement budget

<!-- splat:values path=splat_rule/parameters/movement_budget -->

| Field | Value |
| --- | --- |
| ` var ` | ` subject.speed ` |

#### Speed source

<!-- splat:values path=splat_rule/parameters/speed_source -->

| Field | Value |
| --- | --- |
| ` character ` | ` character_creation ` |
| ` monster ` | ` stat_block ` |

#### Movement mode rules

<!-- splat:rows path=splat_rule/parameters/movement_mode_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/climbing ` |
| ` 1 ` | ` dnd2024:glossary/crawling ` |
| ` 2 ` | ` dnd2024:glossary/jumping ` |
| ` 3 ` | ` dnd2024:glossary/swimming ` |
| ` 4 ` | ` dnd2024:glossary/speed ` |
