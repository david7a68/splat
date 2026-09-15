---
splat_schema: 1
splat_id: 3480ce3e-ec81-597d-9ecf-a57846062274
splat_kind: creature
splat_key: dnd2024:creatures/archmage
splat_name: Archmage
splat_origin:
  key: dnd2024:reference/creature/archmage
splat_sources:
- title: SRD 5.2.1
  locator: pp. 305
  url: https://www.dndbeyond.com/srd
---

# Archmage

## Source text

<!-- SRD page 305 -->

Medium or Small Humanoid (Wizard), Neutral\
AC 17 Initiative +7 (17)\
HP 170 (31d8 + 31)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 14 +2 +2 Con 12 +1 +1\
Int 20 +5 +9 Wis 15 +2 +6 Cha 16 +3 +3\
Skills Arcana +13, History +9, Perception +6\
Immunities Psychic; Charmed (with Mind Blank)\
Gear Wand\
Senses Passive Perception 16\
Languages Common plus five other languages\
CR 12 (XP 8,000; PB +4)\

### Traits

Magic Resistance. The archmage has Advantage on\
saving throws against spells and other magical effects.\

### Actions

Multiattack. The archmage makes four Arcane\
Burst attacks.\
Arcane Burst. Melee or Ranged Attack Roll: +9, reach\
5 ft. or range 150 ft. Hit: 27 (4d10 + 5) Force damage.\
Spellcasting. The archmage casts one of the following\
spells, using Intelligence as the spellcasting ability (spell\
save DC 17):\
At Will: Detect Magic, Detect Thoughts, Disguise Self,\
Invisibility, Light, Mage Armor (included in AC), Mage\
Hand, Prestidigitation\
2/Day Each: Fly, Lightning Bolt (level 7 version)\
1/Day Each: Cone of Cold (level 9 version), Mind Blank\
(cast before combat), Scrying, Teleport\

### Bonus Actions

Misty Step (3/Day). The mage casts Misty Step, using\
the same spellcasting ability as Spellcasting.\

### Reactions

Protective Magic (3/Day). The archmage casts Coun-\
terspell or Shield in response to the spell’s trigger, using\
the same spellcasting ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/305|page 305]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` tags_text ` | ` (Wizard) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Arcana +13, History +9, Perception +6 ` |
| ` immunities_text ` | ` Psychic; Charmed (with Mind Blank) ` |
| ` gear_text ` | ` Wand ` |
| ` senses_text ` | ` Passive Perception 16 ` |
| ` languages_text ` | ` Common plus five other languages ` |
| ` cr_text ` | ` 12 (XP 8,000; PB +4) ` |
| ` hit_points_text ` | ` 170 (31d8 + 31) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 20 ` | ` 9 ` |
| ` Wisdom ` | ` 15 ` | ` 6 ` |
| ` Charisma ` | ` 16 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 170 ` |  |
| ` initiative ` | ` 7 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/archmage ` |
| ` implementation ` | ` dnd2024:creature_archmage ` |

### Parameters

#### Features

##### Magic resistance

<!-- splat:yaml path=splat_rule/parameters/features/magic_resistance -->

```yaml
economy: passive
advantage:
  roll: saving_throw
  against:
  - spell
  - magical_effect
```

##### Arcane burst

<!-- splat:yaml path=splat_rule/parameters/features/arcane_burst -->

```yaml
economy: action
op: attack
attack_modes:
- melee
- ranged
attack_bonus: 9
targets:
  count: 1
hit:
- op: damage
  average: 27
  amount:
    dice:
      count: 4
      sides: 10
    add: 5
  damage_type: force
reach_feet: 5
range_feet: 150
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: arcane_burst
  count: 4
```

##### Spellcasting

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting -->

```yaml
economy: action
op: cast_spell_choice
ability: intelligence
spells:
- spell:
    key: dnd2024:spells/detect_magic
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/detect_thoughts
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/disguise_self
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/invisibility
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/light
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/mage_armor
  frequency: at_will
  precast: true
  included_in_supplied_ac: true
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/mage_hand
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/prestidigitation
  frequency: at_will
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/fly
  frequency: daily
  uses: 2
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
- spell:
    key: dnd2024:spells/lightning_bolt
  frequency: daily
  uses: 2
  resource_scope: per_spell
  cast_level: 7
  recovery: long_rest
- spell:
    key: dnd2024:spells/cone_of_cold
  frequency: daily
  uses: 1
  resource_scope: per_spell
  cast_level: 9
  recovery: long_rest
- spell:
    key: dnd2024:spells/mind_blank
  frequency: daily
  uses: 1
  resource_scope: per_spell
  precast: true
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
    key: dnd2024:spells/teleport
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
components_waived: []
save_dc: 17
required_material_components_available: true
```

##### Misty step

<!-- splat:yaml path=splat_rule/parameters/features/misty_step -->

```yaml
economy: bonus_action
resource:
  capacity: 3
  recovery: long_rest
  cost: 1
op: cast_spell
spell:
  key: dnd2024:spells/misty_step
via: spellcasting
cast_level: spell_minimum
```

##### Protective magic

<!-- splat:yaml path=splat_rule/parameters/features/protective_magic -->

```yaml
economy: reaction
resource:
  capacity: 3
  recovery: long_rest
  cost: 1
op: cast_spell_choice
choices:
- spell:
    key: dnd2024:spells/counterspell
- spell:
    key: dnd2024:spells/shield
trigger: chosen_spell.trigger
ability: intelligence
save_dc: 17
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 13 ` |
| ` history ` | ` 9 ` |
| ` perception ` | ` 6 ` |

#### Defenses

##### Immunities

<!-- splat:yaml path=splat_rule/parameters/defenses/immunities -->

```yaml
conditional:
- while_spell_active:
    key: dnd2024:spells/mind_blank
  damage_types:
  - psychic
  conditions:
  - charmed
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item_name | quantity |
| --- | --- | --- |
| ` 0 ` | ` Wand ` | ` 1 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["common"] ` |
| ` understands ` | ` ["common"] ` |

##### Additional languages

<!-- splat:values path=splat_rule/parameters/communication/additional_languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 5 ` |
| ` chooser ` | ` gm ` |
| ` distinct_from_known ` | ` true ` |
| ` can_speak ` | ` true ` |
| ` can_understand ` | ` true ` |

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
