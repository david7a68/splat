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

Wondrous Item, Uncommon (Requires Attunement)\
If you’re hit by an attack roll made with a Ranged\
or Thrown weapon while wearing these gloves, you\
can take a Reaction to reduce the damage by 1d10\
plus your Dexterity modifier if you have a free hand.\
If you reduce the damage to 0, you can catch the am-\
munition or weapon if it is small enough for you to\
hold in that hand.\

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

<!-- splat:yaml path=splat_rule/parameters/reaction -->

```yaml
trigger:
  hit_by: attack_roll
  weapon_category:
  - ranged
  - thrown
requires: free_hand
damage_reduction:
  sum:
  - dice: 1
    sides: 10
    bonus: 0
  - value_from: dexterity_modifier
if_damage_reduced_to_zero:
  may_catch:
  - ammunition
  - weapon
  requires: fits_free_hand
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
