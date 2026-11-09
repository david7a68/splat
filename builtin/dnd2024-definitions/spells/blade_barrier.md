---
splat_schema: 2
splat_id: 97691d2b-03ba-59cb-a949-ec1ff72defd0
splat_kind: spell
splat_key: dnd2024:spells/blade_barrier
splat_name: Blade Barrier
splat_origin: dnd2024:reference/spell/blade_barrier
splat_sources:
- SRD 5.2.1, pp. 113, https://www.dndbeyond.com/srd
---

# Blade Barrier


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Evocation |
| Classes | Cleric |
| Casting Time | Action |
| Range | 90 feet |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 113 -->

Level 6 Evocation (Cleric)\
Casting Time: Action\
Range: 90 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

You create a wall of whirling blades made of magical energy. The wall appears within range and lasts for the duration. You make a straight wall up to 100 feet long, 20 feet high, and 5 feet thick, or a ringed wall up to 60 feet in diameter, 20 feet high, and 5 feet thick. The wall provides Three-Quarters Cover, and its space is Difficult Terrain.

Any creature in the wall’s space makes a Dexterity saving throw, taking 6d10 Force damage on a failed save or half as much damage on a successful one. A creature also makes that save if it enters the wall’s space or ends it turn there. A creature makes that save only once per turn.

Original layout: [[../../srd-5.2.1/pages/113|page 113]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/blade_barrier ` |
| ` implementation ` | ` dnd2024:spell_blade_barrier ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` area ` |
| ` within_range ` | ` true ` |

#### Wall

<!-- splat:values path=splat_rule/parameters/wall -->

| Field | Value |
| --- | --- |
| ` height_feet ` | ` 20 ` |
| ` thickness_feet ` | ` 5 ` |
| ` cover ` | ` three_quarters ` |
| ` difficult_terrain ` | ` true ` |
| ` duration ` | ` spell_duration ` |

##### Choices

<!-- splat:table
path: splat_rule/parameters/wall/choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Shape:
    path: shape
    type: symbol
  Max length feet:
    path: max_length_feet
    type: integer
    optional: true
  Max diameter feet:
    path: max_diameter_feet
    type: integer
    optional: true
-->

| Key | Shape | Max length feet | Max diameter feet |
| --- | --- | --- | --- |
| ` row_1 ` | ` Straight ` | ` 100 ` |  |
| ` row_2 ` | ` Ring ` |  | ` 60 ` |

#### Damage triggers

<!-- splat:values path=splat_rule/parameters/damage_triggers -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["spell.cast_completed", "area.entered", "turn.ended"] ` |
| ` scope ` | ` creature_in_wall ` |
| ` max_saves_per_turn ` | ` 1 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
failure:
  damage:
    dice: 6d10
    type: force
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 10
  unit: minute
```
