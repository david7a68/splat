---
splat_schema: 1
splat_id: ed93c742-1183-5696-b3b5-c0574dc8a89e
splat_kind: spell
splat_key: dnd2024:spells/arcane_hand
splat_name: Arcane Hand
splat_origin:
  key: dnd2024:reference/spell/arcane_hand
splat_sources:
- title: SRD 5.2.1
  locator: pp. 110
  url: https://www.dndbeyond.com/srd
---

# Arcane Hand

## Source text

<!-- SRD page 110 -->

Level 5 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (an eggshell and a glove)\
Duration: Concentration, up to 1 minute\
You create a Large hand of shimmering magical en-\
ergy in an unoccupied space that you can see within\
range. The hand lasts for the duration, and it moves\
at your command, mimicking the movements of\
your own hand.\
The hand is an object that has AC 20 and Hit\
Points equal to your Hit Point maximum. If it drops\
to 0 Hit Points, the spell ends. The hand doesn’t oc-\
cupy its space.\
When you cast the spell and as a Bonus Action on\
your later turns, you can move the hand up to 60\
feet and then cause one of the following effects:\
Clenched Fist. The hand strikes a target within 5\
feet of it. Make a melee spell attack. On a hit, the\
target takes 5d8 Force damage.\
Forceful Hand. The hand attempts to push a Huge\
or smaller creature within 5 feet of it. The target\
must succeed on a Strength saving throw, or the\
hand pushes the target up to 5 feet plus a number\
of feet equal to five times your spellcasting ability\
modifier. The hand moves with the target, re-\
maining within 5 feet of it.\
Grasping Hand. The hand attempts to grapple a\
Huge or smaller creature within 5 feet of it. The\
target must succeed on a Dexterity saving throw,\
or the target has the Grappled condition, with an\
escape DC equal to your spell save DC. While the\
hand grapples the target, you can take a Bonus\
Action to cause the hand to crush it, dealing Blud-\
geoning damage to the target equal to 4d6 plus\
your spellcasting ability modifier.\
Interposing Hand. The hand grants you Half Cover\
against attacks and other effects that originate\
from its space or that pass through it. In addi-\
tion, its space counts as Difficult Terrain for your\
enemies.\
Using a Higher-Level Spell Slot. The damage of\
the Clenched Fist increases by 2d8 and the damage\
of the Grasping Hand increases by 2d6 for each spell\
slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/110|page 110]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` an eggshell and a glove ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/arcane_hand ` |
| ` implementation ` | ` dnd2024:spell_arcane_hand ` |

### Parameters

#### Creation

<!-- splat:yaml path=splat_rule/parameters/creation -->

```yaml
kind: object
size: large
space:
  unoccupied: true
  visible: true
  within_spell_range: true
ac: 20
hp:
  binding: caster_hp_maximum
occupies_space: false
end_spell_at_hp: 0
duration: spell_duration
appearance: hand
movement_control: caster_hand_gestures
hit_point_maximum:
  binding: caster_hp_maximum
```

#### Activation

<!-- splat:values path=splat_rule/parameters/activation -->

| Field | Value |
| --- | --- |
| ` initial ` | ` spell_cast ` |
| ` subsequent ` | ` bonus_action_on_caster_turn ` |
| ` move_feet_maximum ` | ` 60 ` |
| ` then_choose_effect ` | ` 1 ` |

#### Modes

<!-- splat:yaml path=splat_rule/parameters/modes -->

```yaml
clenched_fist:
  target:
    kind: creature_or_object
    distance_from_hand_feet: 5
  attack: melee_spell
  hit:
    dice:
      count:
        op: add
        args:
        - 5
        - op: multiply
          args:
          - 2
          - op: subtract
            args:
            - binding: cast_level
            - 5
      sides: 8
      add: 0
    type: force
forceful_hand:
  target:
    kind: creature
    maximum_size: huge
    distance_from_hand_feet: 5
  save:
    ability: strength
    dc:
      binding: save_dc
    failure:
      push_feet_maximum:
        op: add
        args:
        - 5
        - op: multiply
          args:
          - 5
          - binding: spellcasting_modifier
      hand_follows: true
      hand_distance_from_target_maximum_feet: 5
    success:
      effect: none
grasping_hand:
  target:
    kind: creature
    maximum_size: huge
    distance_from_hand_feet: 5
  save:
    ability: dexterity
    dc:
      binding: save_dc
    failure:
      condition: grappled
      escape_dc:
        binding: save_dc
    success:
      effect: none
  crush:
    cost: bonus_action
    requires: hand_grapples_target
    damage:
      dice:
        count:
          op: add
          args:
          - 4
          - op: multiply
            args:
            - 2
            - op: subtract
              args:
              - binding: cast_level
              - 5
        sides: 6
        add:
          binding: spellcasting_modifier
      type: bludgeoning
interposing_hand:
  caster_cover: half
  cover_against:
  - originates_in_hand_space
  - passes_through_hand_space
  hand_space_difficult_terrain_for: caster_enemies
```
