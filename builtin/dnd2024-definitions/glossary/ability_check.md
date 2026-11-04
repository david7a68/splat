---
splat_schema: 2
splat_id: 4ce1c2c7-f289-5518-80fc-0fabc2dac658
splat_kind: rule
splat_key: dnd2024:glossary/ability_check
splat_name: Ability Check
splat_origin: dnd2024:reference/glossary/ability_check
splat_sources:
- SRD 5.2.1, pp. 176, https://www.dndbeyond.com/srd
---

# Ability Check

## Source text

<!-- SRD page 176 -->

An ability check is a D20 Test that represents using one of the six abilities—or a specific skill associated with an ability—to overcome a challenge. See also “Playing the Game” (“D20 Tests” and “Proficiency”).

Original layout: [[../../srd-5.2.1/pages/176|page 176]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/ability_check ` |
| ` implementation ` | ` dnd2024:glossary_ability_check ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` test_kind ` | ` ability_check ` |
| ` abilities ` | ` ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"] ` |
| ` skill_optional ` | ` true ` |

#### Base rule

<!-- splat:values path=splat_rule/parameters/base_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/d20_test ` |
