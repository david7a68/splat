---
splat_schema: 2
splat_id: bd94d4ab-8881-5488-a5b9-c44a1b8d2c62
splat_kind: creature
splat_key: dnd2024:creatures/goblin_warrior
splat_name: Goblin Warrior
splat_origin: dnd2024:reference/creature/goblin_warrior
splat_sources:
- SRD 5.2.1, pp. 290, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 142, Habitat
---

# Goblin Warrior


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Fey (Goblinoid) |
| Alignment | Chaotic Neutral |
| Armor Class | 15 |
| Hit Points | 10 (3d6) |
| Initiative | +2 (12) |
| Speed | 30 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Skills | Stealth +6 |
| Senses | Darkvision 60 ft.; Passive Perception 9 |
| Languages | Common, Goblin |
| Gear | Leather Armor, Scimitar, Shield, Shortbow |
| Habitat | Forest, Grassland, Hill, Underdark, Planar (Acheron, Feywild) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 8 | -1 | -1 |
| Dex | 15 | +2 | +2 |
| Con | 10 | +0 | +0 |
| Int | 10 | +0 | +0 |
| Wis | 8 | -1 | -1 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 290 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Fey (Goblinoid), Chaotic Neutral\
> AC 15 Initiative +2 (12)\
> HP 10 (3d6)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 8 −1 −1 Dex 15 +2 +2 Con 10 +0 +0\
> Int 10 +0 +0 Wis 8 −1 −1 Cha 8 −1 −1\
> Skills Stealth +6\
> Gear Leather Armor, Scimitar, Shield, Shortbow\
> Senses Darkvision 60 ft.; Passive Perception 9\
> Languages Common, Goblin\
> CR 1/4 (XP 50; PB +2)


### Actions

Scimitar. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage, plus 2 (1d4) Slashing damage if the attack roll had Advantage.

Shortbow. Ranged Attack Roll: +4, range 80/320 ft. Hit: 5 (1d6 + 2) Piercing damage, plus 2 (1d4) Piercing damage if the attack roll had Advantage.

### Bonus Actions

Nimble Escape. The goblin takes the Disengage or Hide action.

Original layout: [[../../srd-5.2.1/pages/290|page 290]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/goblin_warrior ` |
| ` implementation ` | ` dnd2024:creature_goblin_warrior ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 6 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Item:
    path: item
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Item | Count |
| --- | --- | --- |
| ` row_1 ` | ` Leather armor ` | ` 1 ` |
| ` row_2 ` | ` Scimitar ` | ` 1 ` |
| ` row_3 ` | ` Shield ` | ` 1 ` |
| ` row_4 ` | ` Shortbow ` | ` 1 ` |

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
| ` known ` | ` ["common", "goblin"] ` |

#### Features

##### Scimitar

<!-- splat:yaml path=splat_rule/parameters/features/scimitar -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 6
      add: 2
    average: 5
- op: conditional
  when:
    attack_had_advantage: true
  effects:
  - op: damage
    components:
    - type: slashing
      amount:
        dice:
          count: 1
          sides: 4
        add: 0
      average: 2
```

##### Shortbow

<!-- splat:yaml path=splat_rule/parameters/features/shortbow -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 80
  long: 320
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 6
      add: 2
    average: 5
- op: conditional
  when:
    attack_had_advantage: true
  effects:
  - op: damage
    components:
    - type: piercing
      amount:
        dice:
          count: 1
          sides: 4
        add: 0
      average: 2
```

##### Nimble escape

<!-- splat:values path=splat_rule/parameters/features/nimble_escape -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` action.choose ` |
| ` choices ` | ` ["disengage", "hide"] ` |

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
