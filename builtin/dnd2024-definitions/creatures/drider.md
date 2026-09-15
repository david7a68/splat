---
splat_schema: 1
splat_id: db69f9e6-632e-5775-b339-3f413b618d47
splat_kind: creature
splat_key: dnd2024:creatures/drider
splat_name: Drider
splat_origin:
  key: dnd2024:reference/creature/drider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 281
  url: https://www.dndbeyond.com/srd
---

# Drider

## Source text

<!-- SRD page 281 -->

Large Monstrosity, Chaotic Evil\
AC 19 Initiative +4 (14)\
HP 123 (13d10 + 52)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 19 +4 +4 Con 18 +4 +4\
Int 13 +1 +1 Wis 16 +3 +3 Cha 12 +1 +1\
Skills Perception +6, Stealth +10\
Senses Darkvision 120 ft.; Passive Perception 16\
Languages Elvish, Undercommon\
CR 6 (XP 2,300; PB +3)\

### Traits

Spider Climb. The drider can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Sunlight Sensitivity. While in sunlight, the drider has\
Disadvantage on ability checks and attack rolls.\
Web Walker. The drider ignores movement restrictions\
caused by webs, and the drider knows the location of\
any other creature in contact with the same web.\

### Actions

Multiattack. The drider makes three attacks, using\
Foreleg or Poison Burst in any combination.\
Foreleg. Melee Attack Roll: +7, reach 10 ft. Hit: 13 (2d8\
+ 4) Piercing damage.\
Poison Burst. Ranged Attack Roll: +6, range 120 ft. Hit:\
13 (3d6 + 3) Poison damage.\

### Bonus Actions

Magic of the Spider Queen (Recharge 5–6). The drider\
casts Darkness, Faerie Fire, or Web, requiring no Mate-\
rial components and using Wisdom as the spellcasting\
ability (spell save DC 14).\

Original layout: [[../../srd-5.2.1/pages/281|page 281]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` movement_text ` | ` 30 ft., Climb 30 ft. ` |
| ` skills_text ` | ` Perception +6, Stealth +10 ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 16 ` |
| ` languages_text ` | ` Elvish, Undercommon ` |
| ` cr_text ` | ` 6 (XP 2,300; PB +3) ` |
| ` hit_points_text ` | ` 123 (13d10 + 52) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 19 ` | ` 4 ` |
| ` Constitution ` | ` 18 ` | ` 4 ` |
| ` Intelligence ` | ` 13 ` | ` 1 ` |
| ` Wisdom ` | ` 16 ` | ` 3 ` |
| ` Charisma ` | ` 12 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 19 ` |  |
| ` max_hp ` | ` 123 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` climb_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/drider ` |
| ` implementation ` | ` dnd2024:creature_drider ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |
| ` stealth ` | ` 10 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["elvish", "undercommon"] ` |

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` movement.climb ` |
| ` difficult_surfaces ` | ` true ` |
| ` ceilings ` | ` true ` |
| ` ability_check_required ` | ` false ` |

##### Sunlight sensitivity

<!-- splat:yaml path=splat_rule/parameters/features/sunlight_sensitivity -->

```yaml
economy: passive
op: roll.disadvantage
when:
  in_sunlight: true
rolls:
- ability_check
- attack
```

##### Web walker

<!-- splat:values path=splat_rule/parameters/features/web_walker -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` ignore_movement_restrictions_from ` | ` ["web"] ` |
| ` locate_creatures_in_same_web ` | ` true ` |

##### Foreleg

<!-- splat:yaml path=splat_rule/parameters/features/foreleg -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 10
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 8
      add: 4
    average: 13
```

##### Poison burst

<!-- splat:yaml path=splat_rule/parameters/features/poison_burst -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 6
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 120
  long: 120
hit:
- op: damage
  components:
  - type: poison
    amount:
      dice:
        count: 3
        sides: 6
      add: 3
    average: 13
```

##### Magic of the spider queen

<!-- splat:yaml path=splat_rule/parameters/features/magic_of_the_spider_queen -->

```yaml
economy: bonus_action
op: spellcasting
profile:
  ability: wisdom
  waived_components:
  - material
  save_dc: 14
choose_count: 1
choices:
- op: spell.cast
  spell:
    key: dnd2024:spells/darkness
- op: spell.cast
  spell:
    key: dnd2024:spells/faerie_fire
- op: spell.cast
  spell:
    key: dnd2024:spells/web
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

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 3
  choices:
  - foreleg
  - poison_burst
  combination: any
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
