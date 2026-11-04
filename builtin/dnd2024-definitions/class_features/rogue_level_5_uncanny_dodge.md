---
splat_schema: 2
splat_id: dd38f4d0-ae83-5739-92d4-6507a1d0e480
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_5_uncanny_dodge
splat_name: 'Level 5: Uncanny Dodge'
splat_origin: dnd2024:reference/pages/063
splat_sources:
- SRD 5.2.1, pp. 63, https://www.dndbeyond.com/srd
---

# Level 5: Uncanny Dodge

## Source text

<!-- SRD page 63 -->

When an attacker that you can see hits you with an attack roll, you can take a Reaction to halve the attack’s damage against you (round down).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_5_uncanny_dodge ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_5_uncanny_dodge ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` reaction ` |
| ` trigger ` | ` visible_attacker.attack_hits_actor ` |
| ` damage_multiplier ` | ` 0.5 ` |
| ` rounding ` | ` down ` |
| ` scope ` | ` trigger_attack.damage_against_actor ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 5 ` |
