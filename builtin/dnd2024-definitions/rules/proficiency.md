---
splat_schema: 1
splat_id: 3294f9d1-667e-5099-b2b2-6669bf3273e8
splat_kind: rule
splat_key: dnd2024:rules/proficiency
splat_name: Proficiency
splat_origin:
  key: dnd2024:reference/pages/008
splat_sources:
- title: SRD 5.2.1
  locator: pp. 8
  url: https://www.dndbeyond.com/srd
---

# Proficiency

## Source text

<!-- SRD page 8 -->

Characters and monsters are good at various\
things. Some are skilled with many weapons, while\
others can use only a few. Some are better at under-\
standing people’s motives, and others are better at\
unlocking the secrets of the multiverse. All crea-\
tures have a Proficiency Bonus, which reflects the\
impact that training has on the creature’s capabil-\
ities. A character’s Proficiency Bonus increases as\
the character gains levels (described in “Character\
Creation”). A monster’s Proficiency Bonus is based\
on its Challenge Rating (see “Rules Glossary”). The\
Proficiency Bonus table shows how the bonus is\
determined.\
This bonus is applied to a D20 Test when the crea-\
ture has proficiency in a skill, in a saving throw, or\
with an item that the creature uses to make the D20\
Test. The bonus is also used for spell attacks and for\
calculating the DC of saving throws for spells.\
Proficiency Bonus\
Level or CR Bonus Level or CR Bonus\
Up to 4 +2 17–20 +6\
5–8 +3 21–24 +7\
9–12 +4 25–28 +8\
13–16 +5 29–30 +9\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/proficiency ` |
| ` implementation ` | ` dnd2024:rules_proficiency ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` applications ` | ` ["proficient_skill_test", "proficient_save", "proficient_item_test", "spell_attack", "spell_save_dc"] ` |

#### Basis

<!-- splat:values path=splat_rule/parameters/basis -->

| Field | Value |
| --- | --- |
| ` character ` | ` total_level ` |
| ` monster ` | ` challenge_rating ` |

#### Table

<!-- splat:rows path=splat_rule/parameters/table -->

| Row | minimum | maximum | bonus |
| --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` 4 ` | ` 2 ` |
| ` 1 ` | ` 5 ` | ` 8 ` | ` 3 ` |
| ` 2 ` | ` 9 ` | ` 12 ` | ` 4 ` |
| ` 3 ` | ` 13 ` | ` 16 ` | ` 5 ` |
| ` 4 ` | ` 17 ` | ` 20 ` | ` 6 ` |
| ` 5 ` | ` 21 ` | ` 24 ` | ` 7 ` |
| ` 6 ` | ` 25 ` | ` 28 ` | ` 8 ` |
| ` 7 ` | ` 29 ` | ` 30 ` | ` 9 ` |
