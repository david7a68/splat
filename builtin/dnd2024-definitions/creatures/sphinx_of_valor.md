---
splat_schema: 2
splat_id: 9ebd4f75-7afe-5367-b0a2-cfc7a7a5309e
splat_kind: creature
splat_key: dnd2024:creatures/sphinx_of_valor
splat_name: Sphinx of Valor
splat_origin: dnd2024:reference/creature/sphinx_of_valor
splat_sources:
- SRD 5.2.1, pp. 328, https://www.dndbeyond.com/srd
---

# Sphinx of Valor


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Celestial |
| Alignment | Lawful Neutral |
| Armor Class | 17 |
| Hit Points | 199 (19d10 + 95) |
| Initiative | +12 (22) |
| Speed | 40 ft., Fly 60 ft. |
| Challenge | 17 (XP 18,000, or 20,000 in lair; PB +6) |
| Skills | Arcana +9, Perception +12, Religion +15 |
| Senses | Truesight 120 ft.; Passive Perception 22 |
| Languages | Celestial, Common |
| Resistances | Necrotic, Radiant |
| Immunities | Psychic; Charmed, Frightened |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 22 | +6 | +6 |
| Dex | 10 | +0 | +6 |
| Con | 20 | +5 | +11 |
| Int | 16 | +3 | +9 |
| Wis | 23 | +6 | +12 |
| Cha | 18 | +4 | +4 |


## Source text

<!-- SRD page 328 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Celestial, Lawful Neutral\
> AC 17 Initiative +12 (22)\
> HP 199 (19d10 + 95)\
> Speed 40 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 22 +6 +6 Dex 10 +0 +6 Con 20 +5 +11\
> Int 16 +3 +9 Wis 23 +6 +12 Cha 18 +4 +4\
> Skills Arcana +9, Perception +12, Religion +15\
> Resistances Necrotic, Radiant\
> Immunities Psychic; Charmed, Frightened\
> Senses Truesight 120 ft.; Passive Perception 22\
> Languages Celestial, Common\
> CR 17 (XP 18,000, or 20,000 in lair; PB +6)


### Traits

Inscrutable. No magic can observe the sphinx remotely or detect its thoughts without its permission. Wisdom (Insight) checks made to ascertain its intentions or sincerity are made with Disadvantage.

Legendary Resistance (3/Day, or 4/Day in Lair). If the sphinx fails a saving throw, it can choose to succeed instead.

### Actions

Multiattack. The sphinx makes two Claw attacks and uses Roar.

Claw. Melee Attack Roll: +12, reach 5 ft. Hit: 20 (4d6 + 6) Slashing damage.

Roar (3/Day). The sphinx emits a magical roar. Whenever it roars, the roar has a different effect, as detailed below (the sequence resets when it takes a Long Rest):

First Roar. Wisdom Saving Throw: DC 20, each enemy in a 500-foot Emanation originating from the sphinx. Failure: The target has the Frightened condition for 1 minute.

Second Roar. Wisdom Saving Throw: DC 20, each enemy in a 500-foot Emanation originating from the sphinx. Failure: The target has the Paralyzed condition, and it repeats the save at the end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.

Third Roar. Constitution Saving Throw: DC 20, each enemy in a 500-foot Emanation originating from the sphinx. Failure: 44 (8d10) Thunder damage, and the target has the Prone condition. Success: Half damage only.

Spellcasting. The sphinx casts one of the following spells, requiring no Material components and using Wisdom as the spellcasting ability (spell save DC 20):

At Will: Detect Evil and Good, Thaumaturgy\
1/Day Each: Detect Magic, Dispel Magic, Greater Restoration, Heroes’ Feast, Zone of Truth

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the sphinx can expend a use to take one of the following actions. The sphinx regains all expended uses at the start of each of its turns.

Arcane Prowl. The sphinx can teleport up to 30 feet to an unoccupied space it can see, and it makes one Claw attack.

Weight of Years. Constitution Saving Throw: DC 16, one creature the sphinx can see within 120 feet. Failure: The target gains 1 Exhaustion level. While the target has any Exhaustion levels, it appears 3d10 years older. Failure or Success: The sphinx can’t take this action again until the start of its next turn.

