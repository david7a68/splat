---
splat_schema: 1
splat_id: 6b04a7de-44f9-5a60-a461-56889e2ac0d9
splat_kind: item
splat_key: dnd2024:items/frost_brand
splat_name: Frost Brand
splat_origin:
  key: dnd2024:reference/magic_item/frost_brand
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
---

# Frost Brand

## Source text

<!-- SRD page 223 -->

Weapon (Glaive, Greatsword, Longsword, Rapier, Scimitar, or Shortsword), Very Rare (Requires Attunement)

When you hit with an attack roll using this magic weapon, the target takes an extra 1d6 Cold damage. In addition, while you hold the weapon, you have Resistance to Fire damage.

In freezing temperatures, the weapon sheds Bright Light in a 10-foot radius and Dim Light for an additional 10 feet.

When you draw this weapon, you can extinguish all nonmagical flames within 30 feet of yourself. Once used, this property can’t be used again for 1 hour.

Original layout: [[../../srd-5.2.1/pages/223|page 223]].

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
| ` mechanic ` | ` dnd2024:items/frost_brand ` |
| ` implementation ` | ` dnd2024:item_frost_brand ` |

### Parameters

#### On hit

<!-- splat:yaml path=splat_rule/parameters/on_hit -->

```yaml
extra_damage:
  roll:
    dice: 1
    sides: 6
    bonus: 0
  type: cold
```

#### Held

<!-- splat:values path=splat_rule/parameters/held -->

| Field | Value |
| --- | --- |
| ` resistance ` | ` ["fire"] ` |

#### Freezing temperature

<!-- splat:yaml path=splat_rule/parameters/freezing_temperature -->

```yaml
light:
  bright_radius_ft: 10
  additional_dim_ft: 10
```

#### Draw weapon

<!-- splat:yaml path=splat_rule/parameters/draw_weapon -->

```yaml
optional: true
extinguish:
  radius_ft: 30
  targets: all_nonmagical_flames
recovery:
  value: 1
  unit: hour
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
