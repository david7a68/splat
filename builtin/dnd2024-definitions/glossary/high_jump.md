---
splat_schema: 1
splat_id: f704be79-b52a-5cf3-af24-407a3caf1ee4
splat_kind: rule
splat_key: dnd2024:glossary/high_jump
splat_name: High Jump
splat_origin:
  key: dnd2024:reference/glossary/high_jump
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
---

# High Jump

## Source text

<!-- SRD page 183 -->

When you make a High Jump, you leap into the air a number of feet equal to 3 plus your Strength modifier (minimum of 0 feet) if you move at least 10 feet on foot immediately before the jump. When you make a standing High Jump, you can jump only half that distance. Either way, each foot of the jump costs a foot of movement.

You can extend your arms half your height above yourself during the jump. Thus, you can reach a distance equal to the height of the jump plus 1½ times your height.

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/high_jump ` |
| ` implementation ` | ` dnd2024:glossary_high_jump ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` direction ` | ` vertical ` |
| ` standing_height_multiplier ` | ` 0.5 ` |
| ` movement_cost_per_foot ` | ` 1 ` |
| ` arms_extension_height_multiplier ` | ` 0.5 ` |

#### Running start

<!-- splat:values path=splat_rule/parameters/running_start -->

| Field | Value |
| --- | --- |
| ` minimum_feet ` | ` 10 ` |
| ` on_foot ` | ` true ` |
| ` immediately_before ` | ` true ` |

#### Running height feet

<!-- splat:yaml path=splat_rule/parameters/running_height_feet -->

```yaml
op: max
args:
- 0
- op: add
  args:
  - 3
  - var: strength_modifier
```

#### Maximum reach

<!-- splat:yaml path=splat_rule/parameters/maximum_reach -->

```yaml
op: add
args:
- var: jump_height_feet
- op: multiply
  args:
  - 1.5
  - var: creature_height_feet
```
