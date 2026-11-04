---
splat_schema: 2
splat_id: fd9787b0-eaf0-561d-84ab-0ad3568376fd
splat_kind: creature
splat_key: dnd2024:creatures/ankheg
splat_name: Ankheg
splat_origin: dnd2024:reference/creature/ankheg
splat_sources:
- SRD 5.2.1, pp. 259, https://www.dndbeyond.com/srd
---

# Ankheg


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 45 (6d10 + 12) |
| Initiative | +0 (10) |
| Speed | 30 ft., Burrow 10 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 11 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 11 | +0 | +0 |
| Con | 14 | +2 | +2 |
| Int | 1 | -5 | -5 |
| Wis | 13 | +1 | +1 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 259 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Unaligned\
> AC 14 Initiative +0 (10)\
> HP 45 (6d10 + 12)\
> Speed 30 ft., Burrow 10 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 11 +0 +0 Con 14 +2 +2\
> Int 1 −5 −5 Wis 13 +1 +1 Cha 6 −2 −2\
> Senses Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 11\
> Languages None\
> CR 2 (XP 450; PB +2)


### Traits

Tunneler. The ankheg can burrow through solid rock at half its Burrow Speed and leaves a 10-foot-diameter tunnel in its wake.

### Actions

Bite. Melee Attack Roll: +5 (with Advantage if the target is Grappled by the ankheg), reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage plus 3 (1d6) Acid damage. If the target is a Large or smaller creature, it has the Grappled condition (escape DC 13).

Acid Spray (Recharge 6). Dexterity Saving Throw: DC 12, each creature in a 30-foot-long, 5-foot-wide Line. Failure: 14 (4d6) Acid damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/259|page 259]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ankheg ` |
| ` implementation ` | ` dnd2024:creature_ankheg ` |

### Parameters

#### Features

##### Tunneler

<!-- splat:values path=splat_rule/parameters/features/tunneler -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` burrow_material ` | ` solid_rock ` |
| ` speed_multiplier ` | ` 0.5 ` |
| ` tunnel_diameter_ft ` | ` 10 ` |

##### Bite

<!-- splat:values path=splat_rule/parameters/features/bite -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/bite/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 5
reach_ft: 5
advantage_if:
  target_grappled_by: self
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/bite/hit/damage
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
| ` row_1 ` | ` 10 ` | ` 2 ` | ` 6 ` | ` 3 ` | ` Slashing ` |
| ` row_2 ` | ` 3 ` | ` 1 ` | ` 6 ` | ` 0 ` | ` Acid ` |

###### Grapple

<!-- splat:values path=splat_rule/parameters/features/bite/hit/grapple -->

| Field | Value |
| --- | --- |
| ` condition ` | ` grappled ` |
| ` source ` | ` self ` |
| ` escape_dc ` | ` 13 ` |
| ` maximum_size ` | ` large ` |

##### Acid spray

<!-- splat:yaml path=splat_rule/parameters/features/acid_spray -->

```yaml
economy: action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: saving_throw
ability: dexterity
dc: 12
targets:
  kind: creature
  selection: each
  area:
    shape: line
    size_ft: 30
    origin: self
    width_ft: 5
failure:
  damage:
  - average: 14
    roll:
      count: 4
      sides: 6
      modifier: 0
    type: acid
success:
  damage_multiplier: 0.5
  other_effects: false
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` tremorsense ` | ` 60 ` |
| ` passive_perception ` | ` 11 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
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
> burrow_speed:
>   unit: ft
>   value: 10
> ```

