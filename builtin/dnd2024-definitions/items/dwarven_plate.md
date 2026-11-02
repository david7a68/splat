---
splat_schema: 1
splat_id: 352fc4da-6796-5ed3-8326-600acc8d4387
splat_kind: item
splat_key: dnd2024:items/dwarven_plate
splat_name: Dwarven Plate
splat_origin:
  key: dnd2024:reference/magic_item/dwarven_plate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 219
  url: https://www.dndbeyond.com/srd
---

# Dwarven Plate

## Source text

<!-- SRD page 219 -->

Armor (Half Plate Armor or Plate Armor), Very Rare

While wearing this armor, you gain a +2 bonus to Armor Class. In addition, if an effect moves you against your will along the ground, you can take a Reaction to reduce the distance you are moved by up to 10 feet.

Original layout: [[../../srd-5.2.1/pages/219|page 219]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
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
| ` mechanic ` | ` dnd2024:items/dwarven_plate ` |
| ` implementation ` | ` dnd2024:item_dwarven_plate ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` ac_bonus ` | ` 2 ` |
| ` attunement_required ` | ` false ` |

#### Reaction

<!-- splat:yaml path=splat_rule/parameters/reaction -->

```yaml
trigger: unwilling_movement_along_ground
distance_reduction_ft:
  choice_min: 0
  choice_max: 10
  not_below_zero: true
```
