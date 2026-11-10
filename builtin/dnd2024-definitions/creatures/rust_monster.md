---
splat_schema: 2
splat_id: 83a28d81-b401-51e6-a020-e6523168810e
splat_kind: creature
splat_key: dnd2024:creatures/rust_monster
splat_name: Rust Monster
splat_origin: dnd2024:reference/creature/rust_monster
splat_sources:
- SRD 5.2.1, pp. 320, 321, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 263, Habitat
---

# Rust Monster


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 33 (6d8 + 6) |
| Initiative | +1 (11) |
| Speed | 40 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 11 |
| Languages | None |
| Habitat | Underdark |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 13 | +1 | +1 |
| Dex | 12 | +1 | +1 |
| Con | 13 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 13 | +1 | +1 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 320 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Monstrosity, Unaligned\
> AC 14  Initiative +1 (11)\
> HP 33 (6d8 + 6)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 13 +1 +1 Dex 12 +1 +1 Con 13 +1 +1\
> Int 2 −4 −4 Wis 13 +1 +1 Cha 6 −2 −2\
> Senses Darkvision 60 ft.; Passive Perception 11\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Traits

Iron Scent. The rust monster can pinpoint the location of ferrous metal within 30 feet of itself.

### Actions

Multiattack. The rust monster makes one Bite attack and uses Antennae twice.

Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d8 + 1) Piercing damage.

<!-- SRD page 321 -->

Antennae. The rust monster targets one nonmagical metal object—armor or a weapon—worn or carried by a creature within 5 feet of itself. Dexterity Saving Throw: DC 11, the creature with the object. Failure: The object takes a −1 penalty to the AC it offers (armor) or to its attack rolls (weapon). Armor is destroyed if the penalty reduces its AC to 10, and a weapon is destroyed if its penalty reaches −5. The penalty can be removed by casting the Mending spell on the armor or weapon.

Destroy Metal. The rust monster touches a nonmagical metal object within 5 feet of itself that isn’t being worn or carried. The touch destroys a 1-foot Cube of the object.

### Reactions

Reflexive Antennae. Trigger: An attack roll hits the rust monster. Response: The rust monster uses Antennae.

Original layout: [[../../srd-5.2.1/pages/320|page 320]], [[../../srd-5.2.1/pages/321|page 321]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/rust_monster ` |
| ` implementation ` | ` dnd2024:creature_rust_monster ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Iron scent

<!-- splat:values path=splat_rule/parameters/features/iron_scent -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` location.pinpoint ` |
| ` material ` | ` ferrous_metal ` |
| ` range_ft ` | ` 30 ` |

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 3
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 8
      add: 1
    average: 5
```

##### Antennae

<!-- splat:yaml path=splat_rule/parameters/features/antennae -->

```yaml
economy: action
op: item.target
target:
  kind: object
  magical: false
  material: metal
  categories:
  - armor
  - weapon
  worn_or_carried_by_creature: true
  range_ft: 5
bearer_save:
  op: saving_throw
  ability: dexterity
  dc: 11
  targets:
    kind: item_bearer
  failure:
  - op: item.penalty
    cumulative: true
    amount: -1
    armor:
      stat: provided_ac
      destroy_at_ac: 10
    weapon:
      stat: attack_roll
      destroy_at_penalty: -5
    remove_penalty_spell:
      key: dnd2024:spells/mending
  success: []
```

##### Destroy metal

<!-- splat:yaml path=splat_rule/parameters/features/destroy_metal -->

```yaml
economy: action
op: object.destroy_volume
range_ft: 5
requires_touch: true
target:
  magical: false
  material: metal
  worn_or_carried: false
volume:
  shape: cube
  edge_ft: 1
```

##### Reflexive antennae

<!-- splat:yaml path=splat_rule/parameters/features/reflexive_antennae -->

```yaml
economy: reaction
trigger: attack.hits_actor
effect:
  op: feature.use
  feature: antennae
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` multiattack ` |

###### Sequence

<!-- splat:table
path: splat_rule/parameters/features/multiattack/sequence
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Feature | Count |
| --- | --- | --- |
| ` row_1 ` | ` Bite ` | ` 1 ` |
| ` row_2 ` | ` Antennae ` | ` 2 ` |

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spellcasting defaults

<!-- splat:yaml path=splat_rule/parameters/spellcasting_defaults -->

```yaml
cast_level: lowest_possible_unless_overridden
upcast_allowed: only_explicit_cast_level_overrides
required_material_components_available: true
casting_time_at_least_1_minute:
  uses_normal_time: true
  magic_action_each_turn: true
  concentration_required: true
other_components: spell_definition_unless_waived
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
