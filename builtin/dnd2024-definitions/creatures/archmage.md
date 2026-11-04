---
splat_schema: 2
splat_id: 3480ce3e-ec81-597d-9ecf-a57846062274
splat_kind: creature
splat_key: dnd2024:creatures/archmage
splat_name: Archmage
splat_origin: dnd2024:reference/creature/archmage
splat_sources:
- SRD 5.2.1, pp. 305, https://www.dndbeyond.com/srd
---

# Archmage


## Stat block

%% splat reward=printed %%

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid (Wizard) |
| Alignment | Neutral |
| Armor Class | 17 |
| Hit Points | 170 (31d8 + 31) |
| Initiative | +7 (17) |
| Speed | 30 ft. |
| Challenge | 12 (XP 8,000; PB +4) |
| Skills | Arcana +13, History +9, Perception +6 |
| Senses | Passive Perception 16 |
| Languages | Common plus five other languages |
| Gear | Wand |
| Immunities | Psychic; Charmed (with Mind Blank) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 10 | +0 | +0 |
| Dex | 14 | +2 | +2 |
| Con | 12 | +1 | +1 |
| Int | 20 | +5 | +9 |
| Wis | 15 | +2 | +6 |
| Cha | 16 | +3 | +3 |


## Source text

<!-- SRD page 305 -->

Medium or Small Humanoid (Wizard), Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 17 Initiative +7 (17)\
> HP 170 (31d8 + 31)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 10 +0 +0 Dex 14 +2 +2 Con 12 +1 +1\
> Int 20 +5 +9 Wis 15 +2 +6 Cha 16 +3 +3\
> Skills Arcana +13, History +9, Perception +6\
> Immunities Psychic; Charmed (with Mind Blank)\
> Gear Wand\
> Senses Passive Perception 16\
> Languages Common plus five other languages\
> CR 12 (XP 8,000; PB +4)


### Traits

Magic Resistance. The archmage has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The archmage makes four Arcane Burst attacks.

Arcane Burst. Melee or Ranged Attack Roll: +9, reach 5 ft. or range 150 ft. Hit: 27 (4d10 + 5) Force damage.

Spellcasting. The archmage casts one of the following spells, using Intelligence as the spellcasting ability (spell save DC 17):

At Will: Detect Magic, Detect Thoughts, Disguise Self, Invisibility, Light, Mage Armor (included in AC), Mage Hand, Prestidigitation

2/Day Each: Fly, Lightning Bolt (level 7 version)\
1/Day Each: Cone of Cold (level 9 version), Mind Blank (cast before combat), Scrying, Teleport

### Bonus Actions

Misty Step (3/Day). The mage casts Misty Step, using the same spellcasting ability as Spellcasting.

### Reactions

Protective Magic (3/Day). The archmage casts Counterspell or Shield in response to the spell’s trigger, using the same spellcasting ability as Spellcasting.

