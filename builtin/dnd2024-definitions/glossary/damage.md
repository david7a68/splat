---
splat_schema: 2
splat_id: 73579bfa-70aa-5fd8-9085-b1cc03856533
splat_kind: rule
splat_key: dnd2024:glossary/damage
splat_name: Damage
splat_origin: dnd2024:reference/glossary/damage
splat_sources:
- SRD 5.2.1, pp. 180, https://www.dndbeyond.com/srd
---

# Damage

## Source text

<!-- SRD page 180 -->

Damage represents harm that causes a creature or an object to lose Hit Points.

Original layout: [[../../srd-5.2.1/pages/180|page 180]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/damage ` |
| ` implementation ` | ` dnd2024:glossary_damage ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` targets ` | ` ["creature", "object"] ` |

#### Hit points change

<!-- splat:yaml path=splat_rule/parameters/hit_points_change -->

```yaml
op: negate
args:
- var: damage_amount
```
