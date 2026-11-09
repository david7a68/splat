---
splat_schema: 2
splat_id: 753c9e67-77c6-5e3d-882a-c7e898292576
splat_kind: creature
splat_key: dnd2024:creatures/shadow
splat_name: Shadow
splat_origin: dnd2024:reference/creature/shadow
splat_sources:
- SRD 5.2.1, pp. 322, 323, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 272, Habitat
---

# Shadow


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Undead |
| Alignment | Chaotic Evil |
| Armor Class | 12 |
| Hit Points | 27 (5d8 + 5) |
| Initiative | +2 (12) |
| Speed | 40 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Skills | Stealth +6 |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | None |
| Resistances | Acid, Cold, Fire, Lightning, Thunder |
| Immunities | Necrotic, Poison; Exhaustion, Frightened, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious |
| Vulnerabilities | Radiant |
| Habitat | Underdark, Urban, Planar (Shadowfell) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 6 | -2 | -2 |
| Dex | 14 | +2 | +2 |
| Con | 13 | +1 | +1 |
| Int | 6 | -2 | -2 |
| Wis | 10 | +0 | +0 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 322 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Undead, Chaotic Evil\
> AC 12  Initiative +2 (12)\
> HP 27 (5d8 + 5)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 6 −2 −2 Dex 14 +2 +2 Con 13 +1 +1\
> Int 6 −2 −2 Wis 10 +0 +0 Cha 8 −1 −1\
> Skills Stealth +6\
> Vulnerabilities Radiant\
> Resistances Acid, Cold, Fire, Lightning, Thunder\
> Immunities Necrotic, Poison; Exhaustion, Frightened, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Traits

Amorphous. The shadow can move through a space as narrow as 1 inch without expending extra movement to do so.

Sunlight Weakness. While in sunlight, the shadow has Disadvantage on D20 Tests.

<!-- SRD page 323 -->

### Actions

Draining Swipe. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Necrotic damage, and the target’s Strength score decreases by 1d4. The target dies if this reduces that score to 0. If a Humanoid is slain by this attack, a Shadow rises from the corpse 1d4 hours later.

### Bonus Actions

Shadow Stealth. While in Dim Light or Darkness, the shadow takes the Hide action.

Original layout: [[../../srd-5.2.1/pages/322|page 322]], [[../../srd-5.2.1/pages/323|page 323]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/shadow ` |
| ` implementation ` | ` dnd2024:creature_shadow ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_vulnerabilities ` | ` ["radiant"] ` |
| ` damage_resistances ` | ` ["acid", "cold", "fire", "lightning", "thunder"] ` |
| ` damage_immunities ` | ` ["necrotic", "poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "frightened", "grappled", "paralyzed", "petrified", "poisoned", "prone", "restrained", "unconscious"] ` |
| ` passive_perception ` | ` 10 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 6 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Amorphous

<!-- splat:values path=splat_rule/parameters/features/amorphous -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` movement.form ` |
| ` minimum_opening_inches ` | ` 1 ` |
| ` extra_movement_cost ` | ` 0 ` |

##### Sunlight weakness

<!-- splat:yaml path=splat_rule/parameters/features/sunlight_weakness -->

```yaml
economy: passive
op: roll.disadvantage
when:
  in_sunlight: true
rolls:
- all_d20_tests
```

##### Draining swipe

<!-- splat:yaml path=splat_rule/parameters/features/draining_swipe -->

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
  - type: necrotic
    amount:
      dice:
        count: 1
        sides: 6
      add: 2
    average: 5
- op: ability_score.reduce
  ability: strength
  amount:
    dice:
      count: 1
      sides: 4
    add: 0
  death_at_zero: true
on_humanoid_slain:
  op: undead.create
  definition:
    key: dnd2024:creatures/shadow
  delay:
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    unit: hour
```

##### Shadow stealth

<!-- splat:yaml path=splat_rule/parameters/features/shadow_stealth -->

```yaml
economy: bonus_action
op: action.use
action: hide
when:
  lighting:
  - dim_light
  - darkness
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
