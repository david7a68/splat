---
splat_schema: 1
splat_id: de897d10-b4b2-5e40-92b8-ea286c2d35cd
splat_kind: creature
splat_key: dnd2024:creatures/guardian_naga
splat_name: Guardian Naga
splat_origin:
  key: dnd2024:reference/creature/guardian_naga
splat_sources:
- title: SRD 5.2.1
  locator: pp. 296
  url: https://www.dndbeyond.com/srd
---

# Guardian Naga

## Source text

<!-- SRD page 296 -->

Large Celestial, Lawful Good\
AC 18 Initiative +4 (14)\
HP 136 (16d10 + 48)\
Speed 40 ft., Climb 40 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 18 +4 +8 Con 16 +3 +7\
Int 16 +3 +7 Wis 19 +4 +8 Cha 18 +4 +8\
Skills Arcana +11, History +11, Religion +11\
Immunities Poison; Charmed, Paralyzed, Poisoned, Restrained\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Celestial, Common\
CR 10 (XP 5,900; PB +4)

### Traits

Celestial Restoration. If the naga dies, it returns to life in 1d6 days and regains all its Hit Points unless Dispel Evil and Good is cast on its remains.

### Actions

Multiattack. The naga makes two Bite attacks. It can replace any attack with a use of Poisonous Spittle.

Bite. Melee Attack Roll: +8, reach 10 ft. Hit: 17 (2d12 + 4) Piercing damage plus 22 (4d10) Poison damage.

Poisonous Spittle. Constitution Saving Throw: DC 16, one creature the naga can see within 60 feet. Failure: 31 (7d8) Poison damage, and the target has the Blinded condition until the start of the naga’s next turn. Success: Half damage only.

Spellcasting. The naga casts one of the following spells, requiring no Somatic or Material components and using Wisdom as the spellcasting ability (spell save DC 16):

At Will: Thaumaturgy\
1/Day Each: Clairvoyance, Cure Wounds (level 6 version), Flame Strike (level 6 version), Geas, True Seeing

Original layout: [[../../srd-5.2.1/pages/296|page 296]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` celestial ` |
| ` alignment ` | ` Lawful Good ` |
| ` movement_text ` | ` 40 ft., Climb 40 ft., Swim 40 ft. ` |
| ` skills_text ` | ` Arcana +11, History +11, Religion +11 ` |
| ` immunities_text ` | ` Poison; Charmed, Paralyzed, Poisoned, Restrained ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 14 ` |
| ` languages_text ` | ` Celestial, Common ` |
| ` cr_text ` | ` 10 (XP 5,900; PB +4) ` |
| ` hit_points_text ` | ` 136 (16d10 + 48) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 18 ` | ` 8 ` |
| ` Constitution ` | ` 16 ` | ` 7 ` |
| ` Intelligence ` | ` 16 ` | ` 7 ` |
| ` Wisdom ` | ` 19 ` | ` 8 ` |
| ` Charisma ` | ` 18 ` | ` 8 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 136 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` climb_speed ` | ` 40 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/guardian_naga ` |
| ` implementation ` | ` dnd2024:creature_guardian_naga ` |

### Parameters

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 8
targets:
  count: 1
hit:
- op: damage
  average: 17
  amount:
    dice:
      count: 2
      sides: 12
    add: 4
  damage_type: piercing
- op: damage
  average: 22
  amount:
    dice:
      count: 4
      sides: 10
    add: 0
  damage_type: poison
reach_feet: 10
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: bite
  count: 2
replacement:
  max_attacks: all
  choices:
  - feature: poisonous_spittle
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` wisdom ` |
| ` components_waived ` | ` ["somatic", "material"] ` |
| ` save_dc ` | ` 16 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- thaumaturgy
- clairvoyance
- cure_wounds
- flame_strike
- geas
- true_seeing
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: frequency
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
  Details:
    type: section
    optional: true
-->

| Spell | Frequency | Uses | Resource scope | Recovery | Details |
| --- | --- | --- | --- | --- | --- |
| [Thaumaturgy](<../spells/thaumaturgy.md> "dnd2024:spells/thaumaturgy") | ` At will ` |  |  |  | [Rules](#^splat-68a383d1-thaumaturgy "section:thaumaturgy") |
| [Clairvoyance](<../spells/clairvoyance.md> "dnd2024:spells/clairvoyance") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-clairvoyance "section:clairvoyance") |
| [Cure Wounds](<../spells/cure_wounds.md> "dnd2024:spells/cure_wounds") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-cure_wounds "section:cure_wounds") |
| [Flame Strike](<../spells/flame_strike.md> "dnd2024:spells/flame_strike") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-flame_strike "section:flame_strike") |
| [Geas](<../spells/geas.md> "dnd2024:spells/geas") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-geas "section:geas") |
| [True Seeing](<../spells/true_seeing.md> "dnd2024:spells/true_seeing") | ` Daily ` | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-true_seeing "section:true_seeing") |

###### Thaumaturgy

^splat-68a383d1-thaumaturgy

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@thaumaturgy/cast_level -->

```yaml
spell_minimum
```

###### Clairvoyance

^splat-68a383d1-clairvoyance

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@clairvoyance/cast_level -->

```yaml
spell_minimum
```

###### Cure wounds

^splat-68a383d1-cure_wounds

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@cure_wounds/cast_level -->

```yaml
6
```

###### Flame strike

^splat-68a383d1-flame_strike

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@flame_strike/cast_level -->

```yaml
6
```

###### Geas

^splat-68a383d1-geas

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@geas/cast_level -->

```yaml
spell_minimum
```

###### True seeing

^splat-68a383d1-true_seeing

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@true_seeing/cast_level -->

```yaml
spell_minimum
```

##### Celestial restoration

<!-- splat:yaml path=splat_rule/parameters/features/celestial_restoration -->

```yaml
economy: passive
resource:
  capacity: 1
  recovery:
  - long_rest
  cost: 1
trigger: self.died
op: schedule_revive
delay:
  days:
    dice:
      count: 1
      sides: 6
    add: 0
hit_points: maximum
canceled_by:
  spell:
    key: dnd2024:spells/dispel_evil_and_good
  target: self_remains
```

##### Poisonous spittle

<!-- splat:yaml path=splat_rule/parameters/features/poisonous_spittle -->

```yaml
economy: action
op: save
ability: constitution
dc: 16
targets:
  kind: creature
  count: 1
  range_feet: 60
  visible_to: self
failure:
- op: damage
  average: 31
  amount:
    dice:
      count: 7
      sides: 8
    add: 0
  damage_type: poison
- op: condition
  condition: blinded
  ends:
    event: turn.start
    participant: self
    occurrence: next
success:
- op: damage_fraction
  numerator: 1
  denominator: 2
  round: down
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 11 ` |
| ` history ` | ` 11 ` |
| ` religion ` | ` 11 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["poison"] ` |
| ` conditions ` | ` ["charmed", "paralyzed", "poisoned", "restrained"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["celestial", "common"] ` |
| ` understands ` | ` ["celestial", "common"] ` |

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
