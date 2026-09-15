---
splat_schema: 1
splat_id: bf2814bb-c15d-51ad-aa57-af32a7a81ba7
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_speed
splat_name: Speed
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
---

# Speed

## Source text

<!-- SRD page 255 -->

The Speed entry specifies a monster’s Speed. Some\
monsters have one or more of the following speeds:\
Burrow, Climb, Fly, Swim. Rules for Speed and these\
specials speeds appear in “Rules Glossary.”\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_speed ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_speed ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` speed_fields ` | ` ["walk", "burrow", "climb", "fly", "swim"] ` |

#### Rules

<!-- splat:rows path=splat_rule/parameters/rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/speed ` |
| ` 1 ` | ` dnd2024:glossary/burrow_speed ` |
| ` 2 ` | ` dnd2024:glossary/climb_speed ` |
| ` 3 ` | ` dnd2024:glossary/fly_speed ` |
| ` 4 ` | ` dnd2024:glossary/swim_speed ` |
