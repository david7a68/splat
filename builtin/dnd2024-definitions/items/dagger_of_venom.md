---
splat_schema: 2
splat_id: bd466de5-38b0-58da-a08b-e07395c573a5
splat_kind: item
splat_key: dnd2024:items/dagger_of_venom
splat_name: Dagger of Venom
splat_origin: dnd2024:reference/magic_item/dagger_of_venom
splat_sources:
- SRD 5.2.1, pp. 216, https://www.dndbeyond.com/srd
---

# Dagger of Venom


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 216 -->

Weapon (Dagger), Rare

You gain a +1 bonus to attack rolls and damage rolls made with this magic weapon.

You can take a Bonus Action to magically coat the blade with poison. The poison remains for 1 minute or until an attack using this weapon hits a creature. That creature must succeed on a DC 15 Constitution saving throw or take 2d10 Poison damage and have the Poisoned condition for 1 minute. The weapon can’t be used this way again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/216|page 216]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/dagger_of_venom ` |
| ` implementation ` | ` dnd2024:item_dagger_of_venom ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_roll_bonus ` | ` 1 ` |
| ` damage_roll_bonus ` | ` 1 ` |
| ` scope ` | ` attacks_with_source_item ` |

#### Poison coating

<!-- splat:yaml path=splat_rule/parameters/poison_coating -->

```yaml
action: bonus_action
uses:
  maximum: 1
  initial: 1
  recovery:
    event: dawn
    amount:
      expression: '1'
      scope: use
    cap: maximum
duration:
  amount: 1
  unit: minute
ends_on: first_creature_hit
on_first_creature_hit:
  save:
    ability: constitution
    dc: 15
    failure:
      damage:
        roll:
          expression: 2d10
          scope: attack
        type: poison
      condition: poisoned
      duration:
        amount: 1
        unit: minute
    success:
      effect: none
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
