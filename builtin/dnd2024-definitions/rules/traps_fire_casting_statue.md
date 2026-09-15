---
splat_schema: 1
splat_id: 26fabfbe-7fc0-5d8f-8a44-013a4f8e1d7d
splat_kind: rule
splat_key: dnd2024:rules/traps_fire_casting_statue
splat_name: Fire-Casting Statue
splat_origin:
  key: dnd2024:reference/pages/199
splat_sources:
- title: SRD 5.2.1
  locator: pp. 199, 200
  url: https://www.dndbeyond.com/srd
---

# Fire-Casting Statue

## Source text

<!-- SRD page 199 -->

Deadly Trap (Levels 1–4)\
Trigger: A creature moves onto a pressure plate\

<!-- SRD page 200 -->

Duration: Instantaneous, and the trap resets at the\
start of the next turn\
When a creature moves onto this trap’s pressure\
plate for the first time on a turn or starts its turn\
there, a nearby statue exhales a 15-foot Cone of\
magical flame. The statue can look like anything,\
such as a dragon or a wizard. Each creature in the\
Cone must succeed on a DC 15 Dexterity saving\
throw, taking 11 (2d10) Fire damage on a failed\
save or half as much damage on a successful one.\
Detect and Disarm. A Detect Magic spell reveals\
an aura of Evocation magic around the statue. As a\
Search action, a creature within 5 feet of the statue\
can examine it and make a DC 10 Wisdom (Percep-\
tion) check, detecting a tiny glyph on the statue on a\
successful check. Once the glyph is found, a charac-\
ter can take a Study action to examine it and make a\
DC 15 Intelligence (Arcana) check, ascertaining that\
the glyph means “fire” on a successful check. As an\
action, a character can use a sharp tool to deface the\
glyph, which disarms the trap.\
As a Search action, a creature can examine the\
section of floor that forms the pressure plate and\
make a DC 15 Wisdom (Perception) check, detecting\
the pressure plate on a successful check. Wedging\
an Iron Spike or similar object under the pressure\
plate prevents the trap from triggering.\
At Higher Levels. You can scale the trap for higher\
levels by increasing the damage and the area of ef-\
fect, as shown in the following table.\
Levels Fire Damage Area of Effect\
5–10 22 (4d10) 30-foot Cone\
11–16 55 (10d10) 60-foot Cone\
17–20 99 (18d10) 120-foot Cone\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/traps_fire_casting_statue ` |
| ` implementation ` | ` dnd2024:rules_traps_fire_casting_statue ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` classification ` | ` deadly ` |
| ` triggers ` | ` ["pressure_plate_first_entry_on_turn", "turn_started_on_pressure_plate"] ` |
| ` duration ` | ` instantaneous ` |
| ` reset ` | ` next_turn_start ` |
| ` success_damage ` | ` half_round_down ` |

#### Base levels

<!-- splat:values path=splat_rule/parameters/base_levels -->

| Field | Value |
| --- | --- |
| ` min ` | ` 1 ` |
| ` max ` | ` 4 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cone ` |
| ` length_ft ` | ` 15 ` |
| ` origin ` | ` statue ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` 15 ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
dice:
  count: 2
  sides: 10
average: 11
type: fire
```

#### Detect magic

<!-- splat:values path=splat_rule/parameters/detect_magic -->

| Field | Value |
| --- | --- |
| ` aura ` | ` evocation ` |
| ` location ` | ` statue ` |

#### Examine statue

<!-- splat:values path=splat_rule/parameters/examine_statue -->

| Field | Value |
| --- | --- |
| ` action ` | ` search ` |
| ` distance_max_ft ` | ` 5 ` |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` perception ` |
| ` dc ` | ` 10 ` |
| ` success ` | ` reveal_glyph ` |

#### Decipher glyph

<!-- splat:values path=splat_rule/parameters/decipher_glyph -->

| Field | Value |
| --- | --- |
| ` requires_glyph_found ` | ` true ` |
| ` action ` | ` study ` |
| ` ability ` | ` intelligence ` |
| ` skill ` | ` arcana ` |
| ` dc ` | ` 15 ` |
| ` success ` | ` identify_fire_symbol ` |

#### Disarm glyph

<!-- splat:values path=splat_rule/parameters/disarm_glyph -->

| Field | Value |
| --- | --- |
| ` requires_glyph_found ` | ` true ` |
| ` action_cost ` | ` 1 ` |
| ` tool ` | ` sharp ` |
| ` operation ` | ` deface_glyph ` |
| ` result ` | ` trap_disarmed ` |

#### Examine floor

<!-- splat:values path=splat_rule/parameters/examine_floor -->

| Field | Value |
| --- | --- |
| ` action ` | ` search ` |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` perception ` |
| ` dc ` | ` 15 ` |
| ` success ` | ` reveal_pressure_plate ` |

#### Block pressure plate

<!-- splat:values path=splat_rule/parameters/block_pressure_plate -->

| Field | Value |
| --- | --- |
| ` object_choice ` | ` ["iron_spike", "similar_wedge"] ` |
| ` result ` | ` triggering_prevented ` |

#### Optional level scaling

<!-- splat:table
path: splat_rule/parameters/optional_level_scaling
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Levels min:
    path: levels/min
    type: integer
  Levels max:
    path: levels/max
    type: integer
  Damage dice count:
    path: damage/dice/count
    type: integer
  Damage dice sides:
    path: damage/dice/sides
    type: integer
  Damage average:
    path: damage/average
    type: integer
  Cone length ft:
    path: cone_length_ft
    type: integer
-->

| Key | Levels min | Levels max | Damage dice count | Damage dice sides | Damage average | Cone length ft |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 5 ` | ` 10 ` | ` 4 ` | ` 10 ` | ` 22 ` | ` 30 ` |
| ` row_2 ` | ` 11 ` | ` 16 ` | ` 10 ` | ` 10 ` | ` 55 ` | ` 60 ` |
| ` row_3 ` | ` 17 ` | ` 20 ` | ` 18 ` | ` 10 ` | ` 99 ` | ` 120 ` |
