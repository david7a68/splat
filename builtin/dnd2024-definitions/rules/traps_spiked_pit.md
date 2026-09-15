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

<!-- splat:rows path=splat_rule/parameters/damage -->

| Row | type | average |
| --- | --- | --- |
| ` 0 ` | ` bludgeoning ` | ` 3 ` |
| ` 1 ` | ` piercing ` | ` 9 ` |

##### Dice

<!-- splat:values path=splat_rule/parameters/damage/0/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

##### Dice

<!-- splat:values path=splat_rule/parameters/damage/1/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` sides ` | ` 8 ` |
| ` add ` | ` 0 ` |

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

<!-- splat:rows path=splat_rule/parameters/scaling -->

| Row | levels | depth_feet |
| --- | --- | --- |
| ` 0 ` | ` [1, 4] ` | ` 10 ` |
| ` 1 ` | ` [5, 10] ` | ` 30 ` |
| ` 2 ` | ` [11, 16] ` | ` 60 ` |
| ` 3 ` | ` [17, 20] ` | ` 120 ` |

##### Damage

<!-- splat:rows path=splat_rule/parameters/scaling/0/damage -->

| Row | type | average |
| --- | --- | --- |
| ` 0 ` | ` bludgeoning ` | ` 3 ` |
| ` 1 ` | ` piercing ` | ` 9 ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/scaling/0/damage/0/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/scaling/0/damage/1/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` sides ` | ` 8 ` |
| ` add ` | ` 0 ` |

##### Damage

<!-- splat:rows path=splat_rule/parameters/scaling/1/damage -->

| Row | type | average |
| --- | --- | --- |
| ` 0 ` | ` bludgeoning ` | ` 10 ` |
| ` 1 ` | ` piercing ` | ` 13 ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/scaling/1/damage/0/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 3 ` |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/scaling/1/damage/1/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 3 ` |
| ` sides ` | ` 8 ` |
| ` add ` | ` 0 ` |

##### Damage

<!-- splat:rows path=splat_rule/parameters/scaling/2/damage -->

| Row | type | average |
| --- | --- | --- |
| ` 0 ` | ` bludgeoning ` | ` 21 ` |
| ` 1 ` | ` piercing ` | ` 36 ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/scaling/2/damage/0/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 6 ` |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/scaling/2/damage/1/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 8 ` |
| ` sides ` | ` 8 ` |
| ` add ` | ` 0 ` |

##### Damage

<!-- splat:rows path=splat_rule/parameters/scaling/3/damage -->

| Row | type | average |
| --- | --- | --- |
| ` 0 ` | ` bludgeoning ` | ` 42 ` |
| ` 1 ` | ` piercing ` | ` 57 ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/scaling/3/damage/0/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 12 ` |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/scaling/3/damage/1/dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 13 ` |
| ` sides ` | ` 8 ` |
| ` add ` | ` 0 ` |
