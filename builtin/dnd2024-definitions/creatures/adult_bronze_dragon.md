---
splat_schema: 1
splat_id: 5b9a0df5-b2a4-5f4f-add9-6ec294914017
splat_kind: creature
splat_key: dnd2024:creatures/adult_bronze_dragon
splat_name: Adult Bronze Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_bronze_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 270, 271
  url: https://www.dndbeyond.com/srd
---

# Adult Bronze Dragon

## Source text

<!-- SRD page 270 -->

Huge Dragon (Metallic), Lawful Good\
AC 18 Initiative +10 (20)\
HP 212 (17d12 + 102)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 25 +7 +7 Dex 10 +0 +5 Con 23 +6 +6\
Int 16 +3 +3 Wis 15 +2 +7 Cha 20 +5 +5\
Skills Insight +7, Perception +12, Stealth +5\
Immunities Lightning\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 22\
Languages Common, Draconic\
CR 15 (XP 13,000, or 15,000 in lair; PB +5)\

### Traits

Amphibious. The dragon can breathe air and water.\
Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
It can replace one attack with a use of (A) Repulsion\
Breath or (B) Spellcasting to cast Guiding Bolt (level\
2 version).\
Rend. Melee Attack Roll: +12, reach 10 ft. Hit: 16 (2d8\
+ 7) Slashing damage plus 5 (1d10) Lightning damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 19, each creature in a 90-foot-long, 5-foot-\
wide Line. Failure: 55 (10d10) Lightning damage. Suc-\
cess: Half damage.\
Repulsion Breath. Strength Saving Throw: DC 19, each\
creature in a 30-foot Cone. Failure: The target is pushed\
up to 60 feet straight away from the dragon and has the\
Prone condition.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 17,\
+10 to hit with spell attacks):\
At Will: Detect Magic, Guiding Bolt (level 2 version),\
Shapechange (Beast or Humanoid form only, no\
Temporary Hit Points gained from the spell, and no\
Concentration or Temporary Hit Points required to\
maintain the spell), Speak with Animals, Thaumaturgy\
1/Day Each: Detect Thoughts, Water Breathing\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Guiding Light. The dragon uses Spellcasting to cast\
Guiding Bolt (level 2 version).\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\
Thunderclap. Constitution Saving Throw: DC 17, each\
creature in a 20-foot-radius Sphere centered on a point\
the dragon can see within 90 feet. Failure: 10 (3d6)\

<!-- SRD page 271 -->

Thunder damage, and the target has the Deafened con-\
dition until the end of its next turn.\

