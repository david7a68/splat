---
splat_schema: 1
splat_id: d4c20ad9-5f3e-5c66-9df0-6af616fc1ec9
splat_kind: creature
splat_key: dnd2024:creatures/couatl
splat_name: Couatl
splat_origin:
  key: dnd2024:reference/creature/couatl
splat_sources:
- title: SRD 5.2.1
  locator: pp. 277, 278
  url: https://www.dndbeyond.com/srd
---

# Couatl

## Source text

<!-- SRD page 277 -->

Medium Celestial, Lawful Good\
AC 19 Initiative +5 (15)\
HP 60 (8d8 + 24)\
Speed 30 ft., Fly 90 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 20 +5 +5 Con 17 +3 +5\
Int 18 +4 +4 Wis 20 +5 +7 Cha 18 +4 +4\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Psychic, Radiant\
Senses Truesight 120 ft.; Passive Perception 15\
Languages All; telepathy 120 ft.\
CR 4 (XP 1,100; PB +2)\

### Traits

Shielded Mind. The couatl’s thoughts can’t be read by\
any means, and other creatures can communicate with\
it telepathically only if it allows them.\

### Actions

Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 11 (1d12\
+ 5) Piercing damage, and the target has the Poisoned\
condition until the end of the couatl’s next turn.\
Constrict. Strength Saving Throw: DC 15, one Medium\
or smaller creature the couatl can see within 5 feet.\
Failure: 8 (1d6 + 5) Bludgeoning damage. The target has\
the Grappled condition (escape DC 13), and it has the\
Restrained condition until the grapple ends.\
Spellcasting. The couatl casts one of the following\
spells, requiring no spell components and using Wis-\
dom as the spellcasting ability (spell save DC 15):\
At Will: Detect Evil and Good, Detect Magic, Detect\
Thoughts, Shapechange (Beast or Humanoid form\
only, no Temporary Hit Points gained from the spell,\

<!-- SRD page 278 -->

and no Concentration or Temporary Hit Points re-\
quired to maintain the spell)\
1/Day Each: Create Food and Water, Dream, Greater\
Restoration, Scrying, Sleep\

### Bonus Actions

Divine Aid (2/Day). The couatl casts Bless, Lesser Res-\
toration, or Sanctuary, requiring no spell components\
and using the same spellcasting ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/277|page 277]], [[../../srd-5.2.1/pages/278|page 278]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` celestial ` |
| ` alignment ` | ` Lawful Good ` |
| ` movement_text ` | ` 30 ft., Fly 90 ft. ` |
| ` resistances_text ` | ` Bludgeoning, Piercing, Slashing ` |
| ` immunities_text ` | ` Psychic, Radiant ` |
| ` senses_text ` | ` Truesight 120 ft.; Passive Perception 15 ` |
| ` languages_text ` | ` All; telepathy 120 ft. ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 60 (8d8 + 24) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 20 ` | ` 5 ` |
| ` Constitution ` | ` 17 ` | ` 5 ` |
| ` Intelligence ` | ` 18 ` | ` 4 ` |
| ` Wisdom ` | ` 20 ` | ` 7 ` |
| ` Charisma ` | ` 18 ` | ` 4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 19 ` |  |
| ` max_hp ` | ` 60 ` |  |
| ` initiative ` | ` 5 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/couatl ` |
| ` implementation ` | ` dnd2024:creature_couatl ` |

### Parameters

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 7
targets:
  count: 1
hit:
- op: damage
  average: 11
  amount:
    dice:
      count: 1
      sides: 12
    add: 5
  damage_type: piercing
- op: condition
  condition: poisoned
  ends:
    event: turn.end
    participant: self
    occurrence: next
reach_feet: 5
```

##### Spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting -->

```yaml
economy: action
op: cast_spell_choice
ability: wisdom
spells:
- spell:
    key: dnd2024:spells/detect_evil_and_good
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/detect_magic
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/detect_thoughts
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/shapechange
  frequency: at_will
  overrides:
    form_types:
    - beast
    - humanoid
    temporary_hit_points_gained: 0
    concentration_required: false
    temporary_hit_points_to_maintain_required: false
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/create_food_and_water
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/dream
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/greater_restoration
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/scrying
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/sleep
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
components_waived:
- verbal
- somatic
- material
save_dc: 15
```

##### Shielded mind

<!-- splat:values path=splat_rule/parameters/features/shielded_mind -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` thought_reading_allowed ` | ` false ` |
| ` incoming_telepathy_requires_self_permission ` | ` true ` |

##### Divine aid

<!-- splat:yaml path=splat_rule/parameters/features/divine_aid -->

```yaml
economy: bonus_action
resource:
  capacity: 2
  cost: 1
  recovery: long_rest
  scope: shared_choices
op: cast_spell_choice
choices:
- spell:
    key: dnd2024:spells/bless
- spell:
    key: dnd2024:spells/lesser_restoration
- spell:
    key: dnd2024:spells/sanctuary
ability: wisdom
save_dc: 15
components_waived:
- verbal
- somatic
- material
```

##### Constrict

<!-- splat:yaml path=splat_rule/parameters/features/constrict -->

```yaml
economy: action
op: save
ability: strength
dc: 15
targets:
  kind: creature
  count: 1
  range_feet: 5
  visible_to: self
  size_max: medium
failure:
- op: damage
  average: 8
  amount:
    dice:
      count: 1
      sides: 6
    add: 5
  damage_type: bludgeoning
- op: condition
  condition: grappled
  escape_dc: 13
  source: self
- op: condition
  condition: restrained
  ends:
    event: grapple.end
    source: self
success: []
```

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["bludgeoning", "piercing", "slashing"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["psychic", "radiant"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` truesight ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` all ` |
| ` understands ` | ` all ` |

##### Telepathy

<!-- splat:values path=splat_rule/parameters/communication/telepathy -->

| Field | Value |
| --- | --- |
| ` range_feet ` | ` 120 ` |

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
