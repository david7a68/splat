---
splat_schema: 1
splat_id: 793209c7-3287-5bbd-a8e3-4e99c5897553
splat_kind: item
splat_key: dnd2024:items/vorpal_sword
splat_name: Vorpal Sword
splat_origin:
  key: dnd2024:reference/magic_item/vorpal_sword
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
---

# Vorpal Sword

## Source text

<!-- SRD page 250 -->

Weapon (Glaive, Greatsword, Longsword, or Scimitar), Legendary (Requires Attunement)

You gain a +3 bonus to attack rolls and damage rolls made with this magic weapon. In addition, the weapon ignores Resistance to Slashing damage.

When you use this weapon to attack a creature that has at least one head and roll a 20 on the d20 for the attack roll, you cut off one of the creature’s heads. The creature dies if it can’t survive without the lost head. A creature is immune to this effect if it has Immunity to Slashing damage, if it doesn’t have or need a head, or if the GM decides that the creature is too big for its head to be cut off with this weapon. Such a creature instead takes an extra 30 Slashing damage from the hit. If the creature has Legendary Resistance, it can expend one daily use of that trait to avoid losing its head, taking the extra damage instead.

Original layout: [[../../srd-5.2.1/pages/250|page 250]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/vorpal_sword ` |
| ` implementation ` | ` dnd2024:item_vorpal_sword ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_roll_bonus ` | ` 3 ` |
| ` damage_roll_bonus ` | ` 3 ` |
| ` scope ` | ` attacks_with_source_item ` |
| ` ignore_damage_resistance ` | ` ["slashing"] ` |

#### On natural 20

<!-- splat:yaml path=splat_rule/parameters/on_natural_20 -->

```yaml
target: creature
decapitation:
  heads_removed: 1
  requires_at_least_one_head: true
  dies_if_cannot_survive_head_loss: true
  immune_if:
  - slashing_damage_immunity
  - no_head
  - head_not_needed
  gm_choice:
    type: boolean
    scope: head_too_large_to_sever_with_weapon
  legendary_resistance:
    optional_use: true
    daily_uses_cost: 1
    effect: replace_decapitation_with_extra_damage
instead_if_decapitation_immune_or_resisted:
  extra_damage:
    amount: 30
    type: slashing
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
