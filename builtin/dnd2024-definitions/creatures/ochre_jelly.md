---
splat_schema: 2
splat_id: 2d3a918a-f418-5eea-a832-45d6ee544c4c
splat_kind: creature
splat_key: dnd2024:creatures/ochre_jelly
splat_name: Ochre Jelly
splat_origin: dnd2024:reference/creature/ochre_jelly
splat_sources:
- SRD 5.2.1, pp. 312, https://www.dndbeyond.com/srd
---

# Ochre Jelly


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Ooze |
| Alignment | Unaligned |
| Armor Class | 8 |
| Hit Points | 52 (7d10 + 14) |
| Initiative | -2 (8) |
| Speed | 20 ft., Climb 20 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Blindsight 60 ft.; Passive Perception 8 |
| Languages | None |
| Resistances | Acid |
| Immunities | Lightning, Slashing; Charmed, Deafened, Exhaustion, Frightened, Grappled, Prone, Restrained |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 6 | -2 | -2 |
| Con | 14 | +2 | +2 |
| Int | 2 | -4 | -4 |
| Wis | 6 | -2 | -2 |
| Cha | 1 | -5 | -5 |


## Source text

<!-- SRD page 312 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Ooze, Unaligned\
> AC 8 Initiative −2 (8)\
> HP 52 (7d10 + 14)\
> Speed 20 ft., Climb 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 6 −2 −2 Con 14 +2 +2\
> Int 2 −4 −4 Wis 6 −2 −2 Cha 1 −5 −5\
> Resistances Acid\
> Immunities Lightning, Slashing; Charmed, Deafened, Exhaustion, Frightened, Grappled, Prone, Restrained\
> Senses Blindsight 60 ft.; Passive Perception 8\
> Languages None\
> CR 2 (XP 450; PB +2)


### Traits

Amorphous. The jelly can move through a space as narrow as 1 inch without expending extra movement to do so.

Spider Climb. The jelly can climb difficult surfaces, including along ceilings, without needing to make an ability check.

### Actions

Pseudopod. Melee Attack Roll: +4, reach 5 ft. Hit: 12 (3d6 + 2) Acid damage.

### Reactions

Split. Trigger: While the jelly is Large or Medium and has 10+ Hit Points, it becomes Bloodied or is subjected to Lightning or Slashing damage. Response: The jelly splits into two new Ochre Jellies. Each new jelly is one size smaller than the original jelly and acts on its Initiative. The original jelly’s Hit Points are divided evenly between the new jellies (round down).

Original layout: [[../../srd-5.2.1/pages/312|page 312]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ochre_jelly ` |
| ` implementation ` | ` dnd2024:creature_ochre_jelly ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["acid"] ` |
| ` damage_immunities ` | ` ["lightning", "slashing"] ` |
| ` condition_immunities ` | ` ["charmed", "deafened", "exhaustion", "frightened", "grappled", "prone", "restrained"] ` |
| ` passive_perception ` | ` 8 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Amorphous

<!-- splat:values path=splat_rule/parameters/features/amorphous -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` movement.form ` |
| ` minimum_opening_inches ` | ` 1 ` |
| ` extra_movement_cost ` | ` 0 ` |

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` movement.climb ` |
| ` difficult_surfaces ` | ` true ` |
| ` ceilings ` | ` true ` |
| ` ability_check_required ` | ` false ` |

##### Pseudopod

<!-- splat:yaml path=splat_rule/parameters/features/pseudopod -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: acid
    amount:
      dice:
        count: 3
        sides: 6
      add: 2
    average: 12
```

##### Split

<!-- splat:yaml path=splat_rule/parameters/features/split -->

```yaml
economy: reaction
trigger_any:
- actor.becomes_bloodied
- actor.subjected_to_lightning_damage
- actor.subjected_to_slashing_damage
requires:
  sizes:
  - large
  - medium
  hp_at_least: 10
op: actor.replace_with_creatures
definition:
  key: dnd2024:creatures/ochre_jelly
count: 2
each:
  size_steps: -1
  initiative: original_actor
  hp:
    divide:
    - actor_stat: hp
    - 2
    round: down
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> climb_speed:
>   unit: ft
>   value: 20
> ```

