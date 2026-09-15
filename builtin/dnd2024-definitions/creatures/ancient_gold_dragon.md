---
splat_schema: 1
splat_id: f2b6de84-c5f4-505a-9d08-668227967563
splat_kind: creature
splat_key: dnd2024:creatures/ancient_gold_dragon
splat_name: Ancient Gold Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_gold_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 292
  url: https://www.dndbeyond.com/srd
---

# Ancient Gold Dragon

## Source text

<!-- SRD page 292 -->

Gargantuan Dragon (Metallic), Lawful Good\
AC 22 Initiative +16 (26)\
HP 546 (28d20 + 252)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 30 +10 +10 Dex 14 +2 +9 Con 29 +9 +9\
Int 18 +4 +4 Wis 17 +3 +10 Cha 28 +9 +9\
Skills Insight +10, Perception +17, Persuasion +16, Stealth\
+9\
Immunities Fire\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 27\
Languages Common, Draconic\
CR 24 (XP 62,000, or 75,000 in lair; PB +7)\

### Traits

Amphibious. The dragon can breathe air and water.\
Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of (A) Spellcasting\
to cast Guiding Bolt (level 4 version) or (B) Weaken-\
ing Breath.\
Rend. Melee Attack Roll: +17 to hit, reach 15 ft. Hit: 19\
(2d8 + 10) Slashing damage plus 9 (2d8) Fire damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 24, each creature in a 90-foot Cone. Failure: 71\
(13d10) Fire damage. Success: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 24,\
+16 to hit with spell attacks):\
At Will: Detect Magic, Guiding Bolt (level 4 version),\
Shapechange (Beast or Humanoid form only, no\
Temporary Hit Points gained from the spell, and no\
Concentration or Temporary Hit Points required to\
maintain the spell)\
1/Day Each: Flame Strike (level 6 version), Word of Re-\
call, Zone of Truth\
Weakening Breath. Strength Saving Throw: DC 24,\
each creature that isn’t currently affected by this breath\
in a 90-foot Cone. Failure: The target has Disadvantage\
on Strength-based D20 Tests and subtracts 5 (1d10)\
from its damage rolls. It repeats the save at the end of\
each of its turns, ending the effect on itself on a suc-\
cess. After 1 minute, it succeeds automatically.\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Banish. Charisma Saving Throw: DC 24, one creature\
the dragon can see within 120 feet. Failure: 24 (7d6)\
Force damage, and the target has the Incapacitated\
condition and is transported to a harmless demiplane\
until the start of the dragon’s next turn, at which point\
it reappears in an unoccupied space of the dragon’s\
choice within 120 feet of the dragon. Failure or Success:\
The dragon can’t take this action again until the start of\
its next turn.\
Guiding Light. The dragon uses Spellcasting to cast\
Guiding Bolt (level 4 version).\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/292|page 292]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` gargantuan ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Lawful Good ` |
| ` tags_text ` | ` (Metallic) ` |
| ` movement_text ` | ` 40 ft., Fly 80 ft., Swim 40 ft. ` |
| ` skills_text ` | ` Insight +10, Perception +17, Persuasion +16, Stealth +9 ` |
| ` immunities_text ` | ` Fire ` |
| ` senses_text ` | ` Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 24 (XP 62,000, or 75,000 in lair; PB +7) ` |
| ` hit_points_text ` | ` 546 (28d20 + 252) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 30 ` | ` 10 ` |
| ` Dexterity ` | ` 14 ` | ` 9 ` |
| ` Constitution ` | ` 29 ` | ` 9 ` |
| ` Intelligence ` | ` 18 ` | ` 4 ` |
| ` Wisdom ` | ` 17 ` | ` 10 ` |
| ` Charisma ` | ` 28 ` | ` 9 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 22 ` |  |
| ` max_hp ` | ` 546 ` |  |
| ` initiative ` | ` 16 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` fly_speed ` | ` 80 ` | ` ft ` |
| ` swim_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ancient_gold_dragon ` |
| ` implementation ` | ` dnd2024:creature_ancient_gold_dragon ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["air", "water"] ` |

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: trait
resource:
  kind: daily
  uses: 4
  recovery: long_rest
  lair_uses: 5
operation: replace_save_result
trigger: self.save_failed
optional: true
result: success
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: rend
  count: 3
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: choice
    options:
    - operation: use_feature
      feature: weakening_breath
    - operation: cast_spell
      spell:
        key: dnd2024:spells/guiding_bolt
      cast_level: 4
      spellcasting_profile: spellcasting
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 17
  reach_ft: 15
hit:
  damage:
  - average: 19
    roll:
      count: 2
      sides: 8
      modifier: 10
    type: slashing
  - average: 9
    roll:
      count: 2
      sides: 8
      modifier: 0
    type: fire
```

