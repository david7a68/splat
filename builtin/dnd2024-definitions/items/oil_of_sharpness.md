---
splat_schema: 1
splat_id: bd98da33-393a-5fcd-8607-15d564659eeb
splat_kind: item
splat_key: dnd2024:items/oil_of_sharpness
splat_name: Oil of Sharpness
splat_origin:
  key: dnd2024:reference/magic_item/oil_of_sharpness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233
  url: https://www.dndbeyond.com/srd
---

# Oil of Sharpness

## Source text

<!-- SRD page 233 -->

Potion, Very Rare\
One vial of this oil can coat one Melee weapon or\
twenty pieces of ammunition, but only ammunition\
and Melee weapons that are nonmagical and deal\
Slashing or Piercing damage are affected. Applying\
the oil takes 1 minute, after which the oil magically\
seeps into whatever it coats, turning the coated\
weapon into a +3 Weapon or the coated ammunition\
into +3 Ammunition.\
This clear, gelatinous oil sparkles with tiny,\
ultrathin silver shards.\

Original layout: [[../../srd-5.2.1/pages/233|page 233]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/oil_of_sharpness ` |
| ` implementation ` | ` dnd2024:item_oil_of_sharpness ` |

### Parameters

#### Application

<!-- splat:values path=splat_rule/parameters/application -->

| Field | Value |
| --- | --- |
| ` consume_vial ` | ` true ` |
| ` duration_of_conversion ` | ` permanent ` |

##### Duration

<!-- splat:values path=splat_rule/parameters/application/duration -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` minute ` |

##### Target choice

<!-- splat:table
path: splat_rule/parameters/application/target_choice
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Kind | Count |
| --- | --- | --- |
| ` row_1 ` | ` Melee weapon ` | ` 1 ` |
| ` row_2 ` | ` Ammunition ` | ` 20 ` |

##### Eligibility

<!-- splat:values path=splat_rule/parameters/application/eligibility -->

| Field | Value |
| --- | --- |
| ` nonmagical ` | ` true ` |
| ` damage_types_any ` | ` ["slashing", "piercing"] ` |

##### Result

<!-- splat:yaml path=splat_rule/parameters/application/result -->

```yaml
melee_weapon:
  definition:
    key: dnd2024:items/weapon_1_2_or_3
  bonus: 3
ammunition:
  definition:
    key: dnd2024:items/ammunition_1_2_or_3
  bonus: 3
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
