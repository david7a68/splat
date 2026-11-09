---
splat_schema: 2
splat_id: 2268e2e6-8129-53ff-8c67-b9ce03805d92
splat_kind: creature
splat_key: dnd2024:creatures/black_pudding
splat_name: Black Pudding
splat_origin: dnd2024:reference/creature/black_pudding
splat_sources:
- SRD 5.2.1, pp. 265, 266, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 42, Habitat
---

# Black Pudding


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Ooze |
| Alignment | Unaligned |
| Armor Class | 7 |
| Hit Points | 68 (8d10 + 24) |
| Initiative | -3 (7) |
| Speed | 20 ft., Climb 20 ft. |
| Challenge | 4 (XP 1,100; PB +2) |
| Senses | Blindsight 60 ft.; Passive Perception 8 |
| Languages | None |
| Immunities | Acid, Cold, Lightning, Slashing; Charmed, Deafened, Exhaustion, Frightened, Grappled, Prone, Restrained |
| Habitat | Underdark |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 5 | -3 | -3 |
| Con | 16 | +3 | +3 |
| Int | 1 | -5 | -5 |
| Wis | 6 | -2 | -2 |
| Cha | 1 | -5 | -5 |


## Source text

<!-- SRD page 265 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Ooze, Unaligned\
> AC 7 Initiative −3 (7)\
> HP 68 (8d10 + 24)\
> Speed 20 ft., Climb 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 5 −3 −3 Con 16 +3 +3\
> Int 1 −5 −5 Wis 6 −2 −2 Cha 1 −5 −5\
> Immunities Acid, Cold, Lightning, Slashing; Charmed, Deafened, Exhaustion, Frightened, Grappled, Prone, Restrained\
> Senses Blindsight 60 ft.; Passive Perception 8\
> Languages None\
> CR 4 (XP 1,100; PB +2)


### Traits

Amorphous. The pudding can move through a space as narrow as 1 inch without expending extra movement to do so.

Corrosive Form. A creature that hits the pudding with a melee attack roll takes 4 (1d8) Acid damage. Nonmagical ammunition is destroyed immediately after hitting the pudding and dealing any damage. Any nonmagical weapon takes a cumulative −1 penalty to attack rolls immediately after dealing damage to the pudding and coming into contact with it. The weapon is destroyed if the penalty reaches −5. The penalty can be removed by casting the Mending spell on the weapon.

In 1 minute, the pudding can eat through 2 feet of nonmagical wood or metal.

Spider Climb. The pudding can climb difficult surfaces, including along ceilings, without needing to make an ability check.

### Actions

Dissolving Pseudopod. Melee Attack Roll: +5, reach 10 ft. Hit: 17 (4d6 + 3) Acid damage. Nonmagical armor worn by the target takes a −1 penalty to the AC it offers. The armor is destroyed if the penalty reduces its AC to 10. The penalty can be removed by casting the Mending spell on the armor.

### Reactions

Split. Trigger: While the pudding is Large or Medium and has 10+ Hit Points, it becomes Bloodied or is subjected to Lightning or Slashing damage. Response: The pudding splits into two new Black Puddings. Each new pudding is one size smaller than the original pudding and acts on its Initiative. The original pudding’s Hit Points are divided evenly between the new puddings (round down).

Original layout: [[../../srd-5.2.1/pages/265|page 265]], [[../../srd-5.2.1/pages/266|page 266]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/black_pudding ` |
| ` implementation ` | ` dnd2024:creature_black_pudding ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["acid", "cold", "lightning", "slashing"] ` |
| ` condition_immunities ` | ` ["charmed", "deafened", "exhaustion", "frightened", "grappled", "prone", "restrained"] ` |

#### Features

##### Amorphous

<!-- splat:values path=splat_rule/parameters/features/amorphous -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` minimum_gap_inches ` | ` 1 ` |
| ` extra_movement_cost ` | ` 0 ` |

##### Corrosive form

<!-- splat:yaml path=splat_rule/parameters/features/corrosive_form -->

```yaml
economy: trait
operation: passive_rule
on_melee_attack_hit:
  attacker_damage:
  - roll:
      count: 1
      sides: 8
      modifier: 0
    type: acid
on_ammunition_damage:
  requires_nonmagical: true
  destroy_ammunition: true
on_weapon_damage_contact:
  requires_nonmagical: true
  cumulative_attack_penalty: -1
  destroy_at_penalty: -5
  remove_penalty_spell:
    key: dnd2024:spells/mending
consume_material:
  materials:
  - nonmagical_wood
  - nonmagical_metal
  depth_ft: 2
  seconds: 60
```

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` climb_difficult_surfaces ` | ` true ` |
| ` climb_ceilings ` | ` true ` |
| ` ability_check_required ` | ` false ` |

##### Dissolving pseudopod

<!-- splat:yaml path=splat_rule/parameters/features/dissolving_pseudopod -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 10
hit:
  damage:
  - average: 17
    roll:
      count: 4
      sides: 6
      modifier: 3
    type: acid
  armor_corrosion:
    worn: true
    nonmagical: true
    cumulative_ac_penalty: -1
    destroy_at_ac: 10
    remove_penalty_spell:
      key: dnd2024:spells/mending
```

##### Split

<!-- splat:yaml path=splat_rule/parameters/features/split -->

```yaml
economy: reaction
operation: split_creature
trigger:
  any_of:
  - self.becomes_bloodied
  - self.subjected_to_lightning_damage
  - self.subjected_to_slashing_damage
requires:
  size:
  - medium
  - large
  hp_minimum: 10
remove_original: true
offspring:
  definition:
    key: dnd2024:creatures/black_pudding
  count: 2
  size_steps: -1
  hp_each:
    floor:
      divide:
      - stat: self.current_hp
      - 2
  initiative: original
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 60 ` |
| ` passive_perception ` | ` 8 ` |

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
>   value: 20
> ```

