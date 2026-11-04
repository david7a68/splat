---
splat_schema: 2
splat_id: e5de36e0-984b-5e2f-a757-f53366ad3ebc
splat_kind: item
splat_key: dnd2024:items/cubic_gate
splat_name: Cubic Gate
splat_origin: dnd2024:reference/magic_item/cubic_gate
splat_sources:
- SRD 5.2.1, pp. 216, https://www.dndbeyond.com/srd
---

# Cubic Gate


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Legendary |
| Attunement | No |


## Source text

<!-- SRD page 216 -->

Wondrous Item, Legendary

This cube is 3 inches across and radiates palpable magical energy. The six sides of the cube are each keyed to a different plane of existence, one of which is the Material Plane. The other sides are linked to planes determined by the GM.

The cube has 3 charges and regains 1d3 expended charges daily at dawn. As a Magic action, you can expend 1 of the cube’s charges to cast one of the following spells using the cube.

Gate. Pressing one side of the cube, you cast Gate, opening a portal to the plane of existence keyed to that side.

Plane Shift. Pressing one side of the cube twice, you cast Plane Shift, transporting the targets to the plane of existence keyed to that side.

Original layout: [[../../srd-5.2.1/pages/216|page 216]].





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

<!-- splat:values path=splat_rule/parameters/activation -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` cost ` | ` 1 ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/activation/options
key: Spell
order:
- gate
- plane_shift
columns:
  Spell:
    path: spell
    type: reference
  Presses:
    path: presses
    type: integer
  Destination:
    path: destination
    type: symbol
-->

| Spell | Presses | Destination |
| --- | --- | --- |
| [Gate](<../spells/gate.md> "dnd2024:spells/gate") | ` 1 ` | ` Selected side plane ` |
| [Plane Shift](<../spells/plane_shift.md> "dnd2024:spells/plane_shift") | ` 2 ` | ` Selected side plane ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
