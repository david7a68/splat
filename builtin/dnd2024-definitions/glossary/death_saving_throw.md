---
splat_schema: 1
splat_id: 247390f9-a645-5c90-899a-a1976cf3cb49
splat_kind: rule
splat_key: dnd2024:glossary/death_saving_throw
splat_name: Death Saving Throw
splat_origin:
  key: dnd2024:reference/glossary/death_saving_throw
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
---

# Death Saving Throw

## Source text

<!-- SRD page 181 -->

A player character must make a Death Saving\
Throw (also called a Death Save) if they start their\
turn with 0 Hit Points. See also “Playing the Game”\
(“Damage and Healing”).\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/death_saving_throw ` |
| ` implementation ` | ` dnd2024:glossary_death_saving_throw ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` own_turn_start ` |
| ` subject_kind ` | ` player_character ` |
| ` hit_points ` | ` 0 ` |
| ` required_test ` | ` death_saving_throw ` |

#### Exception rule

<!-- splat:values path=splat_rule/parameters/exception_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/stable ` |
