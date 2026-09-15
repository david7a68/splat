---
splat_schema: 1
splat_id: 95cc7ac0-5cbd-5c9f-b4c3-ef18cdb1b45b
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_saving_throw_effect_notation
splat_name: Saving Throw Effect Notation
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
---

# Saving Throw Effect Notation

## Source text

<!-- SRD page 256 -->

If an effect forces a saving throw, the effect identi-\
fies the kind of save required and then provides the\
save’s DC, a description of which creatures make\
the save, and an explanation of what happens on a\
failed or successful save.\
“Half damage only” on a successful save means\
the target takes half as much damage (round down)\
as targets that fail the save, while also ignoring all\
other parts of the effect.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_saving_throw_effect_notation ` |
| ` implementation ` | ` dnd2024:rule_parts_of_a_stat_block_saving_throw_effect_notation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` save_fields ` | ` ["ability", "dc", "target_eligibility", "failure_effect", "success_effect"] ` |

#### Half damage only

<!-- splat:values path=splat_rule/parameters/half_damage_only -->

| Field | Value |
| --- | --- |
| ` damage_fraction ` | ` 0.5 ` |
| ` rounding ` | ` floor ` |
| ` other_effects_applied ` | ` false ` |
