---
splat_schema: 1
splat_id: ab984309-6e33-565e-a496-bdc20321584e
splat_kind: item
splat_key: dnd2024:items/universal_solvent
splat_name: Universal Solvent
splat_origin:
  key: dnd2024:reference/magic_item/universal_solvent
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
---

# Universal Solvent

## Source text

<!-- SRD page 250 -->

Wondrous Item, Legendary\
This tube holds milky liquid with a strong alcohol\
smell. When found, a tube contains 1d6 + 1 ounces.\
You can take a Utilize action to pour 1 or more\
ounces of solvent from the tube onto a surface\
within reach. Each ounce instantly dissolves up to 1\
square foot of adhesive it touches, including Sover-\
eign Glue.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/universal_solvent ` |
| ` implementation ` | ` dnd2024:item_universal_solvent ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_required ` | ` false ` |

#### Initial ounces

<!-- splat:yaml path=splat_rule/parameters/initial_ounces -->

```yaml
roll:
  count: 1
  sides: 6
  add: 1
```

#### Apply

<!-- splat:yaml path=splat_rule/parameters/apply -->

```yaml
action: utilize
ounces_minimum: 1
surface: within_reach
square_ft_adhesive_per_ounce: 1
effect_timing: instantaneous
operation: dissolve_adhesive
includes:
  key: dnd2024:items/sovereign_glue
consumes_applied_ounces: true
```
