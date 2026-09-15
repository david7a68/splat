---
splat_schema: 1
splat_id: 6c9e1405-4d99-56f7-b83c-7fe4c9c964e1
splat_kind: rule
splat_key: dnd2024:glossary/d20_test
splat_name: D20 Test
splat_origin:
  key: dnd2024:reference/glossary/d20_test
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
---

# D20 Test

## Source text

<!-- SRD page 180 -->

D20 Tests encompass the three main d20 rolls of\
the game: ability checks, attack rolls, and saving\
throws. If something in the game affects D20 Tests,\
it affects all three of these rolls. The GM determines\
whether a D20 Test is warranted in a given circum-\
stance. See also “Playing the Game” (“D20 Tests”).\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/d20_test ` |
| ` implementation ` | ` dnd2024:glossary_d20_test ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` test_kinds ` | ` ["ability_check", "attack_roll", "saving_throw"] ` |
| ` d20_test_modifiers_apply_to_all_kinds ` | ` true ` |

#### Test warranted

<!-- splat:values path=splat_rule/parameters/test_warranted -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` options ` | ` [true, false] ` |
