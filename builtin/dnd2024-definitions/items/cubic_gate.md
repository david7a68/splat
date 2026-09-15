---
splat_schema: 1
splat_id: e5de36e0-984b-5e2f-a757-f53366ad3ebc
splat_kind: item
splat_key: dnd2024:items/cubic_gate
splat_name: Cubic Gate
splat_origin:
  key: dnd2024:reference/magic_item/cubic_gate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
---

# Cubic Gate

## Source text

<!-- SRD page 216 -->

Wondrous Item, Legendary\
This cube is 3 inches across and radiates palpable\
magical energy. The six sides of the cube are each\
keyed to a different plane of existence, one of which\
is the Material Plane. The other sides are linked to\
planes determined by the GM.\
The cube has 3 charges and regains 1d3 expended\
charges daily at dawn. As a Magic action, you can\
expend 1 of the cube’s charges to cast one of the fol-\
lowing spells using the cube.\
Gate. Pressing one side of the cube, you cast Gate,\
opening a portal to the plane of existence keyed to\
that side.\
Plane Shift. Pressing one side of the cube twice,\
you cast Plane Shift, transporting the targets to the\
plane of existence keyed to that side.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].

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
| ` mechanic ` | ` dnd2024:items/cubic_gate ` |
| ` implementation ` | ` dnd2024:item_cubic_gate ` |

### Parameters

#### Dimensions

<!-- splat:values path=splat_rule/parameters/dimensions -->

| Field | Value |
| --- | --- |
| ` side_inches ` | ` 3 ` |

#### Planes

<!-- splat:values path=splat_rule/parameters/planes -->

| Field | Value |
| --- | --- |
| ` side_count ` | ` 6 ` |
| ` all_distinct ` | ` true ` |
| ` fixed ` | ` ["material"] ` |
| ` gm_choices ` | ` 5 ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    dice: 1
    sides: 3
    bonus: 0
```

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
cost: 1
options:
- spell:
    key: dnd2024:spells/gate
  presses: 1
  destination: selected_side_plane
- spell:
    key: dnd2024:spells/plane_shift
  presses: 2
  destination: selected_side_plane
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
