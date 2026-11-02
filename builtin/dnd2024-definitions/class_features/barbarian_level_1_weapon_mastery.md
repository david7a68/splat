---
splat_schema: 1
splat_id: 70bc35fb-2e4a-55a0-80b8-6cafa38bcb7c
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_1_weapon_mastery
splat_name: 'Level 1: Weapon Mastery'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
---

# Level 1: Weapon Mastery

## Source text

<!-- SRD page 29 -->

Your training with weapons allows you to use the mastery properties of two kinds of Simple or Martial Melee weapons of your choice, such as Greataxes and Handaxes. Whenever you finish a Long Rest, you can practice weapon drills and change one of those weapon choices.

When you reach certain Barbarian levels, you gain the ability to use the mastery properties of more kinds of weapons, as shown in the Weapon Mastery column of the Barbarian Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_1_weapon_mastery ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_1_weapon_mastery ` |

### Parameters

#### Mastery choices

<!-- splat:yaml path=splat_rule/parameters/mastery_choices -->

```yaml
count:
  table:
    key: dnd2024:tables/barbarian_progression
  row:
    var: actor.class_levels.barbarian
  column: weapon_mastery
eligible:
  weapon_category:
  - simple
  - martial
  weapon_range: melee
replacement:
  event: rest.long_completed
  maximum: 1
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 1 ` |
