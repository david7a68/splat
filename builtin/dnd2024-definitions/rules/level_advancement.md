---
splat_schema: 1
splat_id: 6b13a834-95d3-54ba-8233-b94b19856c22
splat_kind: rule
splat_key: dnd2024:rules/level_advancement
splat_name: Level Advancement
splat_origin:
  key: dnd2024:reference/pages/023
splat_sources:
- title: SRD 5.2.1
  locator: pp. 23
  url: https://www.dndbeyond.com/srd
---

# Level Advancement

## Source text

<!-- SRD page 23 -->

While going on adventures, your character gains experience, represented by Experience Points (XP). A character who reaches a specified Experience Point total advances in capability. This advancement is called gaining a level.

The Character Advancement table lists the XP you need to advance to a level and the Proficiency Bonus for a character of that level. When your XP total equals or exceeds a number in the Experience Points column, you reach the corresponding level.

**Character Advancement**

| Level | Experience Points | Proficiency Bonus |
| --- | --- | --- |
| 1 | 0 | +2 |
| 2 | 300 | +2 |
| 3 | 900 | +2 |
| 4 | 2,700 | +2 |
| 5 | 6,500 | +3 |
| 6 | 14,000 | +3 |
| 7 | 23,000 | +3 |
| 8 | 34,000 | +3 |
| 9 | 48,000 | +4 |
| 10 | 64,000 | +4 |
| 11 | 85,000 | +4 |
| 12 | 100,000 | +4 |
| 13 | 120,000 | +5 |
| 14 | 140,000 | +5 |
| 15 | 165,000 | +5 |
| 16 | 195,000 | +5 |
| 17 | 225,000 | +6 |
| 18 | 265,000 | +6 |
| 19 | 305,000 | +6 |
| 20 | 355,000 | +6 |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/level_advancement ` |
| ` implementation ` | ` dnd2024:rules_level_advancement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` level_selection ` | ` highest_threshold_met ` |

#### Advancement

<!-- splat:table
path: splat_rule/parameters/advancement
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
- row_11
- row_12
- row_13
- row_14
- row_15
- row_16
- row_17
- row_18
- row_19
- row_20
columns:
  Key:
    type: key
  Level:
    path: level
    type: integer
  Minimum xp:
    path: minimum_xp
    type: integer
  Proficiency bonus:
    path: proficiency_bonus
    type: integer
-->

| Key | Level | Minimum xp | Proficiency bonus |
| --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 0 ` | ` 2 ` |
| ` row_2 ` | ` 2 ` | ` 300 ` | ` 2 ` |
| ` row_3 ` | ` 3 ` | ` 900 ` | ` 2 ` |
| ` row_4 ` | ` 4 ` | ` 2700 ` | ` 2 ` |
| ` row_5 ` | ` 5 ` | ` 6500 ` | ` 3 ` |
| ` row_6 ` | ` 6 ` | ` 14000 ` | ` 3 ` |
| ` row_7 ` | ` 7 ` | ` 23000 ` | ` 3 ` |
| ` row_8 ` | ` 8 ` | ` 34000 ` | ` 3 ` |
| ` row_9 ` | ` 9 ` | ` 48000 ` | ` 4 ` |
| ` row_10 ` | ` 10 ` | ` 64000 ` | ` 4 ` |
| ` row_11 ` | ` 11 ` | ` 85000 ` | ` 4 ` |
| ` row_12 ` | ` 12 ` | ` 100000 ` | ` 4 ` |
| ` row_13 ` | ` 13 ` | ` 120000 ` | ` 5 ` |
| ` row_14 ` | ` 14 ` | ` 140000 ` | ` 5 ` |
| ` row_15 ` | ` 15 ` | ` 165000 ` | ` 5 ` |
| ` row_16 ` | ` 16 ` | ` 195000 ` | ` 5 ` |
| ` row_17 ` | ` 17 ` | ` 225000 ` | ` 6 ` |
| ` row_18 ` | ` 18 ` | ` 265000 ` | ` 6 ` |
| ` row_19 ` | ` 19 ` | ` 305000 ` | ` 6 ` |
| ` row_20 ` | ` 20 ` | ` 355000 ` | ` 6 ` |
