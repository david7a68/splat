---
splat_schema: 1
splat_id: 9a565af0-d3f5-5b2b-ae60-c1ef925a3dd3
splat_kind: item
splat_key: dnd2024:items/gloves_of_missile_snaring
splat_name: Gloves of Missile Snaring
splat_origin:
  key: dnd2024:reference/magic_item/gloves_of_missile_snaring
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
---

# Gloves of Missile Snaring

## Source text

<!-- SRD page 224 -->

Wondrous Item, Uncommon (Requires Attunement)

If you’re hit by an attack roll made with a Ranged or Thrown weapon while wearing these gloves, you can take a Reaction to reduce the damage by 1d10 plus your Dexterity modifier if you have a free hand. If you reduce the damage to 0, you can catch the ammunition or weapon if it is small enough for you to hold in that hand.

Original layout: [[../../srd-5.2.1/pages/224|page 224]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/gloves_of_missile_snaring ` |
| ` implementation ` | ` dnd2024:item_gloves_of_missile_snaring ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Reaction

<!-- splat:values path=splat_rule/parameters/reaction -->

| Field | Value |
| --- | --- |
| ` requires ` | ` free_hand ` |

##### Trigger

<!-- splat:values path=splat_rule/parameters/reaction/trigger -->

| Field | Value |
| --- | --- |
| ` hit_by ` | ` attack_roll ` |
| ` weapon_category ` | ` ["ranged", "thrown"] ` |

##### Damage reduction

###### Sum

<!-- splat:table
path: splat_rule/parameters/reaction/damage_reduction/sum
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Dice:
    path: dice
    type: integer
    optional: true
  Sides:
    path: sides
    type: integer
    optional: true
  Bonus:
    path: bonus
    type: integer
    optional: true
  Value from:
    path: value_from
    type: symbol
    optional: true
-->

| Key | Dice | Sides | Bonus | Value from |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 10 ` | ` 0 ` |  |
| ` row_2 ` |  |  |  | ` Dexterity modifier ` |

##### If damage reduced to zero

<!-- splat:values path=splat_rule/parameters/reaction/if_damage_reduced_to_zero -->

| Field | Value |
| --- | --- |
| ` may_catch ` | ` ["ammunition", "weapon"] ` |
| ` requires ` | ` fits_free_hand ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
