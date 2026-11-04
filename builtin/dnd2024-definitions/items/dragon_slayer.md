---
splat_schema: 2
splat_id: 064624ef-1abe-55f7-83bb-1b014c33d5b9
splat_kind: item
splat_key: dnd2024:items/dragon_slayer
splat_name: Dragon Slayer
splat_origin: dnd2024:reference/magic_item/dragon_slayer
splat_sources:
- SRD 5.2.1, pp. 219, https://www.dndbeyond.com/srd
---

# Dragon Slayer


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 219 -->

Weapon (Any Simple or Martial), Rare

You gain a +1 bonus to attack rolls and damage rolls made with this magic weapon.

The weapon deals an extra 3d6 damage of the weapon’s type if the target is a Dragon.

Original layout: [[../../srd-5.2.1/pages/219|page 219]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/dragon_slayer ` |
| ` implementation ` | ` dnd2024:item_dragon_slayer ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_roll_bonus ` | ` 1 ` |
| ` damage_roll_bonus ` | ` 1 ` |
| ` scope ` | ` attacks_with_source_item ` |

#### On hit

<!-- splat:yaml path=splat_rule/parameters/on_hit -->

```yaml
target_creature_type: dragon
extra_damage:
  roll:
    expression: 3d6
    scope: attack
  type: weapon_damage_type
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_weapons ` |
