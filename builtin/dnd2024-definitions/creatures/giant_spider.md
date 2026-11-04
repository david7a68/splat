---
splat_schema: 2
splat_id: d00bd6b3-23c5-5121-81d2-def8a283e438
splat_kind: creature
splat_key: dnd2024:creatures/giant_spider
splat_name: Giant Spider
splat_origin: dnd2024:reference/creature/giant_spider
splat_sources:
- SRD 5.2.1, pp. 353, 354, https://www.dndbeyond.com/srd
---

# Giant Spider


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 26 (4d10 + 4) |
| Initiative | +3 (13) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Perception +4, Stealth +7 |
| Senses | Darkvision 60 ft.; Passive Perception 14 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 16 | +3 | +3 |
| Con | 12 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 11 | +0 | +0 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 353 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 14  Initiative +3 (13)\
> HP 26 (4d10 + 4)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 14 +2 +2 Dex 16 +3 +3 Con 12 +1 +1\
> Int 2 −4 −4 Wis 11 +0 +0 Cha 4 −3 −3\
> Skills Perception +4, Stealth +7\
> Senses Darkvision 60 ft.; Passive Perception 14\
> Languages None\
> CR 1 (XP 200; PB +2)


<!-- SRD page 354 -->

### Traits

Spider Climb. The spider can climb difficult surfaces, including along ceilings, without needing to make an ability check.

Web Walker. The spider ignores movement restrictions caused by webs, and it knows the location of any other creature in contact with the same web.

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage plus 7 (2d6) Poison damage.

Web (Recharge 5–6). Dexterity Saving Throw: DC 13, one creature the spider can see within 60 feet. Failure: The target has the Restrained condition until the web is destroyed (AC 10; HP 5; Vulnerability to Fire damage; Immunity to Poison and Psychic damage).

Original layout: [[../../srd-5.2.1/pages/353|page 353]], [[../../srd-5.2.1/pages/354|page 354]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_spider ` |
| ` implementation ` | ` dnd2024:creature_giant_spider ` |

### Parameters

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` climb_difficult_surfaces ` | ` true ` |
| ` climb_ceilings ` | ` true ` |
| ` ability_check_required ` | ` false ` |

##### Web walker

<!-- splat:values path=splat_rule/parameters/features/web_walker -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` ignore_web_movement_restrictions ` | ` true ` |
| ` locate_creatures_on_same_web ` | ` true ` |

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
| ` row_1 ` | ` 7 ` | ` 1 ` | ` 8 ` | ` 3 ` | ` Piercing ` |
| ` row_2 ` | ` 7 ` | ` 2 ` | ` 6 ` | ` 0 ` | ` Poison ` |

##### Web

<!-- splat:yaml path=splat_rule/parameters/features/web -->

```yaml
economy: action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 5
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: saving_throw
ability: dexterity
dc: 13
targets:
  kind: creature
  count: 1
  range_ft: 60
  visible: true
failure:
  conditions:
  - condition: restrained
    until: web.destroyed
  web_object:
    ac: 10
    hp: 5
    damage_vulnerabilities:
    - fire
    damage_immunities:
    - poison
    - psychic
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 7 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 14 ` |

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
> climb_speed:
>   unit: ft
>   value: 30
> ```

