---
splat_schema: 2
splat_id: fe1f5e23-0df2-50cc-a56c-77412a9a8bdb
splat_kind: creature
splat_key: dnd2024:creatures/sphinx_of_wonder
splat_name: Sphinx of Wonder
splat_origin: dnd2024:reference/creature/sphinx_of_wonder
splat_sources:
- SRD 5.2.1, pp. 327, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 291, Habitat
---

# Sphinx of Wonder


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Celestial |
| Alignment | Lawful Good |
| Armor Class | 13 |
| Hit Points | 24 (7d4 + 7) |
| Initiative | +3 (13) |
| Speed | 20 ft., Fly 40 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Arcana +4, Religion +4, Stealth +5 |
| Senses | Darkvision 60 ft.; Passive Perception 11 |
| Languages | Celestial, Common |
| Resistances | Necrotic, Psychic, Radiant |
| Habitat | Desert, Planar (Upper Planes) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 6 | -2 | -2 |
| Dex | 17 | +3 | +3 |
| Con | 13 | +1 | +1 |
| Int | 15 | +2 | +2 |
| Wis | 12 | +1 | +1 |
| Cha | 11 | +0 | +0 |


## Source text

<!-- SRD page 327 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Celestial, Lawful Good\
> AC 13  Initiative +3 (13)\
> HP 24 (7d4 + 7)\
> Speed 20 ft., Fly 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 6 −2 −2 Dex 17 +3 +3 Con 13 +1 +1\
> Int 15 +2 +2 Wis 12 +1 +1 Cha 11 +0 +0\
> Skills Arcana +4, Religion +4, Stealth +5\
> Resistances Necrotic, Psychic, Radiant\
> Senses Darkvision 60 ft.; Passive Perception 11\
> Languages Celestial, Common\
> CR 1 (XP 200; PB +2)


### Traits

Magic Resistance. The sphinx has Advantage on saving throws against spells and other magical effects.

### Actions

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Slashing damage plus 7 (2d6) Radiant damage.

### Reactions

Burst of Ingenuity (2/Day). Trigger: The sphinx or another creature within 30 feet makes an ability check or a saving throw. Response: The sphinx adds 2 to the roll.

Original layout: [[../../srd-5.2.1/pages/327|page 327]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/sphinx_of_wonder ` |
| ` implementation ` | ` dnd2024:creature_sphinx_of_wonder ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["necrotic", "psychic", "radiant"] ` |

#### Features

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Rend

<!-- splat:values path=splat_rule/parameters/features/rend -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/rend/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 5
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/rend/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 5 ` | ` 1 ` | ` 4 ` | ` 3 ` | ` Slashing ` |
| ` row_2 ` | ` 7 ` | ` 2 ` | ` 6 ` | ` 0 ` | ` Radiant ` |

##### Burst of ingenuity

<!-- splat:values path=splat_rule/parameters/features/burst_of_ingenuity -->

| Field | Value |
| --- | --- |
| ` economy ` | ` reaction ` |
| ` operation ` | ` modify_roll ` |
| ` trigger ` | ` ability_check_or_saving_throw ` |
| ` bonus ` | ` 2 ` |

###### Resource

<!-- splat:values path=splat_rule/parameters/features/burst_of_ingenuity/resource -->

| Field | Value |
| --- | --- |
| ` kind ` | ` daily ` |
| ` uses ` | ` 2 ` |
| ` recovery ` | ` long_rest ` |

###### Targets

###### Any of

<!-- splat:table
path: splat_rule/parameters/features/burst_of_ingenuity/targets/any_of
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Self:
    path: self
    type: boolean
    optional: true
  Within ft:
    path: within_ft
    type: integer
    optional: true
-->

| Key | Self | Within ft |
| --- | --- | --- |
| ` row_1 ` | true |  |
| ` row_2 ` |  | ` 30 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 4 ` |
| ` religion ` | ` 4 ` |
| ` stealth ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 11 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- celestial
- common
understands: []
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 40
> ```

