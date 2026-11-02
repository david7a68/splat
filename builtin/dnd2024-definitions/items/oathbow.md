---
splat_schema: 1
splat_id: fcc1fd54-e8f6-57b6-be5f-c4128d587fe8
splat_kind: item
splat_key: dnd2024:items/oathbow
splat_name: Oathbow
splat_origin:
  key: dnd2024:reference/magic_item/oathbow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233
  url: https://www.dndbeyond.com/srd
---

# Oathbow

## Source text

<!-- SRD page 233 -->

Weapon (Longbow or Shortbow), Very Rare (Requires Attunement)

When you nock an arrow on this bow, it whispers in Elvish, “Swift defeat to my enemies.” When you use this weapon to make a ranged attack, you can utter or sign the following command words: “Swift death to you who have wronged me.” The target of your attack becomes your sworn enemy until it dies or until dawn 7 days later. You can have only one such sworn enemy at a time. When your sworn enemy dies, you can choose a new one after the next dawn.

When you make a ranged attack roll with this weapon against your sworn enemy, you have Advantage on the roll. In addition, your target gains no benefit from Half Cover or Three-Quarters Cover, and you suffer no Disadvantage due to long range. If the attack hits, your sworn enemy takes an extra 3d6 Piercing damage.

While your sworn enemy lives, you have Disadvantage on attack rolls with all other weapons.

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
| ` mechanic ` | ` dnd2024:items/oathbow ` |
| ` implementation ` | ` dnd2024:item_oathbow ` |

### Parameters

#### Declare enemy

<!-- splat:yaml path=splat_rule/parameters/declare_enemy -->

```yaml
trigger: ranged_attack_with_bow
command:
  modes:
  - utter
  - sign
target: attack_target
maximum_active: 1
duration:
  until_any:
  - enemy_death
  - dawn_seven_days_later
if_enemy_dies_next_selection_after: next_dawn
```

#### Against sworn enemy

<!-- splat:yaml path=splat_rule/parameters/against_sworn_enemy -->

```yaml
attack_advantage: true
ignore_cover:
- half
- three_quarters
ignore_long_range_disadvantage: true
on_hit:
  damage:
    roll:
      expression: 3d6
      scope: attack
    type: piercing
```

#### While sworn enemy alive

<!-- splat:values path=splat_rule/parameters/while_sworn_enemy_alive -->

| Field | Value |
| --- | --- |
| ` other_weapons_attack_disadvantage ` | ` true ` |

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
