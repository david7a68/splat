---
splat_schema: 1
splat_id: 5a4d8879-58d0-523b-8bcc-ea6ca8f3490f
splat_kind: rule
splat_key: dnd2024:glossary/grappled_condition
splat_name: Grappled [Condition]
splat_origin:
  key: dnd2024:reference/glossary/grappled_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
---

# Grappled [Condition]

## Source text

<!-- SRD page 182 -->

While you have the Grappled condition, you experi-\
ence the following effects.\
Speed 0. Your Speed is 0 and can’t increase.\
Attacks Affected. You have Disadvantage on at-\
tack rolls against any target other than the grappler.\
Movable. The grappler can drag or carry you\
when it moves, but every foot of movement costs it 1\
extra foot unless you are Tiny or two or more sizes\
smaller than it.\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/grappled_condition ` |
| ` implementation ` | ` dnd2024:glossary_grappled_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` grappled ` |
| ` speed ` | ` 0 ` |
| ` speed_can_increase ` | ` false ` |
| ` attack_disadvantage_unless_target_is_grappler ` | ` true ` |

#### Grappler moves subject

<!-- splat:yaml path=splat_rule/parameters/grappler_moves_subject -->

```yaml
allowed_modes:
- drag
- carry
extra_cost_per_foot: 1
extra_cost_exceptions:
- target_size: tiny
- target_size_at_least_categories_smaller: 2
```
