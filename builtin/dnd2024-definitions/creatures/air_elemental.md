---
splat_schema: 1
splat_id: 27dce8d8-ae3a-5153-999b-db85e2a98284
splat_kind: creature
splat_key: dnd2024:creatures/air_elemental
splat_name: Air Elemental
splat_origin:
  key: dnd2024:reference/creature/air_elemental
splat_sources:
- title: SRD 5.2.1
  locator: pp. 258, 259
  url: https://www.dndbeyond.com/srd
---

# Air Elemental

## Source text

<!-- SRD page 258 -->

Large Elemental, Neutral\
AC 15 Initiative +5 (15)\
HP 90 (12d10 + 24)\
Speed 10 ft., Fly 90 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 20 +5 +5 Con 14 +2 +2\
Int 6 −2 −2 Wis 10 +0 +0 Cha 6 −2 −2\
Resistances Bludgeoning, Lightning, Piercing, Slashing\
Immunities Poison, Thunder; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Auran)\
CR 5 (XP 1,800; PB +3)

### Traits

Air Form. The elemental can enter a creature’s space and stop there. It can move through a space as narrow as 1 inch without expending extra movement to do so.

### Actions

Multiattack. The elemental makes two Thunderous Slam attacks.

Thunderous Slam. Melee Attack Roll: +8, reach 10 ft. Hit: 14 (2d8 + 5) Thunder damage.

<!-- SRD page 259 -->

Whirlwind (Recharge 4–6). Strength Saving Throw: DC 13, one Medium or smaller creature in the elemental’s space. Failure: 24 (4d10 + 2) Thunder damage, and the target is pushed up to 20 feet straight away from the elemental and has the Prone condition. Success: Half damage only.

Original layout: [[../../srd-5.2.1/pages/258|page 258]], [[../../srd-5.2.1/pages/259|page 259]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 10 ft., Fly 90 ft. (hover) ` |
| ` resistances_text ` | ` Bludgeoning, Lightning, Piercing, Slashing ` |
| ` immunities_text ` | ` Poison, Thunder; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 10 ` |
| ` languages_text ` | ` Primordial (Auran) ` |
| ` cr_text ` | ` 5 (XP 1,800; PB +3) ` |
| ` hit_points_text ` | ` 90 (12d10 + 24) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 20 ` | ` 5 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 6 ` | ` -2 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 15 ` |  |
| ` max_hp ` | ` 90 ` |  |
| ` initiative ` | ` 5 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` fly_speed ` | ` 90 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/air_elemental ` |
| ` implementation ` | ` dnd2024:creature_air_elemental ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["bludgeoning", "lightning", "piercing", "slashing"] ` |
| ` damage_immunities ` | ` ["poison", "thunder"] ` |
| ` condition_immunities ` | ` ["exhaustion", "grappled", "paralyzed", "petrified", "poisoned", "prone", "restrained", "unconscious"] ` |
| ` passive_perception ` | ` 10 ` |

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
| ` known ` | ` ["primordial"] ` |
| ` primordial_dialects ` | ` ["auran"] ` |

#### Features

##### Air form

<!-- splat:values path=splat_rule/parameters/features/air_form -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` movement.form ` |
| ` minimum_opening_inches ` | ` 1 ` |
| ` extra_movement_cost ` | ` 0 ` |
| ` can_enter_and_end_in_creature_space ` | ` true ` |

##### Thunderous slam

<!-- splat:yaml path=splat_rule/parameters/features/thunderous_slam -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 8
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: thunder
    amount:
      dice:
        count: 2
        sides: 8
      add: 5
    average: 14
```

##### Whirlwind

<!-- splat:yaml path=splat_rule/parameters/features/whirlwind -->

```yaml
economy: action
op: saving_throw
ability: strength
dc: 13
targets:
  kind: creature
  count: 1
  maximum_size: medium
  shares_actor_space: true
failure:
- op: damage
  components:
  - type: thunder
    amount:
      dice:
        count: 4
        sides: 10
      add: 2
    average: 24
- op: push
  maximum_ft: 20
  direction: straight_away_from_actor
- op: condition.apply
  condition: prone
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
recharge:
  die_sides: 6
  success_min: 4
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: thunderous_slam
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
