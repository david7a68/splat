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

While going on adventures, your character gains ex-\
perience, represented by Experience Points (XP). A\
character who reaches a specified Experience Point\
total advances in capability. This advancement is\
called gaining a level.\
The Character Advancement table lists the XP\
you need to advance to a level and the Proficiency\
Bonus for a character of that level. When your XP\
total equals or exceeds a number in the Experience\
Points column, you reach the corresponding level.\
Character Advancement\
Level Experience Points Proficiency Bonus\
1 0 +2\
2 300 +2\
3 900 +2\
4 2,700 +2\
5 6,500 +3\
6 14,000 +3\
7 23,000 +3\
8 34,000 +3\
9 48,000 +4\
10 64,000 +4\
11 85,000 +4\
12 100,000 +4\
13 120,000 +5\
14 140,000 +5\
15 165,000 +5\
16 195,000 +5\
17 225,000 +6\
18 265,000 +6\
19 305,000 +6\
20 355,000 +6\

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

<!-- splat:rows path=splat_rule/parameters/advancement -->

| Row | level | minimum_xp | proficiency_bonus |
| --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 0 ` | ` 2 ` |
| ` 1 ` | ` 2 ` | ` 300 ` | ` 2 ` |
| ` 2 ` | ` 3 ` | ` 900 ` | ` 2 ` |
| ` 3 ` | ` 4 ` | ` 2700 ` | ` 2 ` |
| ` 4 ` | ` 5 ` | ` 6500 ` | ` 3 ` |
| ` 5 ` | ` 6 ` | ` 14000 ` | ` 3 ` |
| ` 6 ` | ` 7 ` | ` 23000 ` | ` 3 ` |
| ` 7 ` | ` 8 ` | ` 34000 ` | ` 3 ` |
| ` 8 ` | ` 9 ` | ` 48000 ` | ` 4 ` |
| ` 9 ` | ` 10 ` | ` 64000 ` | ` 4 ` |
| ` 10 ` | ` 11 ` | ` 85000 ` | ` 4 ` |
| ` 11 ` | ` 12 ` | ` 100000 ` | ` 4 ` |
| ` 12 ` | ` 13 ` | ` 120000 ` | ` 5 ` |
| ` 13 ` | ` 14 ` | ` 140000 ` | ` 5 ` |
| ` 14 ` | ` 15 ` | ` 165000 ` | ` 5 ` |
| ` 15 ` | ` 16 ` | ` 195000 ` | ` 5 ` |
| ` 16 ` | ` 17 ` | ` 225000 ` | ` 6 ` |
| ` 17 ` | ` 18 ` | ` 265000 ` | ` 6 ` |
| ` 18 ` | ` 19 ` | ` 305000 ` | ` 6 ` |
| ` 19 ` | ` 20 ` | ` 355000 ` | ` 6 ` |
