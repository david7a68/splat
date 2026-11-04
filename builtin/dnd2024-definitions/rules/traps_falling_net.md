---
splat_schema: 2
splat_id: df3ce1cb-9599-538b-8830-8dfe93453ce7
splat_kind: rule
splat_key: dnd2024:rules/traps_falling_net
splat_name: Falling Net
splat_origin: dnd2024:reference/pages/199
splat_sources:
- SRD 5.2.1, pp. 199, https://www.dndbeyond.com/srd
---

# Falling Net

## Source text

<!-- SRD page 199 -->

Nuisance Trap (Levels 1–4)

Trigger: A creature crosses a trip wire\
Duration: Instantaneous

A falling net trap uses a trip wire to release a weighted, 10-foot-square Net suspended from the ceiling. The trip wire is 3 inches off the ground and stretches between two columns or trees.

The first creature that crosses the trip wire causes the Net to fall on it. The target must succeed on a DC 10 Dexterity saving throw or have the Restrained condition until it escapes. The target succeeds automatically if it’s Huge or larger. A creature can take an action to make a DC 10 Strength (Athletics) check, freeing itself or another creature within its reach from the Net on a successful check.

Detect and Disarm. As a Search action, a creature can examine the trapped area and make a DC 11 Wisdom (Perception) check, detecting the trip wire and the suspended Net on a successful check. Once detected, the trip wire can be easily cut or avoided (no ability check required).

Destroy the Net. Reducing the Net to 0 Hit Points frees any creature trapped in it (see “Adventuring Gear” for the Net’s statistics).

Set the Trap. A creature that has Thieves’ Tools and all the trap’s components (including a Net) can try to set a falling net trap, doing so with a successful DC 13 Dexterity (Sleight of Hand) check. Each attempt to set this trap takes 10 minutes.

At Higher Levels. You can scale the trap for higher levels by increasing the weight of the Net, which increases the save DC and the DC of the Strength (Athletics) check as follows: DC 12 at levels 5–10, DC 14 at levels 11–16, or DC 16 at levels 17–20.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/traps_falling_net ` |
| ` implementation ` | ` dnd2024:rules_traps_falling_net ` |

### Parameters

#### Trap

<!-- splat:values path=splat_rule/parameters/trap -->

| Field | Value |
| --- | --- |
| ` severity ` | ` nuisance ` |
| ` levels ` | ` [1, 4] ` |
| ` trigger ` | ` first_creature_crosses_tripwire ` |
| ` duration ` | ` instantaneous ` |

#### Geometry

<!-- splat:values path=splat_rule/parameters/geometry -->

| Field | Value |
| --- | --- |
| ` net_square_side_feet ` | ` 10 ` |
| ` tripwire_height_inches ` | ` 3 ` |
| ` tripwire_between ` | ` ["columns", "trees"] ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` 10 ` |
| ` automatic_success_size_minimum ` | ` huge ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` condition ` | ` restrained ` |
| ` until ` | ` escape_or_net_destroyed ` |

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
economy: action
check:
  ability: strength
  dc: 10
  skill: athletics
targets:
- self
- other_within_reach
```

#### Detection

<!-- splat:yaml path=splat_rule/parameters/detection -->

```yaml
economy: search
check:
  ability: wisdom
  dc: 11
  skill: perception
reveals:
- tripwire
- net
```

#### Disarm

<!-- splat:values path=splat_rule/parameters/disarm -->

| Field | Value |
| --- | --- |
| ` after_detection ` | ` ["cut_tripwire", "avoid_tripwire"] ` |
| ` check_required ` | ` false ` |

#### Destroy

<!-- splat:yaml path=splat_rule/parameters/destroy -->

```yaml
net_hp: 0
frees_all: true
statistics_source:
  key: dnd2024:equipment/net_1_gp
```

#### Set

<!-- splat:yaml path=splat_rule/parameters/set -->

```yaml
requires:
- thieves_tools
- all_components
- net
check:
  ability: dexterity
  dc: 13
  skill: sleight_of_hand
attempt_duration:
  value: 10
  unit: minutes
```

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
  Save and escape dc:
    path: save_and_escape_dc
    type: integer
-->

| Key | Levels | Save and escape dc |
| --- | --- | --- |
| ` row_1 ` | ` [1, 4] ` | ` 10 ` |
| ` row_2 ` | ` [5, 10] ` | ` 12 ` |
| ` row_3 ` | ` [11, 16] ` | ` 14 ` |
| ` row_4 ` | ` [17, 20] ` | ` 16 ` |
