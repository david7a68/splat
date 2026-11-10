---
splat_schema: 2
splat_id: 006a2999-e089-576e-94db-c2a47aca2c61
splat_kind: creature
splat_key: dnd2024:creatures/mummy
splat_name: Mummy
splat_origin: dnd2024:reference/creature/mummy
splat_sources:
- SRD 5.2.1, pp. 309, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 219, Habitat
---

# Mummy


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Undead |
| Alignment | Lawful Evil |
| Armor Class | 11 |
| Hit Points | 58 (9d8 + 18) |
| Initiative | -1 (9) |
| Speed | 20 ft. |
| Challenge | 3 (XP 700; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 11 |
| Languages | Common plus two other languages |
| Immunities | Necrotic, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned |
| Vulnerabilities | Fire |
| Habitat | Desert, Swamp |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 8 | -1 | -1 |
| Con | 15 | +2 | +2 |
| Int | 6 | -2 | -2 |
| Wis | 12 | +1 | +3 |
| Cha | 12 | +1 | +1 |


## Source text

<!-- SRD page 309 -->

Medium or Small Undead, Lawful Evil\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 11 Initiative −1 (9)\
> HP 58 (9d8 + 18)\
> Speed 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 8 −1 −1 Con 15 +2 +2\
> Int 6 −2 −2 Wis 12 +1 +3 Cha 12 +1 +1\
> Vulnerabilities Fire\
> Immunities Necrotic, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned\
> Senses Darkvision 60 ft.; Passive Perception 11\
> Languages Common plus two other languages\
> CR 3 (XP 700; PB +2)


### Actions

Multiattack. The mummy makes two Rotting Fist attacks and uses Dreadful Glare.

Rotting Fist. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3) Bludgeoning damage plus 10 (3d6) Necrotic damage. If the target is a creature, it is cursed. While cursed, the target can’t regain Hit Points, its Hit Point maximum doesn’t return to normal when finishing a Long Rest, and its Hit Point maximum decreases by 10 (3d6) every 24 hours that elapse. A creature dies and turns to dust if reduced to 0 Hit Points by this attack.

Dreadful Glare. Wisdom Saving Throw: DC 11, one creature the mummy can see within 60 feet. Failure: The target has the Frightened condition until the end of the mummy’s next turn. Success: The target is immune to this mummy’s Dreadful Glare for 24 hours.

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
| ` mechanic ` | ` dnd2024:creatures/mummy ` |
| ` implementation ` | ` dnd2024:creature_mummy ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_vulnerabilities ` | ` ["fire"] ` |
| ` damage_immunities ` | ` ["necrotic", "poison"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened", "paralyzed", "poisoned"] ` |
| ` passive_perception ` | ` 11 ` |

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
| ` known ` | ` ["common"] ` |
| ` additional_choice_count ` | ` 2 ` |

#### Features

##### Rotting fist

<!-- splat:yaml path=splat_rule/parameters/features/rotting_fist -->

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
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 10
      add: 3
    average: 8
  - type: necrotic
    amount:
      dice:
        count: 3
        sides: 6
      add: 0
    average: 10
- op: curse.apply
  when:
    target_kind: creature
  rules:
    healing_allowed: false
    long_rest_restores_hp_maximum: false
  periodic:
    interval:
      amount: 24
      unit: hour
    effect:
      op: hp_maximum.reduce
      amount:
        dice:
          count: 3
          sides: 6
        add: 0
      average: 10
on_reduce_target_to_zero_hp:
  when:
    target_kind: creature
  op: death
  corpse: dust
```

##### Dreadful glare

<!-- splat:yaml path=splat_rule/parameters/features/dreadful_glare -->

```yaml
economy: action
op: saving_throw
ability: wisdom
dc: 11
targets:
  kind: creature
  count: 1
  range_ft: 60
  visible_to_actor: true
failure:
- op: condition.apply
  condition: frightened
  duration:
    until: actor.next_turn_end
success:
- op: feature.immunity
  source: actor
  feature: dreadful_glare
  duration:
    amount: 24
    unit: hour
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
| ` row_1 ` | ` Rotting fist ` | ` 2 ` |
| ` row_2 ` | ` Dreadful glare ` | ` 1 ` |

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
