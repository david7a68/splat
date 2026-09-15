---
splat_schema: 1
splat_id: 86bad936-8264-59f8-8911-7f25a067cb15
splat_kind: creature
splat_key: dnd2024:creatures/ancient_red_dragon
splat_name: Ancient Red Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_red_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 319
  url: https://www.dndbeyond.com/srd
---

# Ancient Red Dragon

## Source text

<!-- SRD page 319 -->

Gargantuan Dragon (Chromatic), Chaotic Evil\
AC 22 Initiative +14 (24)\
HP 507 (26d20 + 234)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 30 +10 +10 Dex 10 +0 +7 Con 29 +9 +9\
Int 18 +4 +4 Wis 15 +2 +9 Cha 27 +8 +8\
Skills Perception +16, Stealth +7\
Immunities Fire\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 26\
Languages Common, Draconic\
CR 24 (XP 62,000, or 75,000 in lair; PB +7)\

### Traits

Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to cast\
Scorching Ray (level 3 version).\
Rend. Melee Attack Roll: +17, reach 15 ft. Hit: 19 (2d8\
+ 10) Slashing damage plus 10 (3d6) Fire damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 24, each creature in a 90-foot Cone. Failure: 91\
(26d6) Fire damage. Success: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 23,\
+15 to hit with spell attacks):\
At Will: Command (level 2 version), Detect Magic,\
Scorching Ray (level 3 version)\
1/Day Each: Fireball (level 6 version), Scrying\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Commanding Presence. The dragon uses Spellcasting\
to cast Command (level 2 version). The dragon can’t\
take this action again until the start of its next turn.\
Fiery Rays. The dragon uses Spellcasting to cast Scorch-\
ing Ray (level 3 version). The dragon can’t take this ac-\
tion again until the start of its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/319|page 319]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` gargantuan ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Chromatic) ` |
| ` movement_text ` | ` 40 ft., Climb 40 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Perception +16, Stealth +7 ` |
| ` immunities_text ` | ` Fire ` |
| ` senses_text ` | ` Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 26 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 24 (XP 62,000, or 75,000 in lair; PB +7) ` |
| ` hit_points_text ` | ` 507 (26d20 + 234) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 30 ` | ` 10 ` |
| ` Dexterity ` | ` 10 ` | ` 7 ` |
| ` Constitution ` | ` 29 ` | ` 9 ` |
| ` Intelligence ` | ` 18 ` | ` 4 ` |
| ` Wisdom ` | ` 15 ` | ` 9 ` |
| ` Charisma ` | ` 27 ` | ` 8 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 22 ` |  |
| ` max_hp ` | ` 507 ` |  |
| ` initiative ` | ` 14 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/ancient_red_dragon ` |
| ` implementation ` | ` dnd2024:creature_ancient_red_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |
| ` passive_perception ` | ` 26 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 16 ` |
| ` stealth ` | ` 7 ` |

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
uses_in_lair: 5
uses:
  count: 4
  recovery: long_rest
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 17
target:
  kind: creature_or_object
  count: 1
reach_ft: 15
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 8
      add: 10
    average: 19
  - type: fire
    amount:
      dice:
        count: 3
        sides: 6
      add: 0
    average: 10
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
| ` save_dc ` | ` 23 ` |
| ` attack_bonus ` | ` 15 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- command
- detect_magic
- scorching_ray
- fireball
- scrying
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
| [Scorching Ray](<../spells/scorching_ray.md> "dnd2024:spells/scorching_ray") | true | ` 3 ` |  |  |
| [Fireball](<../spells/fireball.md> "dnd2024:spells/fireball") |  | ` 6 ` | ` 1 ` | ` Long rest ` |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") |  |  | ` 1 ` | ` Long rest ` |

##### Fire breath

<!-- splat:yaml path=splat_rule/parameters/features/fire_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 24
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 90
failure:
- op: damage
  components:
  - type: fire
    amount:
      dice:
        count: 26
        sides: 6
      add: 0
    average: 91
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
  cast_level: 3
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
    cast_level: 3
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
