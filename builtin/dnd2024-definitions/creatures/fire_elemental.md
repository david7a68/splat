---
splat_schema: 2
splat_id: 14fca385-727c-5c85-a854-f79578a20735
splat_kind: creature
splat_key: dnd2024:creatures/fire_elemental
splat_name: Fire Elemental
splat_origin: dnd2024:reference/creature/fire_elemental
splat_sources:
- SRD 5.2.1, pp. 284, 285, https://www.dndbeyond.com/srd
---

# Fire Elemental


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Elemental |
| Alignment | Neutral |
| Armor Class | 13 |
| Hit Points | 93 (11d10 + 33) |
| Initiative | +3 (13) |
| Speed | 50 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | Primordial (Ignan) |
| Resistances | Bludgeoning, Piercing, Slashing |
| Immunities | Fire, Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 10 | +0 | +0 |
| Dex | 17 | +3 | +3 |
| Con | 16 | +3 | +3 |
| Int | 6 | -2 | -2 |
| Wis | 10 | +0 | +0 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 284 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Elemental, Neutral\
> AC 13 Initiative +3 (13)\
> HP 93 (11d10 + 33)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 10 +0 +0 Dex 17 +3 +3 Con 16 +3 +3\
> Int 6 −2 −2 Wis 10 +0 +0 Cha 7 −2 −2\
> Resistances Bludgeoning, Piercing, Slashing\
> Immunities Fire, Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages Primordial (Ignan)\
> CR 5 (XP 1,800; PB +3)


### Traits

Fire Aura. At the end of each of the elemental’s turns, each creature in a 10-foot Emanation originating from the elemental takes 5 (1d10) Fire damage. Creatures and flammable objects in the Emanation start burning.

Fire Form. The elemental can move through a space as narrow as 1 inch without expending extra movement to do so, and it can enter a creature’s space and stop there. The first time it enters a creature’s space on a turn, that creature takes 5 (1d10) Fire damage.

Illumination. The elemental sheds Bright Light in a 30-foot radius and Dim Light for an additional 30 feet.

<!-- SRD page 285 -->

Water Susceptibility. The elemental takes 3 (1d6) Cold damage for every 5 feet the elemental moves in water or for every gallon of water splashed on it.

### Actions

Multiattack. The elemental makes two Burn attacks.

Burn. Melee Attack Roll: +6, reach 5 ft. Hit: 10 (2d6 + 3) Fire damage. If the target is a creature or a flammable object, it starts burning.

Original layout: [[../../srd-5.2.1/pages/284|page 284]], [[../../srd-5.2.1/pages/285|page 285]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/fire_elemental ` |
| ` implementation ` | ` dnd2024:creature_fire_elemental ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` damage_immunities ` | ` ["fire", "poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "grappled", "paralyzed", "petrified", "poisoned", "prone", "restrained", "unconscious"] ` |
| ` passive_perception ` | ` 10 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["primordial"] ` |
| ` primordial_dialects ` | ` ["ignan"] ` |

#### Features

##### Fire form

<!-- splat:yaml path=splat_rule/parameters/features/fire_form -->

```yaml
economy: passive
op: movement.form
minimum_opening_inches: 1
extra_movement_cost: 0
can_enter_and_end_in_creature_space: true
on_first_entry_per_creature_per_turn:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 1
        sides: 10
      add: 0
    average: 5
```

##### Illumination

<!-- splat:values path=splat_rule/parameters/features/illumination -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` light.emit ` |
| ` bright_radius_ft ` | ` 30 ` |
| ` dim_additional_ft ` | ` 30 ` |

##### Burn

<!-- splat:yaml path=splat_rule/parameters/features/burn -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 2
        sides: 6
      add: 3
    average: 10
- op: burning.start
  when_any:
  - target_kind: creature
  - target_kind: object
    flammable: true
```

##### Fire aura

<!-- splat:yaml path=splat_rule/parameters/features/fire_aura -->

```yaml
economy: passive
trigger: actor.turn_end
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    origin: actor
    size_ft: 10
effects:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 1
        sides: 10
      add: 0
    average: 5
additional_targets:
  kind:
  - creature
  - flammable_object
  area:
    shape: emanation
    origin: actor
    size_ft: 10
  effect:
    op: burning.start
```

##### Water susceptibility

<!-- splat:values path=splat_rule/parameters/features/water_susceptibility -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |

###### Triggers

<!-- splat:table
path: splat_rule/parameters/features/water_susceptibility/triggers
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Per distance ft:
    path: per_distance_ft
    type: integer
    optional: true
  Per volume gallons:
    path: per_volume_gallons
    type: integer
    optional: true
-->

| Key | Event | Per distance ft | Per volume gallons |
| --- | --- | --- | --- |
| ` row_1 ` | ` actor.moves_in_water ` | ` 5 ` |  |
| ` row_2 ` | ` water.splashed_on_actor ` |  | ` 1 ` |

###### Effects

<!-- splat:rows path=splat_rule/parameters/features/water_susceptibility/effects -->

| Row | op |
| --- | --- |
| ` 0 ` | ` damage ` |

###### Components

<!-- splat:rows path=splat_rule/parameters/features/water_susceptibility/effects/0/components -->

| Row | type | average |
| --- | --- | --- |
| ` 0 ` | ` cold ` | ` 3 ` |

###### Amount

<!-- splat:yaml path=splat_rule/parameters/features/water_susceptibility/effects/0/components/0/amount -->

```yaml
dice:
  count: 1
  sides: 6
add: 0
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: burn
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
