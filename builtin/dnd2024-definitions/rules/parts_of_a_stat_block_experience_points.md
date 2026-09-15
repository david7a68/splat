---
splat_schema: 1
splat_id: d1bc292c-ffb9-56fa-8f3a-db0d56b6233d
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_experience_points
splat_name: Experience Points
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
---

# Experience Points

## Source text

<!-- SRD page 256 -->

The number of Experience Points (XP) a monster\
is worth is based on its CR, as detailed in the Ex-\
perience Points by Challenge Rating table. XP is\
awarded for defeating the monster in combat or\
otherwise neutralizing it.\
Unless a rule says otherwise, a monster sum-\
moned by a spell or another magical ability is worth\
the XP noted in its stat block.\
Experience Points by Challenge Rating\
CR XP CR XP\
0 0 or 10 14 11,500\
1/8 25 15 13,000\
1/4 50 16 15,000\
1/2 100 17 18,000\
1 200 18 20,000\
2 450 19 22,000\
3 700 20 25,000\
4 1,100 21 33,000\
5 1,800 22 41,000\
6 2,300 23 50,000\
7 2,900 24 62,000\
8 3,900 25 75,000\
9 5,000 26 90,000\
10 5,900 27 105,000\
CR XP CR XP\
11 7,200 28 120,000\
12 8,400 29 135,000\
13 10,000 30 155,000\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_experience_points ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_experience_points ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cr_zero_xp_options ` | ` [0, 10] ` |
| ` award_events ` | ` ["defeated_in_combat", "otherwise_neutralized"] ` |
| ` magically_summoned_xp ` | ` stat_block_unless_overridden ` |

#### Xp table

<!-- splat:rows path=splat_rule/parameters/xp_table -->

| Row | cr | xp |
| --- | --- | --- |
| ` 0 ` | ` 0 ` | ` null ` |
| ` 1 ` | ` 0.125 ` | ` 25 ` |
| ` 2 ` | ` 0.25 ` | ` 50 ` |
| ` 3 ` | ` 0.5 ` | ` 100 ` |
| ` 4 ` | ` 1 ` | ` 200 ` |
| ` 5 ` | ` 2 ` | ` 450 ` |
| ` 6 ` | ` 3 ` | ` 700 ` |
| ` 7 ` | ` 4 ` | ` 1100 ` |
| ` 8 ` | ` 5 ` | ` 1800 ` |
| ` 9 ` | ` 6 ` | ` 2300 ` |
| ` 10 ` | ` 7 ` | ` 2900 ` |
| ` 11 ` | ` 8 ` | ` 3900 ` |
| ` 12 ` | ` 9 ` | ` 5000 ` |
| ` 13 ` | ` 10 ` | ` 5900 ` |
| ` 14 ` | ` 11 ` | ` 7200 ` |
| ` 15 ` | ` 12 ` | ` 8400 ` |
| ` 16 ` | ` 13 ` | ` 10000 ` |
| ` 17 ` | ` 14 ` | ` 11500 ` |
| ` 18 ` | ` 15 ` | ` 13000 ` |
| ` 19 ` | ` 16 ` | ` 15000 ` |
| ` 20 ` | ` 17 ` | ` 18000 ` |
| ` 21 ` | ` 18 ` | ` 20000 ` |
| ` 22 ` | ` 19 ` | ` 22000 ` |
| ` 23 ` | ` 20 ` | ` 25000 ` |
| ` 24 ` | ` 21 ` | ` 33000 ` |
| ` 25 ` | ` 22 ` | ` 41000 ` |
| ` 26 ` | ` 23 ` | ` 50000 ` |
| ` 27 ` | ` 24 ` | ` 62000 ` |
| ` 28 ` | ` 25 ` | ` 75000 ` |
| ` 29 ` | ` 26 ` | ` 90000 ` |
| ` 30 ` | ` 27 ` | ` 105000 ` |
| ` 31 ` | ` 28 ` | ` 120000 ` |
| ` 32 ` | ` 29 ` | ` 135000 ` |
| ` 33 ` | ` 30 ` | ` 155000 ` |
