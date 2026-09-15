---
splat_schema: 1
splat_id: 3c3ee465-62cb-54df-b868-1a436b108091
splat_kind: creature
splat_key: dnd2024:creatures/adult_blue_dragon
splat_name: Adult Blue Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_blue_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 266, 267
  url: https://www.dndbeyond.com/srd
---

# Adult Blue Dragon

## Source text

<!-- SRD page 266 -->

Huge Dragon (Chromatic), Lawful Evil\
AC 19 Initiative +10 (20)\
HP 212 (17d12 + 102)\
Speed 40 ft., Burrow 30 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 25 +7 +7 Dex 10 +0 +5 Con 23 +6 +6\
Int 16 +3 +3 Wis 15 +2 +7 Cha 20 +5 +5\
Skills Perception +12, Stealth +5\
Immunities Lightning\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 22\
Languages Common, Draconic\
CR 16 (XP 15,000, or 18,000 in lair; PB +5)\

### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to\
cast Shatter.\
Rend. Melee Attack Roll: +12, reach 10 ft. Hit: 16 (2d8\
+ 7) Slashing damage plus 5 (1d10) Lightning damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 19, each creature in a 90-foot-long, 5-foot-\

<!-- SRD page 267 -->

wide Line. Failure: 60 (11d10) Lightning damage. Suc-\
cess: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 18):\
At Will: Detect Magic, Invisibility, Mage Hand, Shatter\
1/Day Each: Scrying, Sending\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Cloaked Flight. The dragon uses Spellcasting to cast In-\
visibility on itself, and it can fly up to half its Fly Speed.\
The dragon can’t take this action again until the start of\
its next turn.\
Sonic Boom. The dragon uses Spellcasting to cast Shat-\
ter. The dragon can’t take this action again until the\
start of its next turn.\
Tail Swipe. The dragon makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/266|page 266]], [[../../srd-5.2.1/pages/267|page 267]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Lawful Evil ` |
| ` tags_text ` | ` (Chromatic) ` |
| ` movement_text ` | ` 40 ft., Burrow 30 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Perception +12, Stealth +5 ` |
| ` immunities_text ` | ` Lightning ` |
| ` senses_text ` | ` Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 22 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 16 (XP 15,000, or 18,000 in lair; PB +5) ` |
| ` hit_points_text ` | ` 212 (17d12 + 102) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 25 ` | ` 7 ` |
| ` Dexterity ` | ` 10 ` | ` 5 ` |
| ` Constitution ` | ` 23 ` | ` 6 ` |
| ` Intelligence ` | ` 16 ` | ` 3 ` |
| ` Wisdom ` | ` 15 ` | ` 7 ` |
| ` Charisma ` | ` 20 ` | ` 5 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 19 ` |  |
| ` max_hp ` | ` 212 ` |  |
| ` initiative ` | ` 10 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` burrow_speed ` | ` 30 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/adult_blue_dragon ` |
| ` implementation ` | ` dnd2024:creature_adult_blue_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["lightning"] ` |
| ` passive_perception ` | ` 22 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 12 ` |
| ` stealth ` | ` 5 ` |

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
attack_bonus: 12
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
        count: 2
        sides: 8
      add: 7
    average: 16
  - type: lightning
    amount:
      dice:
        count: 1
        sides: 10
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
| ` save_dc ` | ` 18 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- detect_magic
- invisibility
- mage_hand
- shatter
- scrying
- sending
columns:
  Spell:
    path: spell
    type: reference
  Uses unlimited:
    path: uses/unlimited
    type: boolean
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

| Spell | Uses unlimited | Uses count | Uses recovery |
| --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | true |  |  |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") | true |  |  |
| [Mage Hand](<../spells/mage_hand.md> "dnd2024:spells/mage_hand") | true |  |  |
| [Shatter](<../spells/shatter.md> "dnd2024:spells/shatter") | true |  |  |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") |  | ` 1 ` | ` Long rest ` |
| [Sending](<../spells/sending.md> "dnd2024:spells/sending") |  | ` 1 ` | ` Long rest ` |

##### Lightning breath

<!-- splat:yaml path=splat_rule/parameters/features/lightning_breath -->

```yaml
economy: action
op: saving_throw
ability: dexterity
dc: 19
targets:
  kind: creature
  count: all
  area:
    shape: line
    origin: actor
    size_ft: 90
    width_ft: 5
failure:
- op: damage
  components:
  - type: lightning
    amount:
      dice:
        count: 11
        sides: 10
      add: 0
    average: 60
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

##### Cloaked flight

<!-- splat:yaml path=splat_rule/parameters/features/cloaked_flight -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: spell.cast
  spell:
    key: dnd2024:spells/invisibility
  profile: spellcasting
  target: self
- op: movement
  maximum:
    multiply:
    - 0.5
    - actor_stat: fly_speed
  optional: true
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Sonic boom

<!-- splat:yaml path=splat_rule/parameters/features/sonic_boom -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: spell.cast
  spell:
    key: dnd2024:spells/shatter
  profile: spellcasting
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Tail swipe

<!-- splat:values path=splat_rule/parameters/features/tail_swipe -->

| Field | Value |
| --- | --- |
| ` economy ` | ` legendary_action ` |
| ` op ` | ` feature.use ` |
| ` feature ` | ` rend ` |
| ` legendary_cost ` | ` 1 ` |

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
      key: dnd2024:spells/shatter
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
