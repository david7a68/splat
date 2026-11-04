---
splat_schema: 2
splat_id: 0ffb9a54-fb7c-569e-b970-25d02576365e
splat_kind: creature
splat_key: dnd2024:creatures/homunculus
splat_name: Homunculus
splat_origin: dnd2024:reference/creature/homunculus
splat_sources:
- SRD 5.2.1, pp. 299, https://www.dndbeyond.com/srd
---

# Homunculus


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Construct |
| Alignment | Neutral |
| Armor Class | 13 |
| Hit Points | 4 (1d4 + 2) |
| Initiative | +2 (12) |
| Speed | 20 ft., Fly 40 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | Understands Common plus one other language but can’t speak |
| Immunities | Poison; Charmed, Poisoned |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 4 | -3 | -3 |
| Dex | 15 | +2 | +2 |
| Con | 14 | +2 | +2 |
| Int | 10 | +0 | +0 |
| Wis | 10 | +0 | +2 |
| Cha | 7 | -2 | +0 |


## Source text

<!-- SRD page 299 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Construct, Neutral\
> AC 13 Initiative +2 (12)\
> HP 4 (1d4 + 2)\
> Speed 20 ft., Fly 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 4 −3 −3 Dex 15 +2 +2 Con 14 +2 +2\
> Int 10 +0 +0 Wis 10 +0 +2 Cha 7 −2 +0\
> Immunities Poison; Charmed, Poisoned\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages Understands Common plus one other language but can’t speak\
> CR 0 (XP 10; PB +2)


### Traits

Telepathic Bond. While the homunculus is on the same plane of existence as its master, the two of them can communicate telepathically with each other.

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing damage, and the target is subjected to the following effect. Constitution Saving Throw: DC 12. Failure: The target has the Poisoned condition until the end of the homunculus’s next turn. Failure by 5 or More: The target has the Poisoned condition for 1 minute. While Poisoned, the target has the Unconscious condition, which ends early if the target takes any damage.

Original layout: [[../../srd-5.2.1/pages/299|page 299]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/homunculus ` |
| ` implementation ` | ` dnd2024:creature_homunculus ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["charmed", "poisoned"] ` |

#### Features

##### Telepathic bond

<!-- splat:yaml path=splat_rule/parameters/features/telepathic_bond -->

```yaml
economy: trait
operation: passive_rule
telepathy:
  with: master
  requires_same_plane: true
  range: unlimited
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 1
    roll:
      constant: 1
    type: piercing
  save:
    operation: saving_throw
    ability: constitution
    dc: 12
    targets:
      kind: target
    failure:
      conditions:
      - condition: poisoned
        until: self.next_turn_end
    failure_by_5_or_more:
      conditions:
      - condition: poisoned
        duration_seconds: 60
      while_poisoned:
        condition: unconscious
        ends_on: target.damage_taken
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- common
additional_languages:
  count: 1
  chooser: creator
  can_speak: false
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

