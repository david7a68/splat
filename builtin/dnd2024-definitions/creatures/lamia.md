---
splat_schema: 1
splat_id: 4eb6a57f-e362-59ad-8474-2d2e0563379a
splat_kind: creature
splat_key: dnd2024:creatures/lamia
splat_name: Lamia
splat_origin:
  key: dnd2024:reference/creature/lamia
splat_sources:
- title: SRD 5.2.1
  locator: pp. 303, 304
  url: https://www.dndbeyond.com/srd
---

# Lamia

## Source text

<!-- SRD page 303 -->

Large Fiend, Chaotic Evil\
AC 13 Initiative +1 (11)\
HP 97 (13d10 + 26)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 13 +1 +1 Con 15 +2 +2\
Int 14 +2 +2 Wis 15 +2 +2 Cha 16 +3 +3\
Skills Deception +7, Insight +4, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Abyssal, Common\
CR 4 (XP 1,100; PB +2)\

### Actions

Multiattack. The lamia makes two Claw attacks. It can\
replace one attack with a use of Corrupting Touch.\
Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Slashing damage plus 7 (2d6) Psychic damage.\
Corrupting Touch. Wisdom Saving Throw: DC 13, one\
creature the lamia can see within 5 feet. Failure: 13\
(3d8) Psychic damage, and the target is cursed for 1\
hour. Until the curse ends, the target has the Charmed\
and Poisoned conditions.\
Spellcasting. The lamia casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 13):\
At Will: Disguise Self (can appear as a Large or Medium\
biped), Minor Illusion\
1/Day Each: Geas, Major Image, Scrying\

<!-- SRD page 304 -->


### Bonus Actions

Leap. The lamia jumps up to 30 feet by spending 10\
feet of movement.\

Original layout: [[../../srd-5.2.1/pages/303|page 303]], [[../../srd-5.2.1/pages/304|page 304]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Deception +7, Insight +4, Stealth +5 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 12 ` |
| ` languages_text ` | ` Abyssal, Common ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 97 (13d10 + 26) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 15 ` | ` 2 ` |
| ` Intelligence ` | ` 14 ` | ` 2 ` |
| ` Wisdom ` | ` 15 ` | ` 2 ` |
| ` Charisma ` | ` 16 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 97 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/lamia ` |
| ` implementation ` | ` dnd2024:creature_lamia ` |

### Parameters

#### Features

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 7
  amount:
    dice:
      count: 1
      sides: 8
    add: 3
  damage_type: slashing
- op: damage
  average: 7
  amount:
    dice:
      count: 2
      sides: 6
    add: 0
  damage_type: psychic
reach_feet: 5
```

##### Leap

<!-- splat:values path=splat_rule/parameters/features/leap -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` jump ` |
| ` distance_max_feet ` | ` 30 ` |
| ` movement_cost_feet ` | ` 10 ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: claw
  count: 2
replacement:
  max_attacks: 1
  choices:
  - feature: corrupting_touch
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 13 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- disguise_self
- minor_illusion
- geas
- major_image
- scrying
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: frequency
    type: symbol
  Overrides appearance sizes:
    path: overrides/appearance/sizes
    type: yaml
    optional: true
  Overrides appearance anatomy:
    path: overrides/appearance/anatomy
    type: symbol
    optional: true
  Cast level:
    path: cast_level
    type: symbol
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
-->

| Spell | Frequency | Overrides appearance sizes | Overrides appearance anatomy | Cast level | Uses | Resource scope | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [Disguise Self](<../spells/disguise_self.md> "dnd2024:spells/disguise_self") | ` At will ` | ` ["large", "medium"] ` | ` Biped ` | ` Spell minimum ` |  |  |  |
| [Minor Illusion](<../spells/minor_illusion.md> "dnd2024:spells/minor_illusion") | ` At will ` |  |  | ` Spell minimum ` |  |  |  |
| [Geas](<../spells/geas.md> "dnd2024:spells/geas") | ` Daily ` |  |  | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Major Image](<../spells/major_image.md> "dnd2024:spells/major_image") | ` Daily ` |  |  | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` Daily ` |  |  | ` Spell minimum ` | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Corrupting touch

<!-- splat:yaml path=splat_rule/parameters/features/corrupting_touch -->

```yaml
economy: action
op: save
ability: wisdom
dc: 13
targets:
  kind: creature
  count: 1
  range_feet: 5
  visible_to: self
failure:
- op: damage
  average: 13
  amount:
    dice:
      count: 3
      sides: 8
    add: 0
  damage_type: psychic
- op: curse
  duration:
    hours: 1
  effects:
  - op: condition
    condition: charmed
    source: self
  - op: condition
    condition: poisoned
success: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 7 ` |
| ` insight ` | ` 4 ` |
| ` stealth ` | ` 5 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["abyssal", "common"] ` |
| ` understands ` | ` ["abyssal", "common"] ` |

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
