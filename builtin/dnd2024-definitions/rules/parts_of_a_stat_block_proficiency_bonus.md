---
splat_schema: 1
splat_id: 6222314d-f514-5a90-add2-894fef32491a
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_proficiency_bonus
splat_name: Proficiency Bonus
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
---

# Proficiency Bonus

## Source text

<!-- SRD page 256 -->

A monster’s Proficiency Bonus (PB) is determined\
by its CR, as shown in the Proficiency Bonus by\
Challenge Rating table, and is reflected in saving\
throws, skills, and other statistics where the mon-\
ster’s exceptional aptitude is a factor.\
Proficiency Bonus by Challenge Rating\
CR PB CR PB\
0–4 +2 17–20 +6\
5–8 +3 21–24 +7\
9–12 +4 25–28 +8\
13–16 +5 29–30 +9\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_proficiency_bonus ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_proficiency_bonus ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` already_included_in_stat_block ` | ` ["saving_throws", "skills", "other_proficient_statistics"] ` |

#### Proficiency bonus by cr

<!-- splat:rows path=splat_rule/parameters/proficiency_bonus_by_cr -->

| Row | cr_min | cr_max | bonus |
| --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` 4 ` | ` 2 ` |
| ` 1 ` | ` 5 ` | ` 8 ` | ` 3 ` |
| ` 2 ` | ` 9 ` | ` 12 ` | ` 4 ` |
| ` 3 ` | ` 13 ` | ` 16 ` | ` 5 ` |
| ` 4 ` | ` 17 ` | ` 20 ` | ` 6 ` |
| ` 5 ` | ` 21 ` | ` 24 ` | ` 7 ` |
| ` 6 ` | ` 25 ` | ` 28 ` | ` 8 ` |
| ` 7 ` | ` 29 ` | ` 30 ` | ` 9 ` |
