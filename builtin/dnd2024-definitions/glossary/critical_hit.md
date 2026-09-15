---
splat_schema: 1
splat_id: d32266ca-739a-54c6-98ef-e56edb3f1fb8
splat_kind: rule
splat_key: dnd2024:glossary/critical_hit
splat_name: Critical Hit
splat_origin:
  key: dnd2024:reference/glossary/critical_hit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
---

# Critical Hit

## Source text

<!-- SRD page 179 -->

If you roll a 20 on the d20 for an attack roll, you\
score a Critical Hit, and the attack hits regardless of\
any modifiers or the target’s AC. A Critical Hit lets\
you roll extra dice for the attack’s damage against\
the target. Roll all of the attack’s damage dice twice\
and add them together. Then add any relevant mod-\
ifiers. See also “Playing the Game” (“Damage and\
Healing”).\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/critical_hit ` |
| ` implementation ` | ` dnd2024:glossary_critical_hit ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` natural_attack_die_trigger ` | ` 20 ` |
| ` automatic_hit ` | ` true ` |
| ` ignore_ac_and_modifiers_for_hit ` | ` true ` |
| ` damage_dice_multiplier ` | ` 2 ` |
| ` includes_all_attack_damage_dice ` | ` true ` |
| ` damage_modifiers_multiplier ` | ` 1 ` |
