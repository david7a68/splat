---
splat_schema: 1
splat_id: 57ac56de-6e36-5dd6-be4a-7c8729db2ac1
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_critical_hits
splat_name: Critical Hits
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
---

# Critical Hits

## Source text

<!-- SRD page 16 -->

When you score a Critical Hit, you deal extra dam-\
age. Roll the attack’s damage dice twice, add them\
together, and add any relevant modifiers as normal.\
For example, if you score a Critical Hit with a Dag-\
ger, roll 2d4 for the damage rather than 1d4, and\
add your relevant ability modifier. If the attack in-\
volves other damage dice, such as from the Rogue’s\
Sneak Attack feature, you also roll those dice twice.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_critical_hits ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_critical_hits ` |

### Parameters

#### Critical hit

<!-- splat:values path=splat_rule/parameters/critical_hit -->

| Field | Value |
| --- | --- |
| ` damage_dice_multiplier ` | ` 2 ` |
| ` include_additional_attack_dice ` | ` true ` |
| ` flat_modifiers_multiplier ` | ` 1 ` |
