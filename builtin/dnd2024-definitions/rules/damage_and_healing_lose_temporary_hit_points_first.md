---
splat_schema: 1
splat_id: 35969c95-4982-594e-a08b-627c6b2f1b2f
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_lose_temporary_hit_points_first
splat_name: Lose Temporary Hit Points First
splat_origin:
  key: dnd2024:reference/pages/018
splat_sources:
- title: SRD 5.2.1
  locator: pp. 18
  url: https://www.dndbeyond.com/srd
---

# Lose Temporary Hit Points First

## Source text

<!-- SRD page 18 -->

If you have Temporary Hit Points and take damage,\
those points are lost first, and any leftover damage\
carries over to your Hit Points. For example, if you\
have 5 Temporary Hit Points and take 7 damage,\
you lose those points and then lose 2 Hit Points.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_lose_temporary_hit_points_first ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_lose_temporary_hit_points_first ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_absorption_order ` | ` ["temporary_hp", "hp"] ` |

#### Temporary hp absorbed

<!-- splat:values path=splat_rule/parameters/temporary_hp_absorbed -->

| Field | Value |
| --- | --- |
| ` op ` | ` min ` |
| ` terms ` | ` ["damage", "temporary_hp"] ` |

#### Hp damage

<!-- splat:yaml path=splat_rule/parameters/hp_damage -->

```yaml
op: max
terms:
- 0
- op: subtract
  left: damage
  right: temporary_hp
```