Original layout: [[../../srd-5.2.1/pages/305|page 305]].







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

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` intelligence ` |
| ` save_dc ` | ` 17 ` |
| ` required_material_components_available ` | ` true ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- detect_magic
- detect_thoughts
- disguise_self
- invisibility
- light
- mage_armor
- mage_hand
- prestidigitation
- fly
- lightning_bolt
- cone_of_cold
- mind_blank
- scrying
- teleport
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: frequency
    type: symbol
  Precast:
    path: precast
    type: boolean
    optional: true
  Included in supplied ac:
    path: included_in_supplied_ac
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

| Spell | Frequency | Precast | Included in supplied ac | Uses | Resource scope | Recovery | Details |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` |  |  |  |  |  | [Rules](#^splat-68a383d1-detect_magic "section:detect_magic") |
| [Detect Thoughts](<../spells/detect_thoughts.md> "dnd2024:spells/detect_thoughts") | ` At will ` |  |  |  |  |  | [Rules](#^splat-68a383d1-detect_thoughts "section:detect_thoughts") |
| [Disguise Self](<../spells/disguise_self.md> "dnd2024:spells/disguise_self") | ` At will ` |  |  |  |  |  | [Rules](#^splat-68a383d1-disguise_self "section:disguise_self") |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") | ` At will ` |  |  |  |  |  | [Rules](#^splat-68a383d1-invisibility "section:invisibility") |
| [Light](<../spells/light.md> "dnd2024:spells/light") | ` At will ` |  |  |  |  |  | [Rules](#^splat-68a383d1-light "section:light") |
| [Mage Armor](<../spells/mage_armor.md> "dnd2024:spells/mage_armor") | ` At will ` | true | true |  |  |  | [Rules](#^splat-68a383d1-mage_armor "section:mage_armor") |
| [Mage Hand](<../spells/mage_hand.md> "dnd2024:spells/mage_hand") | ` At will ` |  |  |  |  |  | [Rules](#^splat-68a383d1-mage_hand "section:mage_hand") |
| [Prestidigitation](<../spells/prestidigitation.md> "dnd2024:spells/prestidigitation") | ` At will ` |  |  |  |  |  | [Rules](#^splat-68a383d1-prestidigitation "section:prestidigitation") |
| [Fly](<../spells/fly.md> "dnd2024:spells/fly") | ` Daily ` |  |  | ` 2 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-fly "section:fly") |
| [Lightning Bolt](<../spells/lightning_bolt.md> "dnd2024:spells/lightning_bolt") | ` Daily ` |  |  | ` 2 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-lightning_bolt "section:lightning_bolt") |
| [Cone of Cold](<../spells/cone_of_cold.md> "dnd2024:spells/cone_of_cold") | ` Daily ` |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-cone_of_cold "section:cone_of_cold") |
| [Mind Blank](<../spells/mind_blank.md> "dnd2024:spells/mind_blank") | ` Daily ` | true |  | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-mind_blank "section:mind_blank") |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` Daily ` |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-scrying "section:scrying") |
| [Teleport](<../spells/teleport.md> "dnd2024:spells/teleport") | ` Daily ` |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` | [Rules](#^splat-68a383d1-teleport "section:teleport") |

###### Detect magic

^splat-68a383d1-detect_magic

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@detect_magic/cast_level -->

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

###### Disguise self

^splat-68a383d1-disguise_self

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@disguise_self/cast_level -->

```yaml
spell_minimum
```

###### Invisibility

^splat-68a383d1-invisibility

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@invisibility/cast_level -->

```yaml
spell_minimum
```

###### Light

^splat-68a383d1-light

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@light/cast_level -->

```yaml
spell_minimum
```

###### Mage armor

^splat-68a383d1-mage_armor

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@mage_armor/cast_level -->

```yaml
spell_minimum
```

###### Mage hand

^splat-68a383d1-mage_hand

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@mage_hand/cast_level -->

```yaml
spell_minimum
```

###### Prestidigitation

^splat-68a383d1-prestidigitation

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@prestidigitation/cast_level -->

```yaml
spell_minimum
```

###### Fly

^splat-68a383d1-fly

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@fly/cast_level -->

```yaml
spell_minimum
```

###### Lightning bolt

^splat-68a383d1-lightning_bolt

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@lightning_bolt/cast_level -->

```yaml
7
```

###### Cone of cold

^splat-68a383d1-cone_of_cold

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@cone_of_cold/cast_level -->

```yaml
9
```

###### Mind blank

^splat-68a383d1-mind_blank

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@mind_blank/cast_level -->

```yaml
spell_minimum
```

###### Scrying

^splat-68a383d1-scrying

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@scrying/cast_level -->

```yaml
spell_minimum
```

###### Teleport

^splat-68a383d1-teleport

###### Cast level

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/spells/@teleport/cast_level -->

```yaml
spell_minimum
```

###### Components waived

<!-- splat:yaml path=splat_rule/parameters/features/spellcasting/components_waived -->

```yaml
[]
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

<!-- splat:values path=splat_rule/parameters/features/protective_magic -->

| Field | Value |
| --- | --- |
| ` economy ` | ` reaction ` |
| ` op ` | ` cast_spell_choice ` |
| ` trigger ` | ` chosen_spell.trigger ` |
| ` ability ` | ` intelligence ` |
| ` save_dc ` | ` 17 ` |

###### Resource

<!-- splat:values path=splat_rule/parameters/features/protective_magic/resource -->

| Field | Value |
| --- | --- |
| ` capacity ` | ` 3 ` |
| ` recovery ` | ` long_rest ` |
| ` cost ` | ` 1 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/protective_magic/choices
key: Spell
order:
- counterspell
- shield
columns:
  Spell:
    path: spell
    type: reference
-->

| Spell |
| --- |
| [Counterspell](<../spells/counterspell.md> "dnd2024:spells/counterspell") |
| [Shield](<../spells/shield.md> "dnd2024:spells/shield") |

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