##### Fire breath

<!-- splat:yaml path=splat_rule/parameters/features/fire_breath -->

```yaml
economy: action
resource:
  kind: recharge
  die: 6
  success_faces:
  - 5
  - 6
  check: self.turn_start
  rest_recovery:
  - short
  - long
  capacity: 1
  initial_uses: 1
operation: saving_throw
ability: dexterity
dc: 24
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 90
    origin: self
failure:
  damage:
  - average: 71
    roll:
      count: 13
      sides: 10
      modifier: 0
    type: fire
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting -->

```yaml
economy: action
operation: spellcasting
ability: charisma
components_waived:
- material
save_dc: 24
supplied_spell_attack_total: 16
options:
- spell:
    key: dnd2024:spells/detect_magic
  usage:
    kind: at_will
- spell:
    key: dnd2024:spells/guiding_bolt
  usage:
    kind: at_will
  cast_level: 4
- spell:
    key: dnd2024:spells/shapechange
  usage:
    kind: at_will
  overrides:
    allowed_types:
    - beast
    - humanoid
    gain_temporary_hp: false
    maintain_requires_concentration: false
    maintain_requires_temporary_hp: false
- spell:
    key: dnd2024:spells/flame_strike
  usage:
    kind: daily
    uses: 1
    pool: per_spell
    recovery: long_rest
  cast_level: 6
- spell:
    key: dnd2024:spells/word_of_recall
  usage:
    kind: daily
    uses: 1
    pool: per_spell
    recovery: long_rest
- spell:
    key: dnd2024:spells/zone_of_truth
  usage:
    kind: daily
    uses: 1
    pool: per_spell
    recovery: long_rest
material_components_available: true
```

##### Weakening breath

<!-- splat:yaml path=splat_rule/parameters/features/weakening_breath -->

```yaml
economy: action
operation: saving_throw
ability: strength
dc: 24
targets:
  kind: creature
  selection: each
  area:
    shape: cone
    size_ft: 90
    origin: self
  exclude_already_affected_by: this_breath
failure:
  disadvantage_on:
    roll: d20_test
    ability: strength
  damage_roll_modifier:
    subtract:
      count: 1
      sides: 10
      modifier: 0
  repeat_save:
    event: target.turn_end
    success: end_effect
  automatic_success_after_seconds: 60
success:
  effects: []
```

##### Banish

<!-- splat:yaml path=splat_rule/parameters/features/banish -->

```yaml
economy: legendary_action
legendary_cost: 1
cooldown_until: self.next_turn_start
operation: saving_throw
ability: charisma
dc: 24
targets:
  kind: creature
  count: 1
  range_ft: 120
  visible: true
failure:
  damage:
  - average: 24
    roll:
      count: 7
      sides: 6
      modifier: 0
    type: force
  conditions:
  - condition: incapacitated
    until: self.next_turn_start
  transport:
    plane: harmless_demiplane
    until: self.next_turn_start
    return:
      unoccupied: true
      within_self_ft: 120
      chooser: self
success:
  effects: []
```

##### Guiding light

<!-- splat:yaml path=splat_rule/parameters/features/guiding_light -->

```yaml
economy: legendary_action
legendary_cost: 1
operation: cast_spell
spell:
  key: dnd2024:spells/guiding_bolt
cast_level: 4
spellcasting_profile: spellcasting
```

##### Pounce

<!-- splat:yaml path=splat_rule/parameters/features/pounce -->

```yaml
economy: legendary_action
legendary_cost: 1
operation: sequence
effects:
- operation: move
  maximum_ft:
    multiply:
    - stat: speed
    - 0.5
- operation: attack_sequence
  feature: rend
  count: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` insight ` | ` 10 ` |
| ` perception ` | ` 17 ` |
| ` persuasion ` | ` 16 ` |
| ` stealth ` | ` 9 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 60 ` |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 27 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
- draconic
understands: []
```

#### Legendary actions

<!-- splat:yaml path=splat_rule/parameters/legendary_actions -->

```yaml
uses: 3
recovery: self.turn_start
trigger: other_creature.turn_end
cost_per_action: 1
lair_uses: 4
requires:
  not_incapacitated: true
maximum_actions_per_trigger: 1
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```