Original layout: [[../../srd-5.2.1/pages/328|page 328]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/sphinx_of_valor ` |
| ` implementation ` | ` dnd2024:creature_sphinx_of_valor ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["necrotic", "radiant"] ` |
| ` damage_immunities ` | ` ["psychic"] ` |
| ` condition_immunities ` | ` ["charmed", "frightened"] ` |
| ` passive_perception ` | ` 22 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` arcana ` | ` 9 ` |
| ` perception ` | ` 12 ` |
| ` religion ` | ` 15 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` truesight ` | ` 120 ` |

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "celestial"] ` |

#### Features

##### Legendary resistance

<!-- splat:yaml path=splat_rule/parameters/features/legendary_resistance -->

```yaml
economy: passive
trigger: saving_throw.failed
optional: true
op: saving_throw.set_result
result: success
uses_in_lair: 4
uses:
  count: 3
  recovery: long_rest
```

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 12
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 4
        sides: 6
      add: 6
    average: 20
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
| ` ability ` | ` wisdom ` |
| ` waived_components ` | ` ["material"] ` |
| ` save_dc ` | ` 20 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/profile/spells
key: Spell
order:
- detect_evil_and_good
- thaumaturgy
- detect_magic
- dispel_magic
- greater_restoration
- heroes_feast
- zone_of_truth
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
| [Detect Evil and Good](<../spells/detect_evil_and_good.md> "dnd2024:spells/detect_evil_and_good") | true |  |  |
| [Thaumaturgy](<../spells/thaumaturgy.md> "dnd2024:spells/thaumaturgy") | true |  |  |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") |  | ` 1 ` | ` Long rest ` |
| [Dispel Magic](<../spells/dispel_magic.md> "dnd2024:spells/dispel_magic") |  | ` 1 ` | ` Long rest ` |
| [Greater Restoration](<../spells/greater_restoration.md> "dnd2024:spells/greater_restoration") |  | ` 1 ` | ` Long rest ` |
| [Heroes’ Feast](<../spells/heroes_feast.md> "dnd2024:spells/heroes_feast") |  | ` 1 ` | ` Long rest ` |
| [Zone of Truth](<../spells/zone_of_truth.md> "dnd2024:spells/zone_of_truth") |  | ` 1 ` | ` Long rest ` |

##### Inscrutable

<!-- splat:yaml path=splat_rule/parameters/features/inscrutable -->

```yaml
economy: passive
magic_permissions:
  remote_observation: requires_actor_consent
  thought_detection: requires_actor_consent
disadvantage_on_checks:
  ability: wisdom
  skill: insight
  purpose:
  - intentions
  - sincerity
```

##### Roar

<!-- splat:yaml path=splat_rule/parameters/features/roar -->

```yaml
economy: action
op: sequence_by_use
magical: true
sequence_reset: long_rest
effects:
- op: saving_throw
  ability: wisdom
  dc: 20
  targets:
    kind: creature
    count: all
    area:
      shape: emanation
      origin: actor
      size_ft: 500
    allegiance: enemy
  failure:
  - op: condition.apply
    condition: frightened
    duration:
      amount: 1
      unit: minute
  success: []
- op: saving_throw
  ability: wisdom
  dc: 20
  targets:
    kind: creature
    count: all
    area:
      shape: emanation
      origin: actor
      size_ft: 500
    allegiance: enemy
  failure:
  - op: condition.apply
    condition: paralyzed
    duration:
      amount: 1
      unit: minute
    repeat_save:
      event: target.turn_end
      ability: wisdom
      dc: 20
      success: effect.end
      automatic_success_after:
        amount: 1
        unit: minute
  success: []
- op: saving_throw
  ability: constitution
  dc: 20
  targets:
    kind: creature
    count: all
    area:
      shape: emanation
      origin: actor
      size_ft: 500
    allegiance: enemy
  failure:
  - op: damage
    components:
    - type: thunder
      amount:
        dice:
          count: 8
          sides: 10
        add: 0
      average: 44
  - op: condition.apply
    condition: prone
  success:
  - op: damage
    from: failure_damage
    multiplier: 0.5
    round: down
uses:
  count: 3
  recovery: long_rest
```

##### Arcane prowl

<!-- splat:yaml path=splat_rule/parameters/features/arcane_prowl -->

```yaml
economy: legendary_action
op: sequence
steps:
- op: teleport
  optional: true
  maximum_ft: 30
  destination:
    unoccupied: true
    visible_to_actor: true
- op: feature.use
  feature: claw
legendary_cost: 1
```

##### Weight of years

<!-- splat:yaml path=splat_rule/parameters/features/weight_of_years -->

```yaml
economy: legendary_action
op: saving_throw
ability: constitution
dc: 16
targets:
  kind: creature
  count: 1
  range_ft: 120
  visible_to_actor: true
failure:
- op: exhaustion.add
  levels: 1
- op: appearance.age
  years:
    dice:
      count: 3
      sides: 10
    add: 0
  duration:
    until: target.has_no_exhaustion
success: []
lockout_until: actor.next_turn_start
legendary_cost: 1
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` multiattack ` |

###### Sequence

<!-- splat:table
path: splat_rule/parameters/features/multiattack/sequence
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Feature:
    path: feature
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Feature | Count |
| --- | --- | --- |
| ` row_1 ` | ` Claw ` | ` 2 ` |
| ` row_2 ` | ` Roar ` | ` 1 ` |

#### Legendary actions

<!-- splat:values path=splat_rule/parameters/legendary_actions -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 3 ` |
| ` uses_in_lair ` | ` 4 ` |
| ` trigger ` | ` another_creature.turn_end_immediately_after ` |
| ` cost_per_action ` | ` 1 ` |
| ` recover_all ` | ` actor.turn_start ` |
| ` maximum_per_trigger ` | ` 1 ` |
| ` requires_actions_available ` | ` true ` |
| ` excluded_conditions ` | ` ["incapacitated"] ` |

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

