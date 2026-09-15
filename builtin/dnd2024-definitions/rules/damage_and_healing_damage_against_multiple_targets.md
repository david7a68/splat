---
splat_schema: 1
splat_id: 26ef7018-a0f9-5fcb-b61c-9e615779d05f
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_damage_against_multiple_targets
splat_name: Damage against Multiple Targets
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
---

# Damage against Multiple Targets

## Source text

<!-- SRD page 16 -->

When you create a damaging effect that forces two\
or more targets to make saving throws against it at\
the same time, roll the damage once for all the tar-\
gets. For example, when a wizard casts Fireball, the\
spell’s damage is rolled once for all creatures caught\
in the blast.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_damage_against_multiple_targets ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_damage_against_multiple_targets ` |

### Parameters

#### Shared damage roll

<!-- splat:values path=splat_rule/parameters/shared_damage_roll -->

| Field | Value |
| --- | --- |
| ` minimum_targets ` | ` 2 ` |
| ` requires_simultaneous_saving_throws ` | ` true ` |
| ` roll_count ` | ` 1 ` |
