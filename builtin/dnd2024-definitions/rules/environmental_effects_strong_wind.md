---
splat_schema: 1
splat_id: cc79a993-f37a-5d1f-8fa0-80815f9f7787
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_strong_wind
splat_name: Strong Wind
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
---

# Strong Wind

## Source text

<!-- SRD page 195 -->

Strong wind imposes Disadvantage on ranged attack rolls with weapons. It also extinguishes open flames and disperses fog. A flying creature in a strong wind must land at the end of its turn or fall.

A strong wind in a desert can create a sandstorm that imposes Disadvantage on Wisdom (Perception) checks.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/environmental_effects_strong_wind ` |
| ` implementation ` | ` dnd2024:rule_environmental_effects_strong_wind ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ranged_weapon_attacks ` | ` disadvantage ` |
| ` extinguish_open_flames ` | ` true ` |
| ` disperse_fog ` | ` true ` |

#### Flying creature end turn

<!-- splat:values path=splat_rule/parameters/flying_creature_end_turn -->

| Field | Value |
| --- | --- |
| ` must ` | ` land ` |
| ` otherwise ` | ` fall ` |

#### Desert sandstorm

<!-- splat:values path=splat_rule/parameters/desert_sandstorm -->

| Field | Value |
| --- | --- |
| ` gm_option ` | ` true ` |
| ` wisdom_perception ` | ` disadvantage ` |
