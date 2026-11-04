---
splat_schema: 2
splat_id: db6c7e4a-fec4-5e35-92da-3ba0bfc1d570
splat_kind: rule
splat_key: dnd2024:rules/traps_hidden_pit
splat_name: Hidden Pit
splat_origin: dnd2024:reference/pages/200
splat_sources:
- SRD 5.2.1, pp. 200, https://www.dndbeyond.com/srd
---

# Hidden Pit

## Source text

<!-- SRD page 200 -->

Nuisance Trap (Levels 1–4)

Trigger: A creature moves onto the pit’s lid\
Duration: Instantaneous

This 10-foot-deep pit has a hinged lid constructed from material identical to the surrounding floor. When a creature moves onto the lid, it swings open like a trapdoor, causing the creature to fall into the pit. The lid remains open thereafter.

A creature that falls into the pit takes 3 (1d6) Bludgeoning damage from the fall.

Detect and Disarm. As a Study action, a creature can examine the section of floor that forms the pit’s lid and make a DC 15 Intelligence (Investigation) check, detecting the pit on a successful check. Once the pit is detected, an Iron Spike or a similar object can be wedged between the pit’s lid and the surrounding floor to prevent the lid from opening, thereby making it safe to cross. The cover can also be held shut using Arcane Lock or similar magic.

Escape. A creature needs a Climb Speed, climbing gear, or magic such as Spider Climb to scale the pit’s smooth walls. You can make the pit easier to escape by adding cracks in the walls big enough to serve as handholds and footholds.

At Higher Levels. You can scale the trap for higher levels by increasing the pit’s depth and damage, as shown in the following table.

| Levels | Pit Depth | Bludgeoning Damage |
| --- | --- | --- |
| 5–10 | 30 feet | 10 (3d6) |
| 11–16 | 60 feet | 21 (6d6) |
| 17–20 | 120 feet | 42 (12d6) |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/traps_hidden_pit ` |
| ` implementation ` | ` dnd2024:rule_traps_hidden_pit ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` severity ` | ` nuisance ` |
| ` default_levels ` | ` [1, 4] ` |
| ` trigger ` | ` creature_moves_onto_lid ` |
| ` duration ` | ` instantaneous ` |
| ` pit_depth_feet ` | ` 10 ` |

#### Trigger effect

<!-- splat:yaml path=splat_rule/parameters/trigger_effect -->

```yaml
lid: opens_and_remains_open
creature: falls_into_pit
damage:
  dice:
    count: 1
    sides: 6
average: 3
damage_type: bludgeoning
```

#### Detect

<!-- splat:values path=splat_rule/parameters/detect -->

| Field | Value |
| --- | --- |
| ` action ` | ` study ` |
| ` ability ` | ` intelligence ` |
| ` skill ` | ` investigation ` |
| ` dc ` | ` 15 ` |
| ` success ` | ` detect_pit ` |

#### Disarm

<!-- splat:values path=splat_rule/parameters/disarm -->

| Field | Value |
| --- | --- |
| ` requires ` | ` detected ` |
| ` crossing_after_disarm ` | ` safe ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/disarm/options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Wedge:
    path: wedge
    type: symbol
    optional: true
  Result:
    path: result
    type: symbol
  Magic:
    path: magic
    type: reference
    optional: true
-->

| Key | Wedge | Result | Magic |
| --- | --- | --- | --- |
| ` row_1 ` | ` Iron spike or similar ` | ` Lid cannot open ` |  |
| ` row_2 ` |  | ` Lid held shut ` | [Arcane Lock](<../spells/arcane_lock.md> "dnd2024:spells/arcane_lock") |

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
wall: smooth
requires_any:
- climb_speed
- climbing_gear
- spell:
    key: dnd2024:spells/spider_climb
- similar_magic
gm_easier_variant: cracks_for_handholds_and_footholds
```

#### Higher level variants

<!-- splat:table
path: splat_rule/parameters/higher_level_variants
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Levels:
    path: levels
    type: yaml
  Depth feet:
    path: depth_feet
    type: integer
  Damage dice count:
    path: damage/dice/count
    type: integer
  Damage dice sides:
    path: damage/dice/sides
    type: integer
  Average:
    path: average
    type: integer
-->

| Key | Levels | Depth feet | Damage dice count | Damage dice sides | Average |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` [5, 10] ` | ` 30 ` | ` 3 ` | ` 6 ` | ` 10 ` |
| ` row_2 ` | ` [11, 16] ` | ` 60 ` | ` 6 ` | ` 6 ` | ` 21 ` |
| ` row_3 ` | ` [17, 20] ` | ` 120 ` | ` 12 ` | ` 6 ` | ` 42 ` |