Original layout: [[../../srd-5.2.1/pages/270|page 270]], [[../../srd-5.2.1/pages/271|page 271]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Lawful Good ` |
| ` tags_text ` | ` (Metallic) ` |
| ` movement_text ` | ` 40 ft., Fly 80 ft., Swim 40 ft. ` |
| ` skills_text ` | ` Insight +7, Perception +12, Stealth +5 ` |
| ` immunities_text ` | ` Lightning ` |
| ` senses_text ` | ` Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 22 ` |
| ` languages_text ` | ` Common, Draconic ` |
| ` cr_text ` | ` 15 (XP 13,000, or 15,000 in lair; PB +5) ` |
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
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 212 ` |  |
| ` initiative ` | ` 10 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/adult_bronze_dragon ` |
| ` implementation ` | ` dnd2024:creature_adult_bronze_dragon ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` breathing_media ` | ` ["air", "water"] ` |

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
resource:
  capacity: 3
  capacity_in_lair: 4
  recovery: long_rest
  cost: 1
trigger: saving_throw.failed
optional: true
op: replace_save_outcome
outcome: success
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 12
targets:
  count: 1
hit:
- op: damage
  average: 16
  amount:
    dice:
      count: 2
      sides: 8
    add: 7
  damage_type: slashing
- op: damage
  average: 5
  amount:
    dice:
      count: 1
      sides: 10
    add: 0
  damage_type: lightning
reach_feet: 10
```

##### Lightning breath

<!-- splat:yaml path=splat_rule/parameters/features/lightning_breath -->

```yaml
economy: action
resource:
  capacity: 1
  initial: 1
  recharge:
    event: turn.start
    participant: self
    roll:
      dice:
        count: 1
        sides: 6
      add: 0
    succeeds_on:
    - 5
    - 6
  cost: 1
  recovery_also:
  - short_rest
  - long_rest
op: save
ability: dexterity
dc: 19
targets:
  kind: creature
  count: all
  area:
    shape: line
    size_feet: 90
    origin: self
    width_feet: 5
failure:
- op: damage
  average: 55
  amount:
    dice:
      count: 10
      sides: 10
    add: 0
  damage_type: lightning
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` sequence ` |

###### Steps

<!-- splat:rows path=splat_rule/parameters/features/multiattack/steps -->

| Row | op | feature | count |
| --- | --- | --- | --- |
| ` 0 ` | ` use_feature ` | ` rend ` | ` 3 ` |

###### Replacement

<!-- splat:values path=splat_rule/parameters/features/multiattack/replacement -->

| Field | Value |
| --- | --- |
| ` max_attacks ` | ` 1 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/multiattack/replacement/choices
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
  Spell:
    path: spell
    type: reference
    optional: true
  Cast level:
    path: cast_level
    type: integer
    optional: true
-->

| Key | Feature | Spell | Cast level |
| --- | --- | --- | --- |
| ` row_1 ` | ` Repulsion breath ` |  |  |
| ` row_2 ` | ` Spellcasting ` | [Guiding Bolt](<../spells/guiding_bolt.md> "dnd2024:spells/guiding_bolt") | ` 2 ` |

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 17 ` |
| ` spell_attack_bonus ` | ` 10 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- detect_magic
- guiding_bolt
- shapechange
- speak_with_animals
- thaumaturgy
- detect_thoughts
- water_breathing
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: frequency
    type: symbol
  Overrides form types:
    path: overrides/form_types
    type: yaml
    optional: true
  Overrides temporary hit points gained:
    path: overrides/temporary_hit_points_gained
    type: integer
    optional: true
  Overrides concentration required:
    path: overrides/concentration_required
    type: boolean
    optional: true
  Overrides temporary hit points to maintain required:
    path: overrides/temporary_hit_points_to_maintain_required
    type: boolean
    optional: true
  Uses:
    path: uses
    type: integer
    optional: true
  Resource scope:
    path: resource_scope
    type: symbol
    optional: true
  Recovery:
    path: recovery
    type: symbol
    optional: true
  Details:
    type: section
    optional: true
-->

| Spell | Frequency | Overrides form types | Overrides temporary hit points gained | Overrides concentration required | Overrides temporary hit points to maintain required | Uses | Resource scope | Recovery | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |  |  |  |  |  |  |  | [Rules](#^splat-68a383d1-detect_magic "section:detect_magic") |
| [Guiding Bolt](<../spells/guiding_bolt.md> "dnd2024:spells/guiding_bolt") | ` At will ` |  |  |  |  |  |  |  | [Rules](#^splat-68a383d1-guiding_bolt "section:guiding_bolt") |
| [Shapechange](<../spells/shapechange.md> "dnd2024:spells/shapechange") | ` At will ` | ` ["beast", "humanoid"] ` | ` 0 ` | false | false |  |  |  | [Rules](#^splat-68a383d1-shapechange "section:shapechange") |
| [Speak with Animals](<../spells/speak_with_animals.md> "dnd2024:spells/speak_with_animals") | ` At will ` |  |  |  |  |  |  |  | [Rules](#^splat-68a383d1-speak_with_animals "section:speak_with_animals") |
| [Thaumaturgy](<../spells/thaumaturgy.md> "dnd2024:spells/thaumaturgy") | ` At will ` |  |  |  |  |  |  |  | [Rules](#^splat-68a383d1-thaumaturgy "section:thaumaturgy") |
| [Detect Thoughts](<../spells/detect_thoughts.md> "dnd2024:spells/detect_thoughts") | ` Daily ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-detect_thoughts "section:detect_thoughts") |
| [Water Breathing](<../spells/water_breathing.md> "dnd2024:spells/water_breathing") | ` Daily ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-water_breathing "section:water_breathing") |

###### Detect magic

^splat-68a383d1-detect_magic

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@detect_magic/cast_level -->

```yaml
spell_minimum
```

###### Guiding bolt

^splat-68a383d1-guiding_bolt

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@guiding_bolt/cast_level -->

```yaml
2
```

###### Shapechange

^splat-68a383d1-shapechange

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@shapechange/cast_level -->

```yaml
spell_minimum
```

###### Speak with animals

^splat-68a383d1-speak_with_animals

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@speak_with_animals/cast_level -->

```yaml
spell_minimum
```

###### Thaumaturgy

^splat-68a383d1-thaumaturgy

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@thaumaturgy/cast_level -->

```yaml
spell_minimum
```

###### Detect thoughts

^splat-68a383d1-detect_thoughts

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@detect_thoughts/cast_level -->

```yaml
spell_minimum
```

###### Water breathing

^splat-68a383d1-water_breathing

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@water_breathing/cast_level -->

```yaml
spell_minimum
```

##### Thunderclap

<!-- splat:yaml path=splat_rule/parameters/features/thunderclap -->

```yaml
economy: legendary_action
op: save
ability: constitution
dc: 17
targets:
  kind: creature
  count: all
  area:
    shape: sphere
    size_feet: 20
    origin: chosen_point
    origin_visible_to: self
    origin_range_feet: 90
failure:
- op: damage
  average: 10
  amount:
    dice:
      count: 3
      sides: 6
    add: 0
  damage_type: thunder
- op: condition
  condition: deafened
  ends:
    event: turn.end
    participant: target
    occurrence: next
success: []
```

##### Repulsion breath

<!-- splat:yaml path=splat_rule/parameters/features/repulsion_breath -->

```yaml
economy: action
op: save
ability: strength
dc: 19
targets:
  kind: creature
  count: all
  area:
    shape: cone
    size_feet: 30
    origin: self
failure:
- op: push
  distance_max_feet: 60
  direction: straight_away_from_self
- op: condition
  condition: prone
success: []
```

##### Guiding light

<!-- splat:yaml path=splat_rule/parameters/features/guiding_light -->

```yaml
economy: legendary_action
op: cast_spell
spell:
  key: dnd2024:spells/guiding_bolt
via: spellcasting
cast_level: 2
```

##### Pounce

<!-- splat:yaml path=splat_rule/parameters/features/pounce -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: move
  distance_max:
    multiply:
    - stat: speed
    - numerator: 1
      denominator: 2
- op: use_feature
  feature: rend
  count: 1
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` insight ` | ` 7 ` |
| ` perception ` | ` 12 ` |
| ` stealth ` | ` 5 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["lightning"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 22 ` |

##### Special

<!-- splat:table
path: splat_rule/parameters/senses/special
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
  Range feet:
    path: range_feet
    type: integer
-->

| Key | Sense | Range feet |
| --- | --- | --- |
| ` row_1 ` | ` Blindsight ` | ` 60 ` |
| ` row_2 ` | ` Darkvision ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common", "draconic"] ` |
| ` understands ` | ` ["common", "draconic"] ` |

#### Legendary actions

<!-- splat:yaml path=splat_rule/parameters/legendary_actions -->

```yaml
uses: 3
uses_in_lair: 4
cost_per_action: 1
trigger: another_creature.turn.end
recovery:
  event: turn.start
  participant: self
  amount: all
maximum_per_trigger: 1
requires:
  self_incapacitated: false
  self_can_take_actions: true
```

#### Stat block conventions

<!-- splat:yaml path=splat_rule/parameters/stat_block_conventions -->

```yaml
attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
save_dc: supplied_total
damage_mode_choice:
- fixed_average
- rolled_dice
condition_source: self
multiattack_child_economy_cost: 0
multiattack_child_resource_costs_retained: true
spellcasting:
  default_level: spell_minimum
  spell_slots_expended: false
  required_materials: available
  casting_time_at_least_one_minute:
    uses_normal_casting_duration: true
    magic_action_each_turn: true
    concentration_required: true
geometry_assumptions:
  chooser: gm
  supplies:
  - range
  - visibility
  - line_of_sight
  - area_membership
  - reachable_locations
  - unoccupied_locations
  - terrain
  - plane
  - form_appearance
```
