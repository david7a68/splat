---
splat_schema: 2
splat_id: d4c20ad9-5f3e-5c66-9df0-6af616fc1ec9
splat_kind: creature
splat_key: dnd2024:creatures/couatl
splat_name: Couatl
splat_origin: dnd2024:reference/creature/couatl
splat_sources:
- SRD 5.2.1, pp. 277, 278, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 82, Habitat
---

# Couatl


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Celestial |
| Alignment | Lawful Good |
| Armor Class | 19 |
| Hit Points | 60 (8d8 + 24) |
| Initiative | +5 (15) |
| Speed | 30 ft., Fly 90 ft. |
| Challenge | 4 (XP 1,100; PB +2) |
| Senses | Truesight 120 ft.; Passive Perception 15 |
| Languages | All; telepathy 120 ft. |
| Resistances | Bludgeoning, Piercing, Slashing |
| Immunities | Psychic, Radiant |
| Habitat | Desert, Forest, Grassland, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 20 | +5 | +5 |
| Con | 17 | +3 | +5 |
| Int | 18 | +4 | +4 |
| Wis | 20 | +5 | +7 |
| Cha | 18 | +4 | +4 |


## Source text

<!-- SRD page 277 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Celestial, Lawful Good\
> AC 19 Initiative +5 (15)\
> HP 60 (8d8 + 24)\
> Speed 30 ft., Fly 90 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 20 +5 +5 Con 17 +3 +5\
> Int 18 +4 +4 Wis 20 +5 +7 Cha 18 +4 +4\
> Resistances Bludgeoning, Piercing, Slashing\
> Immunities Psychic, Radiant\
> Senses Truesight 120 ft.; Passive Perception 15\
> Languages All; telepathy 120 ft.\
> CR 4 (XP 1,100; PB +2)


### Traits

Shielded Mind. The couatl’s thoughts can’t be read by any means, and other creatures can communicate with it telepathically only if it allows them.

### Actions

Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 11 (1d12 + 5) Piercing damage, and the target has the Poisoned condition until the end of the couatl’s next turn.

Constrict. Strength Saving Throw: DC 15, one Medium or smaller creature the couatl can see within 5 feet. Failure: 8 (1d6 + 5) Bludgeoning damage. The target has the Grappled condition (escape DC 13), and it has the Restrained condition until the grapple ends.

Spellcasting. The couatl casts one of the following spells, requiring no spell components and using Wisdom as the spellcasting ability (spell save DC 15):

At Will: Detect Evil and Good, Detect Magic, Detect Thoughts, Shapechange (Beast or Humanoid form only, no Temporary Hit Points gained from the spell, and no Concentration or Temporary Hit Points required to maintain the spell)

<!-- SRD page 278 -->

1/Day Each: Create Food and Water, Dream, Greater Restoration, Scrying, Sleep

### Bonus Actions

Divine Aid (2/Day). The couatl casts Bless, Lesser Restoration, or Sanctuary, requiring no spell components and using the same spellcasting ability as Spellcasting.

Original layout: [[../../srd-5.2.1/pages/277|page 277]], [[../../srd-5.2.1/pages/278|page 278]].







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

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` wisdom ` |
| ` components_waived ` | ` ["verbal", "somatic", "material"] ` |
| ` save_dc ` | ` 15 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- detect_evil_and_good
- detect_magic
- detect_thoughts
- shapechange
- create_food_and_water
- dream
- greater_restoration
- scrying
- sleep
columns:
  Spell:
    path: spell
    type: reference
  Frequency:
    path: frequency
    type: symbol
  Cast level:
    path: cast_level
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
-->

| Spell | Frequency | Cast level | Overrides form types | Overrides temporary hit points gained | Overrides concentration required | Overrides temporary hit points to maintain required | Uses | Resource scope | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Detect Evil and Good](<../spells/detect_evil_and_good.md> "dnd2024:spells/detect_evil_and_good") | ` At will ` | ` Spell minimum ` |  |  |  |  |  |  |  |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` At will ` | ` Spell minimum ` |  |  |  |  |  |  |  |
| [Detect Thoughts](<../spells/detect_thoughts.md> "dnd2024:spells/detect_thoughts") | ` At will ` | ` Spell minimum ` |  |  |  |  |  |  |  |
| [Shapechange](<../spells/shapechange.md> "dnd2024:spells/shapechange") | ` At will ` | ` Spell minimum ` | ` ["beast", "humanoid"] ` | ` 0 ` | false | false |  |  |  |
| [Create Food and Water](<../spells/create_food_and_water.md> "dnd2024:spells/create_food_and_water") | ` Daily ` | ` Spell minimum ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Dream](<../spells/dream.md> "dnd2024:spells/dream") | ` Daily ` | ` Spell minimum ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Greater Restoration](<../spells/greater_restoration.md> "dnd2024:spells/greater_restoration") | ` Daily ` | ` Spell minimum ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` Daily ` | ` Spell minimum ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Sleep](<../spells/sleep.md> "dnd2024:spells/sleep") | ` Daily ` | ` Spell minimum ` |  |  |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Shielded mind

<!-- splat:values path=splat_rule/parameters/features/shielded_mind -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` thought_reading_allowed ` | ` false ` |
| ` incoming_telepathy_requires_self_permission ` | ` true ` |

##### Divine aid

<!-- splat:values path=splat_rule/parameters/features/divine_aid -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` wisdom ` |
| ` save_dc ` | ` 15 ` |
| ` components_waived ` | ` ["verbal", "somatic", "material"] ` |

###### Resource

<!-- splat:values path=splat_rule/parameters/features/divine_aid/resource -->

| Field | Value |
| --- | --- |
| ` capacity ` | ` 2 ` |
| ` cost ` | ` 1 ` |
| ` recovery ` | ` long_rest ` |
| ` scope ` | ` shared_choices ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/divine_aid/choices
key: Spell
order:
- bless
- lesser_restoration
- sanctuary
columns:
  Spell:
    path: spell
    type: reference
-->

| Spell |
| --- |
| [Bless](<../spells/bless.md> "dnd2024:spells/bless") |
| [Lesser Restoration](<../spells/lesser_restoration.md> "dnd2024:spells/lesser_restoration") |
| [Sanctuary](<../spells/sanctuary.md> "dnd2024:spells/sanctuary") |

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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 90
> ```

