---
splat_schema: 2
splat_id: 0f3a54cb-d789-586c-8fa8-eb65317e6314
splat_kind: creature
splat_key: dnd2024:creatures/deva
splat_name: Deva
splat_origin: dnd2024:reference/creature/deva
splat_sources:
- SRD 5.2.1, pp. 279, https://www.dndbeyond.com/srd
---

# Deva


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Celestial (Angel) |
| Alignment | Lawful Good |
| Armor Class | 17 |
| Hit Points | 229 (27d8 + 108) |
| Initiative | +4 (14) |
| Speed | 30 ft., Fly 90 ft. (hover) |
| Challenge | 10 (XP 5,900; PB +4) |
| Skills | Insight +9, Perception +9 |
| Senses | Darkvision 120 ft.; Passive Perception 19 |
| Languages | All; telepathy 120 ft. |
| Resistances | Radiant |
| Immunities | Charmed, Exhaustion, Frightened |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 18 | +4 | +4 |
| Con | 18 | +4 | +4 |
| Int | 17 | +3 | +3 |
| Wis | 20 | +5 | +9 |
| Cha | 20 | +5 | +9 |


## Source text

<!-- SRD page 279 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Celestial (Angel), Lawful Good\
> AC 17  Initiative +4 (14)\
> HP 229 (27d8 + 108)\
> Speed 30 ft., Fly 90 ft. (hover)\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 18 +4 +4 Con 18 +4 +4\
> Int 17 +3 +3 Wis 20 +5 +9 Cha 20 +5 +9\
> Skills Insight +9, Perception +9\
> Resistances Radiant\
> Immunities Charmed, Exhaustion, Frightened\
> Senses Darkvision 120 ft.; Passive Perception 19\
> Languages All; telepathy 120 ft.\
> CR 10 (XP 5,900; PB +4)


### Traits

Exalted Restoration. If the deva dies outside Mount Celestia, its body disappears, and it gains a new body instantly, reviving with all its Hit Points somewhere in Mount Celestia.

Magic Resistance. The deva has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The deva makes two Holy Mace attacks.

Holy Mace. Melee Attack Roll: +8, reach 5 ft. Hit: 7 (1d6 + 4) Bludgeoning damage plus 18 (4d8) Radiant damage.

Spellcasting. The deva casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 17):

At Will: Detect Evil and Good, Shapechange (Beast or Humanoid form only, no Temporary Hit Points gained from the spell, and no Concentration or Temporary Hit Points required to maintain the spell)

1/Day Each: Commune, Raise Dead

### Bonus Actions

Divine Aid (2/Day). The deva casts Cure Wounds, Lesser Restoration, or Remove Curse, using the same spellcasting ability as Spellcasting.

Original layout: [[../../srd-5.2.1/pages/279|page 279]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/deva ` |
| ` implementation ` | ` dnd2024:creature_deva ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["radiant"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened"] ` |

#### Features

##### Exalted restoration

<!-- splat:yaml path=splat_rule/parameters/features/exalted_restoration -->

```yaml
economy: trait
operation: passive_rule
trigger: self.dies
unless:
  plane: mount_celestia
remove_old_body: true
new_body:
  delay_seconds: 0
  plane: mount_celestia
  hp: maximum
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: holy_mace
  count: 2
```

##### Holy mace

<!-- splat:values path=splat_rule/parameters/features/holy_mace -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/holy_mace/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 8
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/holy_mace/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 7 ` | ` 1 ` | ` 6 ` | ` 4 ` | ` Bludgeoning ` |
| ` row_2 ` | ` 18 ` | ` 4 ` | ` 8 ` | ` 0 ` | ` Radiant ` |

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` spellcasting ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 17 ` |
| ` material_components_available ` | ` true ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/options
key: Spell
order:
- detect_evil_and_good
- shapechange
- commune
- raise_dead
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: usage/kind
    type: symbol
  Overrides allowed types:
    path: overrides/allowed_types
    type: yaml
    optional: true
  Overrides gain temporary hp:
    path: overrides/gain_temporary_hp
    type: boolean
    optional: true
  Overrides maintain requires concentration:
    path: overrides/maintain_requires_concentration
    type: boolean
    optional: true
  Overrides maintain requires temporary hp:
    path: overrides/maintain_requires_temporary_hp
    type: boolean
    optional: true
  Uses:
    path: usage/uses
    type: integer
    optional: true
  Use pool:
    path: usage/pool
    type: symbol
    optional: true
  Recovery:
    path: usage/recovery
    type: symbol
    optional: true
-->

| Spell | Frequency | Overrides allowed types | Overrides gain temporary hp | Overrides maintain requires concentration | Overrides maintain requires temporary hp | Uses | Use pool | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Detect Evil and Good](<../spells/detect_evil_and_good.md> "dnd2024:spells/detect_evil_and_good") | ` At will ` |  |  |  |  |  |  |  |
| [Shapechange](<../spells/shapechange.md> "dnd2024:spells/shapechange") | ` At will ` | ` ["beast", "humanoid"] ` | false | false | false |  |  |  |
| [Commune](<../spells/commune.md> "dnd2024:spells/commune") | ` Daily ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Raise Dead](<../spells/raise_dead.md> "dnd2024:spells/raise_dead") | ` Daily ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Divine aid

<!-- splat:yaml path=splat_rule/parameters/features/divine_aid -->

```yaml
economy: bonus_action
resource:
  kind: daily
  uses: 2
  recovery: long_rest
operation: spell_choice
options:
- operation: cast_spell
  spell:
    key: dnd2024:spells/cure_wounds
- operation: cast_spell
  spell:
    key: dnd2024:spells/lesser_restoration
- operation: cast_spell
  spell:
    key: dnd2024:spells/remove_curse
ability: charisma
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` insight ` | ` 9 ` |
| ` perception ` | ` 9 ` |

#### Damage immunities

<!-- splat:yaml path=splat_rule/parameters/damage_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 120 ` |
| ` passive_perception ` | ` 19 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: all
understands: []
telepathy_range_ft: 120
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 90
> ```

