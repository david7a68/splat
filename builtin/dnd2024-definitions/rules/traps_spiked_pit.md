---
splat_schema: 1
splat_id: 53d7b4b0-dab8-5eb6-9392-d8d81d0d818f
splat_kind: rule
splat_key: dnd2024:rules/traps_spiked_pit
splat_name: Spiked Pit
splat_origin:
  key: dnd2024:reference/pages/201
splat_sources:
- title: SRD 5.2.1
  locator: pp. 201
  url: https://www.dndbeyond.com/srd
---

# Spiked Pit

## Source text

<!-- SRD page 201 -->

Deadly Trap (Levels 1–4)\
Trigger: A creature moves onto the pit’s lid\
Duration: Instantaneous\
A 10-foot-deep pit has a hinged lid constructed from\
material identical to the surrounding floor. When\
a creature moves onto the lid, it swings open like\
a trapdoor, causing the creature to fall into the pit,\
which has sharpened wooden or metal spikes at the\
bottom. The lid remains open thereafter.\
A creature that falls into the pit lands at the\
bottom and takes 3 (1d6) Bludgeoning damage\
from the fall plus 9 (2d8) Piercing damage from\
the spikes.\
Detect and Disarm. As a Study action, a creature\
can examine the section of floor that forms the pit’s\
lid and make a DC 15 Intelligence (Investigation)\
check, detecting the pit on a successful check. Once\
the hidden pit is detected, an Iron Spike or a similar\
object can be wedged between the pit’s lid and the\
surrounding floor to prevent the lid from opening,\
thereby making it safe to cross. The cover can also\
be held shut using an Arcane Lock spell or similar\
magic.\
Escape. A creature needs a Climb Speed, climbing\
gear, or magic such as a Spider Climb spell to scale\
the pit’s smooth walls. You can make the pit easier\
to escape by adding cracks in the walls big enough\
to serve as handholds and footholds.\
At Higher Levels. You can scale the trap for higher\
levels by increasing the pit’s depth and damage, as\
shown in the following table.\
Levels Pit Depth Damage\
5–10 30 feet 10 (3d6) Bludgeoning plus 13\
(3d8) Piercing\
11–16 60 feet 21 (6d6) Bludgeoning plus 36\
(8d8) Piercing\
17–20 120 feet 42 (12d6) Bludgeoning plus 57\
(13d8) Piercing\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/traps_spiked_pit ` |
| ` implementation ` | ` dnd2024:rules_traps_spiked_pit ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger_effect ` | ` fall_to_pit_bottom ` |
| ` depth_feet ` | ` 10 ` |

#### Trap

<!-- splat:values path=splat_rule/parameters/trap -->

| Field | Value |
| --- | --- |
| ` severity ` | ` deadly ` |
| ` levels ` | ` [1, 4] ` |
| ` trigger ` | ` creature_moves_onto_lid ` |
| ` duration ` | ` instantaneous ` |

#### Lid

<!-- splat:values path=splat_rule/parameters/lid -->

| Field | Value |
| --- | --- |
| ` hinged ` | ` true ` |
| ` hidden_as_floor ` | ` true ` |
| ` remains_open_after_trigger ` | ` true ` |

#### Damage

<!-- splat:table
path: splat_rule/parameters/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Type:
    path: type
    type: symbol
  Dice:
    path: dice
    type: dice
  Average:
    path: average
    type: integer
-->

| Key | Type | Dice | Average |
| --- | --- | --- | --- |
| ` row_1 ` | ` Bludgeoning ` | ` 1d6 ` | ` 3 ` |
| ` row_2 ` | ` Piercing ` | ` 2d8 ` | ` 9 ` |

#### Detection

<!-- splat:yaml path=splat_rule/parameters/detection -->

```yaml
economy: study
check:
  ability: intelligence
  dc: 15
  skill: investigation
target: lid_floor_section
```

#### Disarm

<!-- splat:values path=splat_rule/parameters/disarm -->

| Field | Value |
| --- | --- |
| ` after_detection ` | ` ["wedge_lid_with_iron_spike_or_similar", "hold_lid_closed_with_arcane_lock_or_similar_magic"] ` |

#### Escape

<!-- splat:values path=splat_rule/parameters/escape -->

| Field | Value |
| --- | --- |
| ` requires_any ` | ` ["climb_speed", "climbing_gear", "wall_climbing_magic"] ` |
| ` gm_geometry_option ` | ` add_handholds_and_footholds ` |

#### Scaling

<!-- splat:table
path: splat_rule/parameters/scaling
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Levels:
    path: levels
    type: yaml
  Depth feet:
    path: depth_feet
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Levels | Depth feet | Details |
| --- | --- | --- | --- |
| ` row_1 ` | ` [1, 4] ` | ` 10 ` | [Rules](#^splat-c84e121e-row_1 "section:row_1") |
| ` row_2 ` | ` [5, 10] ` | ` 30 ` | [Rules](#^splat-c84e121e-row_2 "section:row_2") |
| ` row_3 ` | ` [11, 16] ` | ` 60 ` | [Rules](#^splat-c84e121e-row_3 "section:row_3") |
| ` row_4 ` | ` [17, 20] ` | ` 120 ` | [Rules](#^splat-c84e121e-row_4 "section:row_4") |

##### Row 1

^splat-c84e121e-row_1

###### Damage

<!-- splat:table
path: splat_rule/parameters/scaling/@row_1/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Type:
    path: type
    type: symbol
  Dice:
    path: dice
    type: dice
  Average:
    path: average
    type: integer
-->

| Key | Type | Dice | Average |
| --- | --- | --- | --- |
| ` row_1 ` | ` Bludgeoning ` | ` 1d6 ` | ` 3 ` |
| ` row_2 ` | ` Piercing ` | ` 2d8 ` | ` 9 ` |

##### Row 2

^splat-c84e121e-row_2

###### Damage

<!-- splat:table
path: splat_rule/parameters/scaling/@row_2/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Type:
    path: type
    type: symbol
  Dice:
    path: dice
    type: dice
  Average:
    path: average
    type: integer
-->

| Key | Type | Dice | Average |
| --- | --- | --- | --- |
| ` row_1 ` | ` Bludgeoning ` | ` 3d6 ` | ` 10 ` |
| ` row_2 ` | ` Piercing ` | ` 3d8 ` | ` 13 ` |

##### Row 3

^splat-c84e121e-row_3

###### Damage

<!-- splat:table
path: splat_rule/parameters/scaling/@row_3/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Type:
    path: type
    type: symbol
  Dice:
    path: dice
    type: dice
  Average:
    path: average
    type: integer
-->

| Key | Type | Dice | Average |
| --- | --- | --- | --- |
| ` row_1 ` | ` Bludgeoning ` | ` 6d6 ` | ` 21 ` |
| ` row_2 ` | ` Piercing ` | ` 8d8 ` | ` 36 ` |

##### Row 4

^splat-c84e121e-row_4

###### Damage

<!-- splat:table
path: splat_rule/parameters/scaling/@row_4/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Type:
    path: type
    type: symbol
  Dice:
    path: dice
    type: dice
  Average:
    path: average
    type: integer
-->

| Key | Type | Dice | Average |
| --- | --- | --- | --- |
| ` row_1 ` | ` Bludgeoning ` | ` 12d6 ` | ` 42 ` |
| ` row_2 ` | ` Piercing ` | ` 13d8 ` | ` 57 ` |
