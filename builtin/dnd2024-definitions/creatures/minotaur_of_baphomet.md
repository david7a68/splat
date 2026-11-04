---
splat_schema: 2
splat_id: 7d967769-0c10-5986-91ca-3bb88fcfa044
splat_kind: creature
splat_key: dnd2024:creatures/minotaur_of_baphomet
splat_name: Minotaur of Baphomet
splat_origin: dnd2024:reference/creature/minotaur_of_baphomet
splat_sources:
- SRD 5.2.1, pp. 309, https://www.dndbeyond.com/srd
---

# Minotaur of Baphomet


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Chaotic Evil |
| Armor Class | 14 |
| Hit Points | 85 (10d10 + 30) |
| Initiative | +0 (10) |
| Speed | 40 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Skills | Perception +7, Survival +7 |
| Senses | Darkvision 60 ft.; Passive Perception 17 |
| Languages | Abyssal |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 11 | +0 | +0 |
| Con | 16 | +3 | +3 |
| Int | 6 | -2 | -2 |
| Wis | 16 | +3 | +3 |
| Cha | 9 | -1 | -1 |


## Source text

<!-- SRD page 309 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Chaotic Evil\
> AC 14 Initiative +0 (10)\
> HP 85 (10d10 + 30)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 11 +0 +0 Con 16 +3 +3\
> Int 6 −2 −2 Wis 16 +3 +3 Cha 9 −1 −1\
> Skills Perception +7, Survival +7\
> Senses Darkvision 60 ft.; Passive Perception 17\
> Languages Abyssal\
> CR 3 (XP 700; PB +2)


### Actions

Abyssal Glaive. Melee Attack Roll: +6, reach 10 ft. Hit: 10 (1d12 + 4) Slashing damage plus 10 (3d6) Necrotic damage.

Gore (Recharge 5–6). Melee Attack Roll: +6, reach 5 ft. Hit: 18 (4d6 + 4) Piercing damage. If the target is a Large or smaller creature and the minotaur moved 10+ feet straight toward it immediately before the hit, the target takes an extra 10 (3d6) Piercing damage and has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/309|page 309]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/minotaur_of_baphomet ` |
| ` implementation ` | ` dnd2024:creature_minotaur_of_baphomet ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 17 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 7 ` |
| ` survival ` | ` 7 ` |

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
| ` known ` | ` ["abyssal"] ` |

#### Features

##### Abyssal glaive

<!-- splat:yaml path=splat_rule/parameters/features/abyssal_glaive -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 12
      add: 4
    average: 10
  - type: necrotic
    amount:
      dice:
        count: 3
        sides: 6
      add: 0
    average: 10
```

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

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
  - type: piercing
    amount:
      dice:
        count: 4
        sides: 6
      add: 4
    average: 18
- op: conditional
  when:
    target_maximum_size: large
    actor_moved_straight_toward_target_ft_at_least: 10
    immediately_before_hit: true
    target_kind: creature
  effects:
  - op: damage
    components:
    - type: piercing
      amount:
        dice:
          count: 3
          sides: 6
        add: 0
      average: 10
  - op: condition.apply
    condition: prone
recharge:
  die_sides: 6
  success_min: 5
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
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
