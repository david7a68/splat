---
splat_schema: 1
splat_id: 064a002d-1b22-5c6d-aeef-eae338efbded
splat_kind: creature
splat_key: dnd2024:creatures/adult_red_dragon
splat_name: Adult Red Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_red_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 318, 319
  url: https://www.dndbeyond.com/srd
---

# Adult Red Dragon

## Source text

<!-- SRD page 318 -->

Huge Dragon (Chromatic), Chaotic Evil\
AC 19 Initiative +12 (22)\
HP 256 (19d12 + 133)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 27 +8 +8 Dex 10 +0 +6 Con 25 +7 +7\
Int 16 +3 +3 Wis 13 +1 +7 Cha 23 +6 +6\
Skills Perception +13, Stealth +6\
Immunities Fire\
Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 23\
Languages Common, Draconic\
CR 17 (XP 18,000, or 20,000 in lair; PB +6)

### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead.

### Actions

Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting to cast Scorching Ray.

Rend. Melee Attack Roll: +14, reach 10 ft. Hit: 13 (1d10 + 8) Slashing damage plus 5 (2d4) Fire damage.

<!-- SRD page 319 -->

Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 21, each creature in a 60-foot Cone. Failure: 59 (17d6) Fire damage. Success: Half damage.

Spellcasting. The dragon casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 20, +12 to hit with spell attacks):

At Will: Command (level 2 version), Detect Magic, Scorching Ray\
1/Day: Fireball

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended uses at the start of each of its turns.

Commanding Presence. The dragon uses Spellcasting to cast Command (level 2 version). The dragon can’t take this action again until the start of its next turn.

Fiery Rays. The dragon uses Spellcasting to cast Scorching Ray. The dragon can’t take this action again until the start of its next turn.

Pounce. The dragon moves up to half its Speed, and it makes one Rend attack.

Original layout: [[../../srd-5.2.1/pages/318|page 318]], [[../../srd-5.2.1/pages/319|page 319]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Chromatic) ` |
| ` movement_text ` | ` 40 ft., Climb 40 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Perception +13, Stealth +6 ` |
| ` immunities_text ` | ` Fire ` |
| ` senses_text ` | ` Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 23 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 17 (XP 18,000, or 20,000 in lair; PB +6) ` |
| ` hit_points_text ` | ` 256 (19d12 + 133) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 27 ` | ` 8 ` |
| ` Dexterity ` | ` 10 ` | ` 6 ` |
| ` Constitution ` | ` 25 ` | ` 7 ` |
| ` Intelligence ` | ` 16 ` | ` 3 ` |
| ` Wisdom ` | ` 13 ` | ` 7 ` |
| ` Charisma ` | ` 23 ` | ` 6 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 19 ` |  |
| ` max_hp ` | ` 256 ` |  |
| ` initiative ` | ` 12 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` climb_speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 80 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/adult_red_dragon ` |
| ` implementation ` | ` dnd2024:creature_adult_red_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |
| ` passive_perception ` | ` 23 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 13 ` |
| ` stealth ` | ` 6 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:table
path: splat_rule/parameters/senses
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Sense:
    path: sense
    type: symbol
  Range ft:
    path: range_ft
    type: integer
-->

| Key | Sense | Range ft |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 60 ` |
| ` row_2 ` | ` Darkvision ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "draconic"] ` |

#### Features

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
trigger: saving_throw.failed
optional: true
op: saving_throw.set_result
result: success
uses_in_lair: 4
uses:
  count: 3
  recovery: long_rest
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 14
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
        sides: 10
      add: 8
    average: 13
  - type: fire
    amount:
      dice:
        count: 2
        sides: 4
      add: 0
    average: 5
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` spellcasting ` |

###### Profile

<!-- splat:values path=splat_rule/parameters/features/spellcasting/profile -->

| Field | Value |
| --- | --- |
| ` ability ` | ` charisma ` |
| ` waived_components ` | ` ["material"] ` |
| ` save_dc ` | ` 20 ` |
| ` attack_bonus ` | ` 12 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- command
- detect_magic
- scorching_ray
- fireball
columns:
  Spell:
    path: spell
    type: reference
  Uses unlimited:
    path: uses/unlimited
    type: boolean
    optional: true
  Cast level:
    path: cast_level
    type: integer
    optional: true
  Uses count:
    path: uses/count
    type: integer
    optional: true
  Uses recovery:
    path: uses/recovery
    type: symbol
    optional: true
-->

| Spell | Uses unlimited | Cast level | Uses count | Uses recovery |
| --- | --- | --- | --- | --- |
| [Command](<../spells/command.md> "dnd2024:spells/command") | true | ` 2 ` |  |  |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | true |  |  |  |
| [Scorching Ray](<../spells/scorching_ray.md> "dnd2024:spells/scorching_ray") | true |  |  |  |
| [Fireball](<../spells/fireball.md> "dnd2024:spells/fireball") |  |  | ` 1 ` | ` Long rest ` |

##### Fire breath

<!-- splat:yaml path=splat_rule/parameters/features/fire_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 21
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 60
failure:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 17
        sides: 6
      add: 0
    average: 59
success:
- op: damage
  from: failure_damage
  multiplier: 0.5
  round: down
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

##### Commanding presence

<!-- splat:yaml path=splat_rule/parameters/features/commanding_presence -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: spell.cast
  spell:
    key: dnd2024:spells/command
  profile: spellcasting
  cast_level: 2
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Fiery rays

<!-- splat:yaml path=splat_rule/parameters/features/fiery_rays -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: spell.cast
  spell:
    key: dnd2024:spells/scorching_ray
  profile: spellcasting
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Pounce

<!-- splat:yaml path=splat_rule/parameters/features/pounce -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: movement
  maximum:
    multiply:
    - 0.5
    - actor_stat: speed
- op: feature.use
  feature: rend
legendary_cost: 1
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 3
  feature: rend
replace_attack:
  maximum: 1
  choices:
  - op: spell.cast
    spell:
      key: dnd2024:spells/scorching_ray
    profile: spellcasting
```

#### Legendary actions

<!-- splat:values path=splat_rule/parameters/legendary_actions -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 3 ` |
| ` uses_in_lair ` | ` 4 ` |
| ` trigger ` | ` another_creature.turn_end_immediately_after ` |
| ` cost_per_action ` | ` 1 ` |
| ` recover_all ` | ` actor.turn_start ` |
| ` maximum_per_trigger ` | ` 1 ` |
| ` requires_actions_available ` | ` true ` |
| ` excluded_conditions ` | ` ["incapacitated"] ` |

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
