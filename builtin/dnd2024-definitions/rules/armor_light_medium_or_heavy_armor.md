---
splat_schema: 1
splat_id: e4fcdad4-0fdb-52b8-a649-c02b37c242f8
splat_kind: rule
splat_key: dnd2024:rules/armor_light_medium_or_heavy_armor
splat_name: Light, Medium, or Heavy Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
---

# Light, Medium, or Heavy Armor

## Source text

<!-- SRD page 92 -->

If you wear Light, Medium, or Heavy armor and lack training with it, you have Disadvantage on any D20 Test that involves Strength or Dexterity, and you can’t cast spells.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/armor_light_medium_or_heavy_armor ` |
| ` implementation ` | ` dnd2024:rule_armor_light_medium_or_heavy_armor ` |

### Parameters

#### When

<!-- splat:values path=splat_rule/parameters/when -->

| Field | Value |
| --- | --- |
| ` wearing_armor_category ` | ` ["light", "medium", "heavy"] ` |
| ` trained_in_worn_armor ` | ` false ` |

#### Effects

<!-- splat:values path=splat_rule/parameters/effects -->

| Field | Value |
| --- | --- |
| ` d20_tests_using ` | ` ["strength", "dexterity"] ` |
| ` roll_mode ` | ` disadvantage ` |
| ` spellcasting_allowed ` | ` false ` |
