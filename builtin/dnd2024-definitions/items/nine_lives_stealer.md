---
splat_schema: 1
splat_id: 8be02c9d-0f00-5118-bd24-e510f38d46d6
splat_kind: item
splat_key: dnd2024:items/nine_lives_stealer
splat_name: Nine Lives Stealer
splat_origin:
  key: dnd2024:reference/magic_item/nine_lives_stealer
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233
  url: https://www.dndbeyond.com/srd
---

# Nine Lives Stealer

## Source text

<!-- SRD page 233 -->

Weapon (Any Simple or Martial), Very Rare (Requires Attunement)

You gain a +2 bonus to attack rolls and damage rolls made with this magic weapon.

Life Stealing. The weapon has 1d8 + 1 charges. When you attack a creature that has fewer than 100 Hit Points with this weapon and roll a 20 on the d20 for the attack roll, the creature must succeed on a DC 15 Constitution saving throw or be slain instantly as the sword tears its life force from its body. Constructs and Undead succeed on the save automatically. The weapon loses 1 charge if the creature is slain. When the weapon has no charges remaining, it loses this property.

Original layout: [[../../srd-5.2.1/pages/233|page 233]].

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
| ` mechanic ` | ` dnd2024:items/nine_lives_stealer ` |
| ` implementation ` | ` dnd2024:item_nine_lives_stealer ` |

### Parameters

#### Weapon bonus

<!-- splat:values path=splat_rule/parameters/weapon_bonus -->

| Field | Value |
| --- | --- |
| ` attack ` | ` 2 ` |
| ` damage ` | ` 2 ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial:
  dice: 1
  sides: 8
  bonus: 1
recovery: null
on_zero: lose_life_stealing
```

#### Life stealing

<!-- splat:yaml path=splat_rule/parameters/life_stealing -->

```yaml
requires: charges_positive
trigger:
  event: attack_roll
  natural_d20: 20
  target_kind: creature
  target_current_hp_less_than: 100
save:
  ability: constitution
  dc: 15
  automatic_success_types:
  - construct
  - undead
failure:
  effect: instant_death
  charge_cost: 1
success_charge_cost: 0
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
