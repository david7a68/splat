---
splat_schema: 1
splat_id: e9180ba2-3083-50d8-8054-9cd64e6f8d02
splat_kind: spell
splat_key: dnd2024:spells/confusion
splat_name: Confusion
splat_origin:
  key: dnd2024:reference/spell/confusion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 117
  url: https://www.dndbeyond.com/srd
---

# Confusion

## Source text

<!-- SRD page 117 -->

Level 4 Enchantment (Bard, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (three nut shells)\
Duration: Concentration, up to 1 minute\
Each creature in a 10-foot-radius Sphere centered\
on a point you choose within range must succeed on\
a Wisdom saving throw, or that target can’t take Bo-\
nus Actions or Reactions and must roll 1d10 at the\
start of each of its turns to determine its behavior\
for that turn, consulting the table below.\
1d10 Behavior for the Turn\
1 The target doesn’t take an action, and it uses all\
its movement to move. Roll 1d4 for the direc-\
tion: 1, north; 2, east; 3, south; or 4, west.\
2–6 The target doesn’t move or take actions.\
7–8 The target doesn’t move, and it takes the At-\
tack action to make one melee attack against\
a random creature within reach. If none are\
within reach, the target takes no action.\
9–10 The target chooses its behavior.\
At the end of each of its turns, an affected target\
repeats the save, ending the spell on itself on a\
success.\
Using a Higher-Level Spell Slot. The Sphere’s\
radius increases by 5 feet for each spell slot level\
above 4.\

Original layout: [[../../srd-5.2.1/pages/117|page 117]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "druid", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 90 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` three nut shells ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/confusion ` |
| ` implementation ` | ` dnd2024:spell_confusion ` |

### Parameters

#### Area

<!-- splat:yaml path=splat_rule/parameters/area -->

```yaml
shape: sphere
radius_feet:
  op: add
  args:
  - 10
  - op: multiply
    args:
    - 5
    - op: subtract
      args:
      - binding: cast_level
      - 4
origin: chosen_point_within_range
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  prohibit:
  - bonus_action
  - reaction
  duration: spell_duration
  turn_start_roll:
    dice:
      dice:
        count: 1
        sides: 10
        add: 0
    table:
    - range:
      - 1
      - 1
      action: none
      movement: all_available
      direction_roll:
        dice:
          dice:
            count: 1
            sides: 4
            add: 0
        faces:
          '1': north
          '2': east
          '3': south
          '4': west
    - range:
      - 2
      - 6
      action: none
      movement: none
    - range:
      - 7
      - 8
      movement: none
      action: attack
      melee_attacks: 1
      target: random_creature_within_reach
      without_target_action: none
    - range:
      - 9
      - 10
      behavior: target_choice
  repeat_save:
    event: target_turn_end
    success: end_effect_on_target
success:
  effect: none
```
