---
splat_schema: 1
splat_id: d61bdb61-79ee-54bc-a351-233ed15ee7ae
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_1_favored_enemy
splat_name: 'Level 1: Favored Enemy'
splat_origin:
  key: dnd2024:reference/pages/058
splat_sources:
- title: SRD 5.2.1
  locator: pp. 58
  url: https://www.dndbeyond.com/srd
---

# Level 1: Favored Enemy

## Source text

<!-- SRD page 58 -->

You always have the Hunter’s Mark spell prepared. You can cast it twice without expending a spell slot, and you regain all expended uses of this ability when you finish a Long Rest.

The number of times you can cast the spell without a spell slot increases when you reach certain Ranger levels, as shown in the Favored Enemy column of the Ranger Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_1_favored_enemy ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_1_favored_enemy ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` counts_against_prepared_limit ` | ` false ` |

#### Always prepared

<!-- splat:rows path=splat_rule/parameters/always_prepared -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/hunter_s_mark ` |

#### Free cast

<!-- splat:yaml path=splat_rule/parameters/free_cast -->

```yaml
resource:
  id: ranger.favored_enemy
  maximum:
    table:
      key: dnd2024:tables/ranger_progression
    row:
      var: actor.class_levels.ranger
    column: favored_enemy
  recovery:
  - event: rest.long_completed
    amount: all
cost: 1
expend_slot: false
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` minimum_level ` | ` 1 ` |
