---
splat_schema: 2
splat_id: e5d39c22-fefe-5740-9dd2-904b59632f06
splat_kind: creature
splat_key: dnd2024:creatures/incubus
splat_name: Incubus
splat_origin: dnd2024:reference/creature/incubus
splat_sources:
- SRD 5.2.1, pp. 301, https://www.dndbeyond.com/srd
---

# Incubus


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fiend |
| Alignment | Neutral Evil |
| Armor Class | 15 |
| Hit Points | 66 (12d8 + 12) |
| Initiative | +3 (13) |
| Speed | 30 ft., Fly 60 ft. |
| Challenge | 4 (XP 1,100; PB +2) |
| Skills | Deception +9, Insight +5, Perception +5, Persuasion +9, Stealth +7 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | Abyssal, Common, Infernal; telepathy 60 ft. |
| Resistances | Cold, Fire, Poison, Psychic |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 8 | -1 | -1 |
| Dex | 17 | +3 | +3 |
| Con | 13 | +1 | +1 |
| Int | 15 | +2 | +2 |
| Wis | 12 | +1 | +1 |
| Cha | 20 | +5 | +5 |


## Source text

<!-- SRD page 301 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fiend, Neutral Evil\
> AC 15  Initiative +3 (13)\
> HP 66 (12d8 + 12)\
> Speed 30 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 8 −1 −1 Dex 17 +3 +3 Con 13 +1 +1\
> Int 15 +2 +2 Wis 12 +1 +1 Cha 20 +5 +5\
> Skills Deception +9, Insight +5, Perception +5, Persuasion +9, Stealth +7\
> Resistances Cold, Fire, Poison, Psychic\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages Abyssal, Common, Infernal; telepathy 60 ft.\
> CR 4 (XP 1,100; PB +2)


### Traits

Succubus Form. When the incubus finishes a Long Rest, it can shape-shift into a Succubus, using that stat block instead of this one. Any equipment it is wearing or carrying isn’t transformed.

### Actions

Multiattack. The incubus makes two Restless Touch attacks.

Restless Touch. Melee Attack Roll: +7, reach 5 ft. Hit: 15 (3d6 + 5) Psychic damage, and the target is cursed for 24 hours or until the incubus dies. Until the curse ends, the target gains no benefit from finishing Short Rests.

Spellcasting. The incubus casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 15):

At Will: Disguise Self, Etherealness\
1/Day Each: Dream, Hypnotic Pattern

### Bonus Actions

Nightmare (Recharge 6). Wisdom Saving Throw: DC 15, one creature the incubus can see within 60 feet. Failure: If the target has 20 Hit Points or fewer, it has the Unconscious condition for 1 hour, until it takes damage, or until a creature within 5 feet of it takes an action to wake it. Otherwise, the target takes 18 (4d8) Psychic damage.

Original layout: [[../../srd-5.2.1/pages/301|page 301]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/incubus ` |
| ` implementation ` | ` dnd2024:creature_incubus ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold", "fire", "poison", "psychic"] ` |
| ` passive_perception ` | ` 15 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` deception ` | ` 9 ` |
| ` insight ` | ` 5 ` |
| ` perception ` | ` 5 ` |
| ` persuasion ` | ` 9 ` |
| ` stealth ` | ` 7 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "infernal", "abyssal"] ` |

#### Telepathy

<!-- splat:values path=splat_rule/parameters/telepathy -->

| Field | Value |
| --- | --- |
| ` range_ft ` | ` 60 ` |
| ` two_way ` | ` true ` |

#### Features

##### Restless touch

<!-- splat:yaml path=splat_rule/parameters/features/restless_touch -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: psychic
    amount:
      dice:
        count: 3
        sides: 6
      add: 5
    average: 15
- op: curse.apply
  duration:
    maximum:
      amount: 24
      unit: hour
    until: actor.died
  rules:
    short_rest_benefits: false
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` spellcasting ` |

###### Profile

<!-- splat:values path=splat_rule/parameters/features/spellcasting/profile -->

| Field | Value |
| --- | --- |
| ` ability ` | ` charisma ` |
| ` waived_components ` | ` ["material"] ` |
| ` save_dc ` | ` 15 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- disguise_self
- etherealness
- dream
- hypnotic_pattern
columns:
  Spell:
    path: spell
    type: reference
  Uses unlimited:
    path: uses/unlimited
    type: boolean
    optional: true
  Uses count:
    path: uses/count
    type: integer
    optional: true
  Uses recovery:
    path: uses/recovery
    type: symbol
    optional: true
-->

| Spell | Uses unlimited | Uses count | Uses recovery |
| --- | --- | --- | --- |
| [Disguise Self](<../spells/disguise_self.md> "dnd2024:spells/disguise_self") | true |  |  |
| [Etherealness](<../spells/etherealness.md> "dnd2024:spells/etherealness") | true |  |  |
| [Dream](<../spells/dream.md> "dnd2024:spells/dream") |  | ` 1 ` | ` Long rest ` |
| [Hypnotic Pattern](<../spells/hypnotic_pattern.md> "dnd2024:spells/hypnotic_pattern") |  | ` 1 ` | ` Long rest ` |

##### Succubus form

<!-- splat:yaml path=splat_rule/parameters/features/succubus_form -->

```yaml
economy: passive
trigger: actor.long_rest_finished
optional: true
op: stat_block.replace
definition:
  key: dnd2024:creatures/succubus
equipment_transformed: false
```

##### Nightmare

<!-- splat:yaml path=splat_rule/parameters/features/nightmare -->

```yaml
economy: bonus_action
op: saving_throw
ability: wisdom
dc: 15
targets:
  kind: creature
  count: 1
  range_ft: 60
  visible_to_actor: true
failure:
- op: branch
  if:
    target_hp_at_most: 20
  then:
  - op: condition.apply
    condition: unconscious
    duration:
      maximum:
        amount: 1
        unit: hour
      until_any:
      - target.takes_damage
      - creature_within_5_ft.uses_action_to_wake
  else:
  - op: damage
    components:
    - type: psychic
      amount:
        dice:
          count: 4
          sides: 8
        add: 0
      average: 18
success: []
recharge:
  die_sides: 6
  success_min: 6
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: restless_touch
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spellcasting defaults

<!-- splat:yaml path=splat_rule/parameters/spellcasting_defaults -->

```yaml
cast_level: lowest_possible_unless_overridden
upcast_allowed: only_explicit_cast_level_overrides
required_material_components_available: true
casting_time_at_least_1_minute:
  uses_normal_time: true
  magic_action_each_turn: true
  concentration_required: true
other_components: spell_definition_unless_waived
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 60
> ```

