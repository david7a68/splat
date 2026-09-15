---
splat_schema: 1
splat_id: 9e1c2359-fb23-576e-abf4-42956b5e8bc6
splat_kind: creature
splat_key: dnd2024:creatures/lich
splat_name: Lich
splat_origin:
  key: dnd2024:reference/creature/lich
splat_sources:
- title: SRD 5.2.1
  locator: pp. 304
  url: https://www.dndbeyond.com/srd
---

# Lich

## Source text

<!-- SRD page 304 -->

Medium Undead (Wizard), Neutral Evil\
AC 20 Initiative +17 (27)\
HP 315 (42d8 + 126)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 16 +3 +10 Con 16 +3 +10\
Int 21 +5 +12 Wis 14 +2 +9 Cha 16 +3 +3\
Skills Arcana +19, History +12, Insight +9, Perception +9\
Resistances Cold, Lightning\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Frightened, Paralyzed, Poisoned\
Gear Component Pouch\
Senses Truesight 120 ft.; Passive Perception 19\
Languages All\
CR 21 (XP 33,000, or 41,000 in lair; PB +7)\

### Traits

Legendary Resistance (4/Day, or 5/Day in Lair). If\
the lich fails a saving throw, it can choose to suc-\
ceed instead.\
Spirit Jar. If destroyed, the lich reforms in 1d10 days\
if it has a spirit jar, reviving with all its Hit Points. The\
new body appears in an unoccupied space within the\
lich’s lair.\

### Actions

Multiattack. The lich makes three attacks, using El-\
dritch Burst or Paralyzing Touch in any combination.\
Eldritch Burst. Melee or Ranged Attack Roll: +12, reach\
5 ft. or range 120 ft. Hit: 31 (4d12 + 5) Force damage.\
Paralyzing Touch. Melee Attack Roll: +12, reach 5\
ft. Hit: 15 (3d6 + 5) Cold damage, and the target has\
the Paralyzed condition until the start of the lich’s\
next turn.\
Spellcasting. The lich casts one of the following spells,\
using Intelligence as the spellcasting ability (spell\
save DC 20):\
At Will: Detect Magic, Detect Thoughts, Dispel Magic,\
Fireball (level 5 version), Invisibility, Lightning Bolt\
(level 5 version), Mage Hand, Prestidigitation\
2/Day Each: Animate Dead, Dimension Door,\
Plane Shift\
1/Day Each: Chain Lightning, Finger of Death, Power\
Word Kill, Scrying\

### Reactions

Protective Magic. The lich casts Counterspell or Shield\
in response to the spell’s trigger, using the same spell-\
casting ability as Spellcasting.\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the lich can expend a use to take\
one of the following actions. The lich regains all expended\
uses at the start of each of its turns.\
Deathly Teleport. The lich teleports up to 60 feet to\
an unoccupied space it can see, and each creature\
within 10 feet of the space it left takes 11 (2d10) Ne-\
crotic damage.\
Disrupt Life. Constitution Saving Throw: DC 20, each\
creature that isn’t an Undead in a 20-foot Emanation\
originating from the lich. Failure: 31 (9d6) Necrotic\
damage. Success: Half damage. Failure or Success:\
The lich can’t take this action again until the start of its\
next turn.\
Frightening Gaze. The lich casts Fear, using the same\
spellcasting ability as Spellcasting. The lich can’t take\
this action again until the start of its next turn.\

