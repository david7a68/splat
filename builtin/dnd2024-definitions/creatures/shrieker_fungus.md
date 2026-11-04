---
splat_schema: 2
splat_id: 6b8e85fc-e125-5c0d-96a0-7c8f04cd52e9
splat_kind: creature
splat_key: dnd2024:creatures/shrieker_fungus
splat_name: Shrieker Fungus
splat_origin: dnd2024:reference/creature/shrieker_fungus
splat_sources:
- SRD 5.2.1, pp. 286, https://www.dndbeyond.com/srd
---

# Shrieker Fungus


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Plant |
| Alignment | Unaligned |
| Armor Class | 5 |
| Hit Points | 13 (3d8) |
| Initiative | -5 (5) |
| Speed | 5 ft. |
| Challenge | 0 (XP 0; PB +2) |
| Senses | Blindsight 30 ft.; Passive Perception 6 |
| Languages | None |
| Immunities | Blinded, Charmed, Deafened, Frightened |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 1 | -5 | -5 |
| Dex | 1 | -5 | -5 |
| Con | 10 | +0 | +0 |
| Int | 1 | -5 | -5 |
| Wis | 3 | -4 | -4 |
| Cha | 1 | -5 | -5 |


## Source text

<!-- SRD page 286 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Plant, Unaligned\
> AC 5 Initiative −5 (5)\
> HP 13 (3d8)\
> Speed 5 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 1 −5 −5 Dex 1 −5 −5 Con 10 +0 +0\
> Int 1 −5 −5 Wis 3 −4 −4 Cha 1 −5 −5\
> Immunities Blinded, Charmed, Deafened, Frightened\
> Senses Blindsight 30 ft.; Passive Perception 6\
> Languages None\
> CR 0 (XP 0; PB +2)


### Reactions

Shriek. Trigger: A creature or a source of Bright Light moves within 30 feet of the shrieker. Response: The shrieker emits a shriek audible within 300 feet of itself for 1 minute or until the shrieker dies.

Original layout: [[../../srd-5.2.1/pages/286|page 286]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/shrieker_fungus ` |
| ` implementation ` | ` dnd2024:creature_shrieker_fungus ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition_immunities ` | ` ["blinded", "charmed", "deafened", "frightened"] ` |
| ` passive_perception ` | ` 6 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 30 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Shriek

<!-- splat:yaml path=splat_rule/parameters/features/shriek -->

```yaml
economy: reaction
trigger: entity.moved
op: sound.emit
audible_radius_ft: 300
duration:
  maximum:
    amount: 1
    unit: minute
  until: actor.died
when:
  entity_kind_any:
  - creature
  - bright_light_source
  distance_to_actor_ft_at_most: 30
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
