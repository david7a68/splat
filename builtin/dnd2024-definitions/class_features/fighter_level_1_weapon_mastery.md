---
splat_schema: 2
splat_id: 367ab48d-99fb-5a8b-9657-3c5fd6f1e72a
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_1_weapon_mastery
splat_name: 'Level 1: Weapon Mastery'
splat_origin: dnd2024:reference/pages/048
splat_sources:
- SRD 5.2.1, pp. 48, https://www.dndbeyond.com/srd
---

# Level 1: Weapon Mastery

## Source text

<!-- SRD page 48 -->

Your training with weapons allows you to use the mastery properties of three kinds of Simple or Martial weapons of your choice. Whenever you finish a Long Rest, you can practice weapon drills and change one of those weapon choices.

When you reach certain Fighter levels, you gain the ability to use the mastery properties of more kinds of weapons, as shown in the Weapon Mastery column of the Fighter Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_1_weapon_mastery ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_1_weapon_mastery ` |

### Parameters

#### Mastery choices

<!-- splat:yaml path=splat_rule/parameters/mastery_choices -->

```yaml
count:
  table:
    key: dnd2024:tables/fighter_progression
  row:
    var: actor.class_levels.fighter
  column: weapon_mastery
eligible:
  weapon_category:
  - simple
  - martial
replacement:
  event: rest.long_completed
  maximum: 1
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` minimum_level ` | ` 1 ` |
