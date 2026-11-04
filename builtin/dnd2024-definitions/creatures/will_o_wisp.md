---
splat_schema: 2
splat_id: be1743d1-eb35-553a-8b6c-392e94eb3535
splat_kind: creature
splat_key: dnd2024:creatures/will_o_wisp
splat_name: Will-o’-Wisp
splat_origin: dnd2024:reference/creature/will_o_wisp
splat_sources:
- SRD 5.2.1, pp. 341, 342, https://www.dndbeyond.com/srd
---

# Will-o’-Wisp


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Undead |
| Alignment | Chaotic Evil |
| Armor Class | 19 |
| Hit Points | 27 (11d4) |
| Initiative | +9 (19) |
| Speed | 5 ft., Fly 50 ft. (hover) |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Darkvision 120 ft.; Passive Perception 12 |
| Languages | Common plus one other language |
| Resistances | Acid, Bludgeoning, Cold, Fire, Necrotic, Piercing, Slashing |
| Immunities | Lightning, Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 1 | -5 | -5 |
| Dex | 28 | +9 | +9 |
| Con | 10 | +0 | +0 |
| Int | 13 | +1 | +1 |
| Wis | 14 | +2 | +2 |
| Cha | 11 | +0 | +0 |


## Source text

<!-- SRD page 341 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Undead, Chaotic Evil\
> AC 19 Initiative +9 (19)\
> HP 27 (11d4)\
> Speed 5 ft., Fly 50 ft. (hover)\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 1 −5 −5 Dex 28 +9 +9 Con 10 +0 +0\
> Int 13 +1 +1 Wis 14 +2 +2 Cha 11 +0 +0\
> Resistances Acid, Bludgeoning, Cold, Fire, Necrotic, Piercing, Slashing\
> Immunities Lightning, Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious\
> Senses Darkvision 120 ft.; Passive Perception 12\
> Languages Common plus one other language\
> CR 2 (XP 450; PB +2)


<!-- SRD page 342 -->

### Traits

Ephemeral. The wisp can’t wear or carry anything.

Illumination. The wisp sheds Bright Light in a 20-foot radius and Dim Light for an additional 20 feet.

Incorporeal Movement. The wisp can move through other creatures and objects as if they were Difficult Terrain. It takes 5 (1d10) Force damage if it ends its turn inside an object.

### Actions

Shock. Melee Attack Roll: +4, reach 5 ft. Hit: 11 (2d8 + 2) Lightning damage.

### Bonus Actions

Consume Life. Constitution Saving Throw: DC 10, one living creature the wisp can see within 5 feet that has 0 Hit Points. Failure: The target dies, and the wisp regains 10 (3d6) Hit Points.

Vanish. The wisp and its light have the Invisible condition until the wisp’s Concentration ends on this effect, which ends early immediately after the wisp makes an attack roll or uses Consume Life.

Original layout: [[../../srd-5.2.1/pages/341|page 341]], [[../../srd-5.2.1/pages/342|page 342]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/will_o_wisp ` |
| ` implementation ` | ` dnd2024:creature_will_o_wisp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["acid", "bludgeoning", "cold", "fire", "necrotic", "piercing", "slashing"] ` |
| ` damage_immunities ` | ` ["lightning", "poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "grappled", "paralyzed", "petrified", "poisoned", "prone", "restrained", "unconscious"] ` |

#### Features

##### Ephemeral

<!-- splat:values path=splat_rule/parameters/features/ephemeral -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` can_wear ` | ` false ` |
| ` can_carry ` | ` false ` |

##### Illumination

<!-- splat:values path=splat_rule/parameters/features/illumination -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` bright_radius_ft ` | ` 20 ` |
| ` additional_dim_radius_ft ` | ` 20 ` |

##### Incorporeal movement

<!-- splat:yaml path=splat_rule/parameters/features/incorporeal_movement -->

```yaml
economy: trait
operation: passive_rule
pass_through:
- creatures
- objects
movement_terrain: difficult
end_turn_inside_object_damage:
  roll:
    count: 1
    sides: 10
    modifier: 0
  type: force
```

##### Shock

<!-- splat:yaml path=splat_rule/parameters/features/shock -->

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
  - average: 11
    roll:
      count: 2
      sides: 8
      modifier: 2
    type: lightning
```

##### Consume life

<!-- splat:yaml path=splat_rule/parameters/features/consume_life -->

```yaml
economy: bonus_action
operation: saving_throw
ability: constitution
dc: 10
targets:
  kind: creature
  count: 1
  range_ft: 5
  visible: true
  living: true
  hp: 0
failure:
  target_dies: true
  heal_self:
    roll:
      count: 3
      sides: 6
      modifier: 0
    average: 10
success:
  effects: []
```

##### Vanish

<!-- splat:values path=splat_rule/parameters/features/vanish -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` operation ` | ` apply_conditions ` |
| ` targets ` | ` ["self", "self.light"] ` |
| ` conditions ` | ` ["invisible"] ` |
| ` concentration ` | ` true ` |
| ` ends_after ` | ` ["self.attack_roll", "self.use_consume_life"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 12 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
understands: []
additional_languages:
  count: 1
  chooser: creator
  can_speak: true
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
>   value: 50
> ```

