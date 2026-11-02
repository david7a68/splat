---
splat_schema: 1
splat_id: 89220c1c-6611-5e59-b721-27175e19516a
splat_kind: rule
splat_key: dnd2024:rules/warlock_one_with_shadows
splat_name: One with Shadows
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
---

# One with Shadows

## Source text

<!-- SRD page 74 -->

Prerequisite: Level 5+ Warlock

While you’re in an area of Dim Light or Darkness, you can cast Invisibility on yourself without expending a spell slot.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_one_with_shadows ` |
| ` implementation ` | ` dnd2024:rules_warlock_one_with_shadows ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` activation_illumination ` | ` ["dim_light", "darkness"] ` |
| ` target ` | ` self ` |
| ` spell_slot_cost ` | ` 0 ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 5 ` |

#### Spell

<!-- splat:values path=splat_rule/parameters/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/invisibility ` |
