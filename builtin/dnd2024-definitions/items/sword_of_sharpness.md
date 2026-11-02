---
splat_schema: 1
splat_id: 7eb7ac23-9716-5f16-9f5c-54f75ddef069
splat_kind: item
splat_key: dnd2024:items/sword_of_sharpness
splat_name: Sword of Sharpness
splat_origin:
  key: dnd2024:reference/magic_item/sword_of_sharpness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248
  url: https://www.dndbeyond.com/srd
---

# Sword of Sharpness

## Source text

<!-- SRD page 248 -->

Weapon (Glaive, Greatsword, Longsword, or Scimitar), Very Rare (Requires Attunement)

When you attack an object with this magic weapon and hit, maximize your weapon damage dice against the target.

When you attack a creature with this weapon and roll a 20 on the d20 for the attack roll, that target takes an extra 14 Slashing damage and gains 1 Exhaustion level.

Original layout: [[../../srd-5.2.1/pages/248|page 248]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/sword_of_sharpness ` |
| ` implementation ` | ` dnd2024:item_sword_of_sharpness ` |

### Parameters

#### On hit object

<!-- splat:values path=splat_rule/parameters/on_hit_object -->

| Field | Value |
| --- | --- |
| ` maximize ` | ` weapon_damage_dice ` |

#### On natural 20 against creature

<!-- splat:yaml path=splat_rule/parameters/on_natural_20_against_creature -->

```yaml
extra_damage:
  amount: 14
  type: slashing
exhaustion_increase: 1
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_weapons ` |
