---
splat_schema: 2
splat_id: c23ce46c-b3d2-57a7-868b-3c9adfecf599
splat_kind: rule
splat_key: dnd2024:glossary/damage_roll
splat_name: Damage Roll
splat_origin: dnd2024:reference/glossary/damage_roll
splat_sources:
- SRD 5.2.1, pp. 180, https://www.dndbeyond.com/srd
---

# Damage Roll

## Source text

<!-- SRD page 180 -->

A damage roll is a die roll, adjusted by any applicable modifiers, that deals damage to a target. See also “Playing the Game” (“Damage and Healing”).

Original layout: [[../../srd-5.2.1/pages/180|page 180]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/damage_roll ` |
| ` implementation ` | ` dnd2024:glossary_damage_roll ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` apply_to ` | ` target ` |

#### Amount

<!-- splat:values path=splat_rule/parameters/amount -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/amount/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Var:
    path: var
    type: symbol
-->

| Key | Var |
| --- | --- |
| ` row_1 ` | ` Rolled total ` |
| ` row_2 ` | ` Applicable modifiers ` |
