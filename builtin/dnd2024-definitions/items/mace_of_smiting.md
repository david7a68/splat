---
splat_schema: 2
splat_id: e77448e7-9e61-56e0-b936-83bf1fe2f395
splat_kind: item
splat_key: dnd2024:items/mace_of_smiting
splat_name: Mace of Smiting
splat_origin: dnd2024:reference/magic_item/mace_of_smiting
splat_sources:
- SRD 5.2.1, pp. 229, https://www.dndbeyond.com/srd
---

# Mace of Smiting


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 229 -->

Weapon (Mace), Rare

You gain a +1 bonus to attack rolls and damage rolls made with this magic weapon. The bonus increases to +3 when you use the weapon to attack a Construct.

When you roll a 20 on an attack roll made with this weapon, the target takes an extra 7 Bludgeoning damage, or 14 Bludgeoning damage if it’s a Construct. If a Construct has 25 Hit Points or fewer after taking this damage, it is destroyed.

Original layout: [[../../srd-5.2.1/pages/229|page 229]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/mace_of_smiting ` |
| ` implementation ` | ` dnd2024:item_mace_of_smiting ` |

### Parameters

#### Weapon bonus

<!-- splat:values path=splat_rule/parameters/weapon_bonus -->

| Field | Value |
| --- | --- |
| ` default ` | ` 1 ` |
| ` against_construct ` | ` 3 ` |
| ` applies ` | ` ["attack", "damage"] ` |

#### Natural twenty

<!-- splat:yaml path=splat_rule/parameters/natural_twenty -->

```yaml
extra_damage:
  type: bludgeoning
  default: 7
  construct: 14
after_damage:
  target_type: construct
  hp_maximum: 25
  effect: destroy
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
