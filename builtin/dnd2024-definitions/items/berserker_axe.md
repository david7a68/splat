---
splat_schema: 1
splat_id: 00eeaa8a-b02d-5feb-9a8f-b72e1561978d
splat_kind: item
splat_key: dnd2024:items/berserker_axe
splat_name: Berserker Axe
splat_origin:
  key: dnd2024:reference/magic_item/berserker_axe
splat_sources:
- title: SRD 5.2.1
  locator: pp. 213
  url: https://www.dndbeyond.com/srd
---

# Berserker Axe

## Source text

<!-- SRD page 213 -->

Weapon (Battleaxe, Greataxe, or Halberd), Rare\
(Requires Attunement)\
You gain a +1 bonus to attack rolls and damage rolls\
made with this magic weapon. In addition, while\
you are attuned to this weapon, your Hit Point\
maximum increases by 1 for each level you have\
attained.\
Curse. This weapon is cursed, and becoming at-\
tuned to it extends the curse to you. As long as you\
remain cursed, you are unwilling to part with the\
weapon, keeping it within reach at all times. You\
also have Disadvantage on attack rolls with weap-\
ons other than this one.\
Whenever another creature damages you while\
the weapon is in your possession, you must succeed\
on a DC 15 Wisdom saving throw or go berserk. This\
berserk state ends when you start your turn and\
there are no creatures within 60 feet of you that you\
can see or hear.\
While berserk, you regard the creature nearest\
to you that you can see or hear as your enemy. If\
there are multiple possible creatures, choose one at\
random. On each of your turns, you must move as\
close to the creature as possible and take the Attack\
action, targeting the creature. If you’re unable to\
get close enough to the creature to attack it with\
the weapon, your turn ends after you’ve used up all\
your available movement. If the creature dies or can\
no longer be seen or heard by you, the next nearest\
creature that you can see or hear becomes your\
new target.\

Original layout: [[../../srd-5.2.1/pages/213|page 213]].

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
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/berserker_axe ` |
| ` implementation ` | ` dnd2024:item_berserker_axe ` |

### Parameters

#### Weapon bonus

<!-- splat:values path=splat_rule/parameters/weapon_bonus -->

| Field | Value |
| --- | --- |
| ` attack ` | ` 1 ` |
| ` damage ` | ` 1 ` |

#### Attuned passive

<!-- splat:yaml path=splat_rule/parameters/attuned_passive -->

```yaml
hit_point_maximum_bonus:
  value_from: character_level
```

#### Curse

<!-- splat:yaml path=splat_rule/parameters/curse -->

```yaml
starts: attunement
unwilling_to_part: true
must_keep: within_reach
other_weapon_attacks: disadvantage
berserk_trigger:
  event: damage_received
  source: other_creature
  requires: item_in_possession
  save:
    ability: wisdom
    dc: 15
  failure: berserk
berserk:
  end_check: self_turn_start
  end_if:
    no_creatures_seen_or_heard_within_ft: 60
  enemy:
    select: nearest_seen_or_heard_creature
    tie: random
  turn:
    movement: as_close_to_enemy_as_possible
    action: attack
    attack_target: enemy
    weapon: this_item
    if_unreachable:
      spend: all_available_movement
      then: end_turn
  retarget_on:
  - target_death
  - target_neither_seen_nor_heard
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
