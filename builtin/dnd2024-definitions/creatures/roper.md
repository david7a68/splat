---
splat_schema: 2
splat_id: aad2d730-166b-50f4-acd0-c355c6b3b9e5
splat_kind: creature
splat_key: dnd2024:creatures/roper
splat_name: Roper
splat_origin: dnd2024:reference/creature/roper
splat_sources:
- SRD 5.2.1, pp. 320, https://www.dndbeyond.com/srd
---

# Roper


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Aberration |
| Alignment | Neutral Evil |
| Armor Class | 20 |
| Hit Points | 93 (11d10 + 33) |
| Initiative | +5 (15) |
| Speed | 10 ft., Climb 20 ft. |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Perception +6, Stealth +5 |
| Senses | Darkvision 60 ft.; Passive Perception 16 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 8 | -1 | -1 |
| Con | 17 | +3 | +3 |
| Int | 7 | -2 | -2 |
| Wis | 16 | +3 | +3 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 320 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Aberration, Neutral Evil\
> AC 20  Initiative +5 (15)\
> HP 93 (11d10 + 33)\
> Speed 10 ft., Climb 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 8 −1 −1 Con 17 +3 +3\
> Int 7 −2 −2 Wis 16 +3 +3 Cha 6 −2 −2\
> Skills Perception +6, Stealth +5\
> Senses Darkvision 60 ft.; Passive Perception 16\
> Languages None\
> CR 5 (XP 1,800; PB +3)


### Traits

Spider Climb. The roper can climb difficult surfaces, including along ceilings, without needing to make an ability check.

### Actions

Multiattack. The roper makes two Tentacle attacks, uses Reel, and makes two Bite attacks.

Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 17 (3d8 + 4) Piercing damage.

Tentacle. Melee Attack Roll: +7, reach 60 ft. Hit: The target has the Grappled condition (escape DC 14) from one of six tentacles, and the target has the Poisoned condition until the grapple ends.

The tentacle can be damaged, freeing a creature it has Grappled when destroyed (AC 20, HP 10, Immunity to Poison and Psychic damage). Damaging the tentacle deals no damage to the roper, and a destroyed tentacle regrows at the start of the roper’s next turn.

Reel. The roper pulls each creature Grappled by it up to 30 feet straight toward it.

Original layout: [[../../srd-5.2.1/pages/320|page 320]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/roper ` |
| ` implementation ` | ` dnd2024:creature_roper ` |

### Parameters

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` climb_ability_check_required ` | ` false ` |
| ` surfaces ` | ` ["difficult", "ceiling"] ` |

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: damage
  average: 17
  amount:
    dice:
      count: 3
      sides: 8
    add: 4
  damage_type: piercing
reach_feet: 5
```

##### Tentacle

<!-- splat:yaml path=splat_rule/parameters/features/tentacle -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: condition
  condition: grappled
  escape_dc: 14
  source: self
  appendages:
    kind: tentacle
    available: 6
    used: 1
- op: condition
  condition: poisoned
  ends:
    event: grapple.end
    source: self
reach_feet: 60
appendage:
  ac: 20
  hp: 10
  damage_immunities:
  - poison
  - psychic
  on_destroyed:
  - free_grappled_creature
  damage_transferred_to_self: false
  regrow:
    event: turn.start
    participant: self
    occurrence: next
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: tentacle
  count: 2
- op: use_feature
  feature: reel
  count: 1
- op: use_feature
  feature: bite
  count: 2
```

##### Reel

<!-- splat:yaml path=splat_rule/parameters/features/reel -->

```yaml
economy: action
op: pull
targets:
  count: all
  grappled_by: self
distance_max_feet: 30
direction: straight_toward_self
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |
| ` stealth ` | ` 5 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Understands

<!-- splat:yaml path=splat_rule/parameters/communication/understands -->

```yaml
[]
```

#### Stat block conventions

<!-- splat:yaml path=splat_rule/parameters/stat_block_conventions -->

```yaml
attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
save_dc: supplied_total
damage_mode_choice:
- fixed_average
- rolled_dice
condition_source: self
multiattack_child_economy_cost: 0
multiattack_child_resource_costs_retained: true
spellcasting:
  default_level: spell_minimum
  spell_slots_expended: false
  required_materials: available
  casting_time_at_least_one_minute:
    uses_normal_casting_duration: true
    magic_action_each_turn: true
    concentration_required: true
geometry_assumptions:
  chooser: gm
  supplies:
  - range
  - visibility
  - line_of_sight
  - area_membership
  - reachable_locations
  - unoccupied_locations
  - terrain
  - plane
  - form_appearance
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> climb_speed:
>   unit: ft
>   value: 20
> ```

