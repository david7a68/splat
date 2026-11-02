---
splat_schema: 1
splat_id: 5ecfe2e4-926f-5058-acd9-438360874d52
splat_kind: creature
splat_key: dnd2024:creatures/dust_mephit
splat_name: Dust Mephit
splat_origin:
  key: dnd2024:reference/creature/dust_mephit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 307
  url: https://www.dndbeyond.com/srd
---

# Dust Mephit

## Source text

<!-- SRD page 307 -->

Small Elemental, Neutral Evil\
AC 12 Initiative +2 (12)\
HP 17 (5d6)\
Speed 30 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 5 −3 −3 Dex 14 +2 +2 Con 10 +0 +0\
Int 9 −1 −1 Wis 11 +0 +0 Cha 10 +0 +0\
Skills Perception +2, Stealth +4\
Vulnerabilities Fire\
Immunities Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Primordial (Auran, Terran)\
CR 1/2 (XP 100; PB +2)

### Traits

Death Burst. The mephit explodes when it dies. Dexterity Saving Throw: DC 10, each creature in a 5-foot Emanation originating from the mephit. Failure: 5 (2d4) Bludgeoning damage. Success: Half damage.

### Actions

Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Slashing damage.

Blinding Breath (Recharge 6). Dexterity Saving Throw: DC 10, each creature in a 15-foot Cone. Failure: The target has the Blinded condition until the end of the mephit’s next turn.

Sleep (1/Day). The mephit casts the Sleep spell, requiring no spell components and using Charisma as the spellcasting ability (spell save DC 10).

Original layout: [[../../srd-5.2.1/pages/307|page 307]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` small ` |
| ` type ` | ` elemental ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 30 ft., Fly 30 ft. ` |
| ` skills_text ` | ` Perception +2, Stealth +4 ` |
| ` vulnerabilities_text ` | ` Fire ` |
| ` immunities_text ` | ` Poison; Exhaustion, Poisoned ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` Primordial (Auran, Terran) ` |
| ` cr_text ` | ` 1/2 (XP 100; PB +2) ` |
| ` hit_points_text ` | ` 17 (5d6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 5 ` | ` -3 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 9 ` | ` -1 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 17 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/dust_mephit ` |
| ` implementation ` | ` dnd2024:creature_dust_mephit ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_vulnerabilities ` | ` ["fire"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["exhaustion", "poisoned"] ` |
| ` passive_perception ` | ` 12 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |
| ` stealth ` | ` 4 ` |

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
| ` primordial_dialects ` | ` ["auran", "terran"] ` |

#### Features

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

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
        sides: 4
      add: 2
    average: 4
```

##### Death burst

<!-- splat:yaml path=splat_rule/parameters/features/death_burst -->

```yaml
economy: passive
op: saving_throw
ability: dexterity
dc: 10
targets:
  kind: creature
  count: all
  area:
    shape: emanation
    origin: actor
    size_ft: 5
failure:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 2
        sides: 4
      add: 0
    average: 5
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
trigger: actor.died
```

##### Blinding breath

<!-- splat:yaml path=splat_rule/parameters/features/blinding_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 10
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 15
failure:
- op: condition.apply
  condition: blinded
  duration:
    until: actor.next_turn_end
success: []
recharge:
  die_sides: 6
  success_min: 6
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Sleep

<!-- splat:yaml path=splat_rule/parameters/features/sleep -->

```yaml
economy: action
op: spellcasting
profile:
  ability: charisma
  waived_components:
  - verbal
  - somatic
  - material
  save_dc: 10
choose_count: 1
choices:
- op: spell.cast
  spell:
    key: dnd2024:spells/sleep
uses:
  count: 1
  recovery: long_rest
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spellcasting defaults

<!-- splat:yaml path=splat_rule/parameters/spellcasting_defaults -->

```yaml
cast_level: lowest_possible_unless_overridden
upcast_allowed: only_explicit_cast_level_overrides
required_material_components_available: true
casting_time_at_least_1_minute:
  uses_normal_time: true
  magic_action_each_turn: true
  concentration_required: true
other_components: spell_definition_unless_waived
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
