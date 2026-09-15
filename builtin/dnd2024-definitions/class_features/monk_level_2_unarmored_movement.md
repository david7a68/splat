---
splat_schema: 1
splat_id: 2e76de70-78de-55fc-b971-9d37390deb52
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_2_unarmored_movement
splat_name: 'Level 2: Unarmored Movement'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
---

# Level 2: Unarmored Movement

## Source text

<!-- SRD page 51 -->

Your speed increases by 10 feet while you aren’t\
wearing armor or wielding a Shield. This bonus\
increases when you reach certain Monk levels, as\
shown on the Monk Features table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_2_unarmored_movement ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_2_unarmored_movement ` |

### Parameters

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` armor_worn ` | ` false ` |
| ` shield_wielded ` | ` false ` |

#### Speed bonus feet

<!-- splat:yaml path=splat_rule/parameters/speed_bonus_feet -->

```yaml
table:
  key: dnd2024:tables/monk_progression
row:
  var: actor.class_levels.monk
column: unarmored_movement
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 2 ` |
