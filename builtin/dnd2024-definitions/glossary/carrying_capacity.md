---
splat_schema: 2
splat_id: e041315e-4ff5-595e-aea6-d711a3482fcd
splat_kind: rule
splat_key: dnd2024:glossary/carrying_capacity
splat_name: Carrying Capacity
splat_origin: dnd2024:reference/glossary/carrying_capacity
splat_sources:
- SRD 5.2.1, pp. 178, https://www.dndbeyond.com/srd
---

# Carrying Capacity

## Source text

<!-- SRD page 178 -->

Your size and Strength score determine the maximum weight in pounds that you can carry, as shown in the Carrying Capacity table. The table also shows the maximum weight you can drag, lift, or push.

While dragging, lifting, or pushing weight in excess of the maximum weight you can carry, your Speed can be no more than 5 feet.

**Carrying Capacity**

| Creature Size | Carry | Drag/Lift/Push |
| --- | --- | --- |
| Tiny | Str. × 7.5 lb. | Str. × 15 lb. |
| Small/Medium | Str. × 15 lb. | Str. × 30 lb. |
| Large | Str. × 30 lb. | Str. × 60 lb. |
| Huge | Str. × 60 lb. | Str. × 120 lb. |
| Gargantuan | Str. × 120 lb. | Str. × 240 lb. |

Original layout: [[../../srd-5.2.1/pages/178|page 178]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/carrying_capacity ` |
| ` implementation ` | ` dnd2024:glossary_carrying_capacity ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weight_unit ` | ` pound ` |

#### Carry strength multipliers

<!-- splat:values path=splat_rule/parameters/carry_strength_multipliers -->

| Field | Value |
| --- | --- |
| ` tiny ` | ` 7.5 ` |
| ` small ` | ` 15 ` |
| ` medium ` | ` 15 ` |
| ` large ` | ` 30 ` |
| ` huge ` | ` 60 ` |
| ` gargantuan ` | ` 120 ` |

#### Drag lift push strength multipliers

<!-- splat:values path=splat_rule/parameters/drag_lift_push_strength_multipliers -->

| Field | Value |
| --- | --- |
| ` tiny ` | ` 15 ` |
| ` small ` | ` 30 ` |
| ` medium ` | ` 30 ` |
| ` large ` | ` 60 ` |
| ` huge ` | ` 120 ` |
| ` gargantuan ` | ` 240 ` |

#### Over carry capacity

<!-- splat:values path=splat_rule/parameters/over_carry_capacity -->

| Field | Value |
| --- | --- |
| ` modes ` | ` ["drag", "lift", "push"] ` |
| ` speed_cap_feet ` | ` 5 ` |