Original layout: [[../../srd-5.2.1/pages/304|page 304]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Neutral Evil ` |
| ` tags_text ` | ` (Wizard) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Arcana +19, History +12, Insight +9, Perception +9 ` |
| ` resistances_text ` | ` Cold, Lightning ` |
| ` immunities_text ` | ` Necrotic, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned ` |
| ` gear_text ` | ` Component Pouch ` |
| ` senses_text ` | ` Truesight 120 ft.; Passive Perception 19 ` |
| ` languages_text ` | ` All ` |
| ` cr_text ` | ` 21 (XP 33,000, or 41,000 in lair; PB +7) ` |
| ` hit_points_text ` | ` 315 (42d8 + 126) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 11 ` | ` 0 ` |
| ` Dexterity ` | ` 16 ` | ` 10 ` |
| ` Constitution ` | ` 16 ` | ` 10 ` |
| ` Intelligence ` | ` 21 ` | ` 12 ` |
| ` Wisdom ` | ` 14 ` | ` 9 ` |
| ` Charisma ` | ` 16 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 20 ` |  |
| ` max_hp ` | ` 315 ` |  |
| ` initiative ` | ` 17 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/lich ` |
| ` implementation ` | ` dnd2024:creature_lich ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold", "lightning"] ` |
| ` damage_immunities ` | ` ["necrotic", "poison"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened", "paralyzed", "poisoned"] ` |

#### Features

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

##### Spirit jar

<!-- splat:yaml path=splat_rule/parameters/features/spirit_jar -->

```yaml
economy: trait
operation: passive_rule
trigger: self.destroyed
requires:
  has_spirit_jar: true
new_body:
  delay:
    roll:
      count: 1
      sides: 10
      modifier: 0
    unit: day
  hp: maximum
  location:
    unoccupied: true
    in: self.lair
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 3
  options:
  - eldritch_burst
  - paralyzing_touch
```

##### Eldritch burst

<!-- splat:yaml path=splat_rule/parameters/features/eldritch_burst -->

```yaml
economy: action
operation: attack
attack:
  mode: melee_or_ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 12
  reach_ft: 5
  range_ft: 120
  long_range_ft: null
hit:
  damage:
  - average: 31
    roll:
      count: 4
      sides: 12
      modifier: 5
    type: force
```

##### Paralyzing touch

<!-- splat:yaml path=splat_rule/parameters/features/paralyzing_touch -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 12
  reach_ft: 5
hit:
  damage:
  - average: 15
    roll:
      count: 3
      sides: 6
      modifier: 5
    type: cold
  conditions:
  - condition: paralyzed
    until: self.next_turn_start
```

##### Spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting -->

```yaml
economy: action
operation: spellcasting
ability: intelligence
components_waived: []
save_dc: 20
options:
- spell:
    key: dnd2024:spells/detect_magic
  usage:
    kind: at_will
- spell:
    key: dnd2024:spells/detect_thoughts
  usage:
    kind: at_will
- spell:
    key: dnd2024:spells/dispel_magic
  usage:
    kind: at_will
- spell:
    key: dnd2024:spells/fireball
  usage:
    kind: at_will
  cast_level: 5
- spell:
    key: dnd2024:spells/invisibility
  usage:
    kind: at_will
- spell:
    key: dnd2024:spells/lightning_bolt
  usage:
    kind: at_will
  cast_level: 5
- spell:
    key: dnd2024:spells/mage_hand
  usage:
    kind: at_will
- spell:
    key: dnd2024:spells/prestidigitation
  usage:
    kind: at_will
- spell:
    key: dnd2024:spells/animate_dead
  usage:
    kind: daily
    uses: 2
    pool: per_spell
    recovery: long_rest
- spell:
    key: dnd2024:spells/dimension_door
  usage:
    kind: daily
    uses: 2
    pool: per_spell
    recovery: long_rest
- spell:
    key: dnd2024:spells/plane_shift
  usage:
    kind: daily
    uses: 2
    pool: per_spell
    recovery: long_rest
- spell:
    key: dnd2024:spells/chain_lightning
  usage:
    kind: daily
    uses: 1
    pool: per_spell
    recovery: long_rest
- spell:
    key: dnd2024:spells/finger_of_death
  usage:
    kind: daily
    uses: 1
    pool: per_spell
    recovery: long_rest
- spell:
    key: dnd2024:spells/power_word_kill
  usage:
    kind: daily
    uses: 1
    pool: per_spell
    recovery: long_rest
- spell:
    key: dnd2024:spells/scrying
  usage:
    kind: daily
    uses: 1
    pool: per_spell
    recovery: long_rest
material_components_available: true
```

##### Protective magic

<!-- splat:yaml path=splat_rule/parameters/features/protective_magic -->

```yaml
economy: reaction
operation: spell_choice
options:
- operation: cast_spell
  spell:
    key: dnd2024:spells/counterspell
- operation: cast_spell
  spell:
    key: dnd2024:spells/shield
trigger: chosen_spell.normal_trigger
ability: intelligence
save_dc: 20
```

##### Deathly teleport

<!-- splat:yaml path=splat_rule/parameters/features/deathly_teleport -->

```yaml
economy: legendary_action
legendary_cost: 1
operation: sequence
effects:
- operation: teleport
  maximum_ft: 60
  visible: true
  unoccupied: true
- operation: damage
  targets:
    kind: creature
    selection: each
    within_old_space_ft: 10
  damage:
  - roll:
      count: 2
      sides: 10
      modifier: 0
    type: necrotic
```

##### Disrupt life

<!-- splat:yaml path=splat_rule/parameters/features/disrupt_life -->

```yaml
economy: legendary_action
legendary_cost: 1
cooldown_until: self.next_turn_start
operation: saving_throw
ability: constitution
dc: 20
targets:
  kind: creature
  selection: each
  area:
    shape: emanation
    size_ft: 20
    origin: self
  exclude_types:
  - undead
failure:
  damage:
  - average: 31
    roll:
      count: 9
      sides: 6
      modifier: 0
    type: necrotic
success:
  damage_multiplier: 0.5
  other_effects: false
```

##### Frightening gaze

<!-- splat:yaml path=splat_rule/parameters/features/frightening_gaze -->

```yaml
economy: legendary_action
legendary_cost: 1
cooldown_until: self.next_turn_start
operation: cast_spell
spell:
  key: dnd2024:spells/fear
ability: intelligence
save_dc: 20
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 19 ` |
| ` history ` | ` 12 ` |
| ` insight ` | ` 9 ` |
| ` perception ` | ` 9 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | quantity |
| --- | --- | --- |
| ` 0 ` | ` component_pouch ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` truesight ` | ` 120 ` |
| ` passive_perception ` | ` 19 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: all
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
