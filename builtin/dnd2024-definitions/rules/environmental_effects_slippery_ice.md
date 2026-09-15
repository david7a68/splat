---
splat_schema: 1
splat_id: 1794c9f1-dec1-5bf7-a926-96724d9510fd
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_slippery_ice
splat_name: Slippery Ice
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
---

# Slippery Ice

## Source text

<!-- SRD page 195 -->

Slippery ice is Difficult Terrain. A creature that\
moves onto slippery ice for the first time on a turn\
or starts its turn there must succeed on a DC 10\
Dexterity saving throw or have the Prone condition.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/environmental_effects_slippery_ice ` |
| ` implementation ` | ` dnd2024:rules_environmental_effects_slippery_ice ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` terrain ` | ` difficult ` |
| ` triggers ` | ` ["first_entry_on_turn", "start_turn_inside"] ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` 10 ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` condition ` | ` prone ` |
