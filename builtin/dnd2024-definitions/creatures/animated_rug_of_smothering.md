---
splat_schema: 2
splat_id: e95b1f68-6031-5ca5-8a44-c1e6784bb620
splat_kind: creature
splat_key: dnd2024:creatures/animated_rug_of_smothering
splat_name: Animated Rug of Smothering
splat_origin: dnd2024:reference/creature/animated_rug_of_smothering
splat_sources:
- SRD 5.2.1, pp. 259, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 17, Habitat
---

# Animated Rug of Smothering


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Construct |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 27 (5d10) |
| Initiative | +4 (14) |
| Speed | 10 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Senses | Blindsight 60 ft.; Passive Perception 6 |
| Languages | None |
| Immunities | Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned |
| Habitat | Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 14 | +2 | +2 |
| Con | 10 | +0 | +0 |
| Int | 1 | -5 | -5 |
| Wis | 3 | -4 | -4 |
| Cha | 1 | -5 | -5 |


## Source text

<!-- SRD page 259 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Construct, Unaligned\
> AC 12 Initiative +4 (14)\
> HP 27 (5d10)\
> Speed 10 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 14 +2 +2 Con 10 +0 +0\
> Int 1 −5 −5 Wis 3 −4 −4 Cha 1 −5 −5\
> Immunities Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
> Senses Blindsight 60 ft.; Passive Perception 6\
> Languages None\
> CR 2 (XP 450; PB +2)


### Actions

Smother. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Bludgeoning damage. If the target is a Medium or smaller creature, the rug can give it the Grappled condition (escape DC 13) instead of dealing damage. Until the grapple ends, the target has the Blinded and Restrained conditions, is suffocating, and takes 10 (2d6 + 3) Bludgeoning damage at the start of each of its turns. The rug can smother only one creature at a time.

While grappling the target, the rug can’t take this action, the rug halves the damage it takes (round down), and the target takes the same amount of damage.

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
| ` mechanic ` | ` dnd2024:creatures/animated_rug_of_smothering ` |
| ` implementation ` | ` dnd2024:creature_animated_rug_of_smothering ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison", "psychic"] ` |
| ` condition_immunities ` | ` ["charmed", "deafened", "exhaustion", "frightened", "paralyzed", "petrified", "poisoned"] ` |
| ` passive_perception ` | ` 6 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Smother

<!-- splat:yaml path=splat_rule/parameters/features/smother -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 5
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: choice
  chooser: actor
  choices:
  - effects:
    - op: damage
      components:
      - type: bludgeoning
        amount:
          dice:
            count: 2
            sides: 6
          add: 3
        average: 10
  - requires:
      target_kind: creature
      target_maximum_size: medium
    effects:
    - op: grapple
      escape_dc: 13
      maximum_targets: 1
      while_grappled:
      - op: condition.apply
        condition: restrained
      - op: condition.apply
        condition: blinded
      - op: suffocation.start
      - trigger: target.turn_start
        effects:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 2
                sides: 6
              add: 3
            average: 10
unavailable_while_grappling: true
while_grappling:
  incoming_damage_multiplier: 0.5
  round: down
  grappled_target_takes_same_damage: true
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
