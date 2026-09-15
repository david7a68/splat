---
splat_schema: 1
splat_id: b8aaf636-c9ef-547d-b777-d2c8ab19ffaa
splat_kind: rule
splat_key: dnd2024:glossary/ability_score_and_modifier
splat_name: Ability Score and Modifier
splat_origin:
  key: dnd2024:reference/glossary/ability_score_and_modifier
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
---

# Ability Score and Modifier

## Source text

<!-- SRD page 176 -->

A creature has six ability scores—Strength, Dex-\
terity, Constitution, Intelligence, Wisdom, and\
Charisma—each of which has a corresponding mod-\
ifier. Add the modifier when you make a D20 Test\
with the corresponding ability or when a rule asks\
you to do so. See also “Playing the Game” (“The Six\
Abilities”).\

Original layout: [[../../srd-5.2.1/pages/176|page 176]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/ability_score_and_modifier ` |
| ` implementation ` | ` dnd2024:glossary_ability_score_and_modifier ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` abilities ` | ` ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"] ` |
| ` other_modifier_application ` | ` when_rule_requires ` |

#### D20 modifier

<!-- splat:yaml path=splat_rule/parameters/d20_modifier -->

```yaml
when: corresponding_ability_used
add:
  var: ability_modifier
```
