---
splat_schema: 1
splat_id: b32a75fd-5ca0-5dbc-b4a7-29e38ed82236
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_3_preserve_life
splat_name: 'Level 3: Preserve Life'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
---

# Level 3: Preserve Life

## Source text

<!-- SRD page 40 -->

As a Magic action, you present your Holy Symbol and expend a use of your Channel Divinity to evoke healing energy that can restore a number of Hit Points equal to five times your Cleric level. Choose Bloodied creatures within 30 feet of yourself (which can include you), and divide those Hit Points among them. This feature can restore a creature to no more than half its Hit Point maximum.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/life_domain_level_3_preserve_life ` |
| ` implementation ` | ` dnd2024:class_features_life_domain_level_3_preserve_life ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` requires_item ` | ` holy_symbol ` |

#### Resource cost

<!-- splat:values path=splat_rule/parameters/resource_cost -->

| Field | Value |
| --- | --- |
| ` id ` | ` cleric.channel_divinity ` |
| ` amount ` | ` 1 ` |

#### Healing pool

<!-- splat:yaml path=splat_rule/parameters/healing_pool -->

```yaml
op: multiply
args:
- 5
- var: actor.class_levels.cleric
```

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` bloodied ` | ` true ` |
| ` range_feet ` | ` 30 ` |
| ` self_allowed ` | ` true ` |
| ` selection ` | ` actor_choice ` |

#### Distribution

<!-- splat:yaml path=splat_rule/parameters/distribution -->

```yaml
chooser: actor
type: integer_allocation
sum_maximum: healing_pool
per_target_hp_cap:
  op: multiply
  args:
  - var: target.hit_point_maximum
  - 0.5
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` minimum_level ` | ` 3 ` |
