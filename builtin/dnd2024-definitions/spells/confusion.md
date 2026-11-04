---
splat_schema: 2
splat_id: e9180ba2-3083-50d8-8054-9cd64e6f8d02
splat_kind: spell
splat_key: dnd2024:spells/confusion
splat_name: Confusion
splat_origin: dnd2024:reference/spell/confusion
splat_sources:
- SRD 5.2.1, pp. 117, https://www.dndbeyond.com/srd
---

# Confusion


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Enchantment |
| Classes | Bard, Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 90 feet |
| Components | V, S, M (three nut shells) |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 117 -->

Level 4 Enchantment (Bard, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (three nut shells)\
Duration: Concentration, up to 1 minute

Each creature in a 10-foot-radius Sphere centered on a point you choose within range must succeed on a Wisdom saving throw, or that target can’t take Bonus Actions or Reactions and must roll 1d10 at the start of each of its turns to determine its behavior for that turn, consulting the table below.

| 1d10 | Behavior for the Turn |
| --- | --- |
| 1 | The target doesn’t take an action, and it uses all its movement to move. Roll 1d4 for the direction: 1, north; 2, east; 3, south; or 4, west. |
| 2–6 | The target doesn’t move or take actions. |
| 7–8 | The target doesn’t move, and it takes the Attack action to make one melee attack against a random creature within reach. If none are within reach, the target takes no action. |
| 9–10 | The target chooses its behavior. |

At the end of each of its turns, an affected target repeats the save, ending the spell on itself on a success.

Using a Higher-Level Spell Slot. The Sphere’s radius increases by 5 feet for each spell slot level above 4.

Original layout: [[../../srd-5.2.1/pages/117|page 117]].







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

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |

##### Dc

<!-- splat:values path=splat_rule/parameters/save/dc -->

| Field | Value |
| --- | --- |
| ` binding ` | ` save_dc ` |

##### Failure

<!-- splat:values path=splat_rule/parameters/save/failure -->

| Field | Value |
| --- | --- |
| ` prohibit ` | ` ["bonus_action", "reaction"] ` |
| ` duration ` | ` spell_duration ` |

###### Turn start roll

###### Dice

<!-- splat:yaml path=splat_rule/parameters/save/failure/turn_start_roll/dice -->

```yaml
dice:
  count: 1
  sides: 10
  add: 0
```

###### Table

<!-- splat:table
path: splat_rule/parameters/save/failure/turn_start_roll/table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Range:
    path: range
    type: yaml
  Action:
    path: action
    type: symbol
    optional: true
  Movement:
    path: movement
    type: symbol
    optional: true
  Direction roll dice dice:
    path: direction_roll/dice/dice
    type: dice
    optional: true
  Melee attacks:
    path: melee_attacks
    type: integer
    optional: true
  Target:
    path: target
    type: symbol
    optional: true
  Without target action:
    path: without_target_action
    type: symbol
    optional: true
  Behavior:
    path: behavior
    type: symbol
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Range | Action | Movement | Direction roll dice dice | Melee attacks | Target | Without target action | Behavior | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` [1, 1] ` | ` None ` | ` All available ` | ` 1d4 ` |  |  |  |  | [Rules](#^splat-62be980c-row_1 "section:row_1") |
| ` row_2 ` | ` [2, 6] ` | ` None ` | ` None ` |  |  |  |  |  |  |
| ` row_3 ` | ` [7, 8] ` | ` Attack ` | ` None ` |  | ` 1 ` | ` Random creature within reach ` | ` None ` |  |  |
| ` row_4 ` | ` [9, 10] ` |  |  |  |  |  |  | ` Target choice ` |  |

###### Row 1

^splat-62be980c-row_1

###### Faces

<!-- splat:yaml path=splat_rule/parameters/save/failure/turn_start_roll/table/@row_1/direction_roll/faces -->

```yaml
'1': north
'2': east
'3': south
'4': west
```

###### Repeat save

<!-- splat:values path=splat_rule/parameters/save/failure/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` target_turn_end ` |
| ` success ` | ` end_effect_on_target ` |

##### Success

<!-- splat:values path=splat_rule/parameters/save/success -->

| Field | Value |
| --- | --- |
| ` effect ` | ` none ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

