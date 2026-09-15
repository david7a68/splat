---
splat_schema: 1
splat_id: f0ae9855-72a2-5415-af89-73df62875bd5
splat_kind: rule
splat_key: dnd2024:rules/traps_collapsing_roof
splat_name: Collapsing Roof
splat_origin:
  key: dnd2024:reference/pages/199
splat_sources:
- title: SRD 5.2.1
  locator: pp. 199
  url: https://www.dndbeyond.com/srd
---

# Collapsing Roof

## Source text

<!-- SRD page 199 -->

Deadly Trap (Levels 1–4)\
Trigger: A creature crosses a trip wire\
Duration: Instantaneous\
This trap uses a trip wire to collapse an unstable\
section of ceiling. The trip wire is 3 inches off the\
ground and stretches between two weak supports\
that topple when the trip wire is pulled.\
The first creature that crosses the trip wire\
causes the supports to topple and the unstable sec-\
tion of ceiling to collapse. Each creature beneath the\
unstable section of ceiling must succeed on a DC 13\
Dexterity saving throw, taking 11 (2d10) Bludgeon-\
ing damage on a failed save or half as much damage\
on a successful one. Rubble from the collapse turns\
the trapped area into Difficult Terrain.\
Detect and Disarm. As a Search action, a creature\
can examine the trapped area and make a DC 11\
Wisdom (Perception) check, detecting the trip wire\
and the unstable section of ceiling on a successful\
check. Once detected, the trip wire can be easily cut\
or avoided (no ability check required).\
At Higher Levels. You can scale the trap for higher\
levels by increasing the damage and the save DC, as\
shown in the following table.\
Levels Bludgeoning Damage Save DC\
5–10 22 (4d10) 15\
11–16 55 (10d10) 17\
17–20 99 (18d10) 19\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/traps_collapsing_roof ` |
| ` implementation ` | ` dnd2024:rule_traps_collapsing_roof ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` severity ` | ` deadly ` |
| ` default_levels ` | ` [1, 4] ` |
| ` trigger ` | ` first_creature_crosses_trip_wire ` |
| ` wire_height_inches ` | ` 3 ` |
| ` support_count ` | ` 2 ` |
| ` duration ` | ` instantaneous ` |
| ` targets ` | ` creatures_beneath_unstable_ceiling ` |
| ` aftermath ` | ` area_becomes_difficult_terrain ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
dc: 13
failure:
  damage:
    dice:
      count: 2
      sides: 10
  average: 11
  type: bludgeoning
success:
  damage_fraction: 0.5
  rounding: floor
```

#### Detect

<!-- splat:values path=splat_rule/parameters/detect -->

| Field | Value |
| --- | --- |
| ` action ` | ` search ` |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` perception ` |
| ` dc ` | ` 11 ` |
| ` success_reveals ` | ` ["trip_wire", "unstable_ceiling"] ` |

#### Disarm or bypass

<!-- splat:values path=splat_rule/parameters/disarm_or_bypass -->

| Field | Value |
| --- | --- |
| ` requires ` | ` detected ` |
| ` options ` | ` ["cut_wire", "avoid_wire"] ` |
| ` check_required ` | ` false ` |

#### Higher level variants

<!-- splat:rows path=splat_rule/parameters/higher_level_variants -->

| Row | levels | average | dc |
| --- | --- | --- | --- |
| ` 0 ` | ` [5, 10] ` | ` 22 ` | ` 15 ` |
| ` 1 ` | ` [11, 16] ` | ` 55 ` | ` 17 ` |
| ` 2 ` | ` [17, 20] ` | ` 99 ` | ` 19 ` |

##### Damage

<!-- splat:yaml path=splat_rule/parameters/higher_level_variants/0/damage -->

```yaml
dice:
  count: 4
  sides: 10
```

##### Damage

<!-- splat:yaml path=splat_rule/parameters/higher_level_variants/1/damage -->

```yaml
dice:
  count: 10
  sides: 10
```

##### Damage

<!-- splat:yaml path=splat_rule/parameters/higher_level_variants/2/damage -->

```yaml
dice:
  count: 18
  sides: 10
```

#### Geometry assumption

<!-- splat:values path=splat_rule/parameters/geometry_assumption -->

| Field | Value |
| --- | --- |
| ` unstable_ceiling_area ` | ` gm_supplied_location ` |
