---
splat_schema: 1
splat_id: d7f4119e-2381-5843-a7a9-5a90087a19fe
splat_kind: creature
splat_key: dnd2024:creatures/giant_owl
splat_name: Giant Owl
splat_origin:
  key: dnd2024:reference/creature/giant_owl
splat_sources:
- title: SRD 5.2.1
  locator: pp. 352
  url: https://www.dndbeyond.com/srd
---

# Giant Owl

## Source text

<!-- SRD page 352 -->

Large Celestial, Neutral\
AC 12 Initiative +2 (12)\
HP 19 (3d10 + 3)\
Speed 5 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 15 +2 +2 Con 12 +1 +1\
Int 10 +0 +0 Wis 14 +2 +4 Cha 10 +0 +0\
Skills Perception +6, Stealth +6\
Resistances Necrotic, Radiant\
Senses Darkvision 120 ft.; Passive Perception 16\
Languages Celestial; understands Common, Elvish, and\
Sylvan but can’t speak them\
CR 1/4 (XP 50; PB +2)\

### Traits

Flyby. The owl doesn’t provoke an Opportunity Attack\
when it flies out of an enemy’s reach.\

### Actions

Talons. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10\
+ 2) Slashing damage.\
Spellcasting. The owl casts one of the following spells,\
requiring no spell components and using Wisdom as\
the spellcasting ability:\
At Will: Detect Evil and Good, Detect Magic\
1/Day: Clairvoyance\

Original layout: [[../../srd-5.2.1/pages/352|page 352]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` celestial ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 5 ft., Fly 60 ft. ` |
| ` skills_text ` | ` Perception +6, Stealth +6 ` |
| ` resistances_text ` | ` Necrotic, Radiant ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 16 ` |
| ` languages_text ` | ` Celestial; understands Common, Elvish, and Sylvan but can’t speak them ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 19 (3d10 + 3) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 13 ` | ` 1 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 14 ` | ` 4 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 19 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 5 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_owl ` |
| ` implementation ` | ` dnd2024:creature_giant_owl ` |

### Parameters

#### Features

##### Flyby

<!-- splat:yaml path=splat_rule/parameters/features/flyby -->

```yaml
economy: passive
opportunity_attack_immunity:
  trigger: fly_out_of_enemy_reach
```

##### Talons

<!-- splat:yaml path=splat_rule/parameters/features/talons -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 4
targets:
  count: 1
hit:
- op: damage
  average: 7
  amount:
    dice:
      count: 1
      sides: 10
    add: 2
  damage_type: slashing
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
    key: dnd2024:spells/clairvoyance
  frequency: daily
  uses: 1
  resource_scope: per_spell
  recovery: long_rest
  cast_level: spell_minimum
components_waived:
- verbal
- somatic
- material
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |
| ` stealth ` | ` 6 ` |

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["necrotic", "radiant"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["celestial"] ` |
| ` understands ` | ` ["celestial", "common", "elvish", "sylvan"] ` |

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
