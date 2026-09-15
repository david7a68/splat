---
splat_schema: 1
splat_id: f626022b-7821-5165-9050-3b82bc627e72
splat_kind: rule
splat_key: dnd2024:glossary/temporary_hit_points
splat_name: Temporary Hit Points
splat_origin:
  key: dnd2024:reference/glossary/temporary_hit_points
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
---

# Temporary Hit Points

## Source text

<!-- SRD page 190 -->

Temporary Hit Points are granted by certain effects\
and act as a buffer against losing real Hit Points. See\
also “Playing the Game” (“Damage and Healing”).\

Original layout: [[../../srd-5.2.1/pages/190|page 190]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/temporary_hit_points ` |
| ` implementation ` | ` dnd2024:glossary_temporary_hit_points ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` resource ` | ` temporary_hit_points ` |
| ` grant_source ` | ` effect_defined ` |

#### Damage absorption

<!-- splat:values path=splat_rule/parameters/damage_absorption -->

| Field | Value |
| --- | --- |
| ` before ` | ` real_hit_points ` |

##### Amount

<!-- splat:values path=splat_rule/parameters/damage_absorption/amount -->

| Field | Value |
| --- | --- |
| ` op ` | ` min ` |

###### Args

<!-- splat:table
path: splat_rule/parameters/damage_absorption/amount/args
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
| ` row_1 ` | ` Temporary hit points ` |
| ` row_2 ` | ` Incoming damage ` |
