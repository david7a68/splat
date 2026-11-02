---
splat_schema: 1
splat_id: e965e3c8-ce7d-571b-b556-2cc1d733703e
splat_kind: item
splat_key: dnd2024:items/vicious_weapon
splat_name: Vicious Weapon
splat_origin:
  key: dnd2024:reference/magic_item/vicious_weapon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
---

# Vicious Weapon

## Source text

<!-- SRD page 250 -->

Weapon (Any Simple or Martial), Rare

This magic weapon deals an extra 2d6 damage to any creature it hits. This extra damage is of the same type as the weapon’s normal damage.

Original layout: [[../../srd-5.2.1/pages/250|page 250]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/vicious_weapon ` |
| ` implementation ` | ` dnd2024:item_vicious_weapon ` |

### Parameters

#### On hit

<!-- splat:yaml path=splat_rule/parameters/on_hit -->

```yaml
target_kind: creature
extra_damage:
  roll:
    dice: 2
    sides: 6
    bonus: 0
  type_from: weapon_normal_damage_type
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
