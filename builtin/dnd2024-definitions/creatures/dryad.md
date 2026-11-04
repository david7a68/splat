---
splat_schema: 2
splat_id: 77df49d6-e151-5948-9bab-834533875dea
splat_kind: creature
splat_key: dnd2024:creatures/dryad
splat_name: Dryad
splat_origin: dnd2024:reference/creature/dryad
splat_sources:
- SRD 5.2.1, pp. 282, https://www.dndbeyond.com/srd
---

# Dryad


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Fey |
| Alignment | Neutral |
| Armor Class | 16 |
| Hit Points | 22 (5d8) |
| Initiative | +1 (11) |
| Speed | 30 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Perception +4, Stealth +5 |
| Senses | Darkvision 60 ft.; Passive Perception 14 |
| Languages | Elvish, Sylvan |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 10 | +0 | +0 |
| Dex | 12 | +1 | +1 |
| Con | 11 | +0 | +0 |
| Int | 14 | +2 | +2 |
| Wis | 15 | +2 | +2 |
| Cha | 18 | +4 | +4 |


## Source text

<!-- SRD page 282 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Fey, Neutral\
> AC 16 Initiative +1 (11)\
> HP 22 (5d8)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 10 +0 +0 Dex 12 +1 +1 Con 11 +0 +0\
> Int 14 +2 +2 Wis 15 +2 +2 Cha 18 +4 +4\
> Skills Perception +4, Stealth +5\
> Senses Darkvision 60 ft.; Passive Perception 14\
> Languages Elvish, Sylvan\
> CR 1 (XP 200; PB +2)


### Traits

Magic Resistance. The dryad has Advantage on saving throws against spells and other magical effects.

Speak with Beasts and Plants. The dryad can communicate with Beasts and Plants as if they shared a language.

### Actions

Multiattack. The dryad makes one Vine Lash or Thorn Burst attack, and it can use Spellcasting to cast Charm Monster.

Vine Lash. Melee Attack Roll: +6, reach 10 ft. Hit: 8 (1d8 + 4) Slashing damage.

Thorn Burst. Ranged Attack Roll: +6, range 60 ft. Hit: 7 (1d6 + 4) Piercing damage.

Spellcasting. The dryad casts one of the following spells, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 14):

At Will: Animal Friendship, Charm Monster (lasts 24 hours; ends early if the dryad casts the spell again), Druidcraft

1/Day Each: Entangle, Pass without Trace

### Bonus Actions

Tree Stride. If within 5 feet of a Large or bigger tree, the dryad teleports to an unoccupied space within 5 feet of a second Large or bigger tree that is within 60 feet of the previous tree.

Original layout: [[../../srd-5.2.1/pages/282|page 282]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/dryad ` |
| ` implementation ` | ` dnd2024:creature_dryad ` |

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

##### Vine lash

<!-- splat:yaml path=splat_rule/parameters/features/vine_lash -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 8
  amount:
    dice:
      count: 1
      sides: 8
    add: 4
  damage_type: slashing
reach_feet: 10
```

##### Thorn burst

<!-- splat:yaml path=splat_rule/parameters/features/thorn_burst -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 6
targets:
  count: 1
hit:
- op: damage
  average: 7
  amount:
    dice:
      count: 1
      sides: 6
    add: 4
  damage_type: piercing
range_feet: 60
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: choose
  choices:
  - op: use_feature
    feature: vine_lash
    count: 1
  - op: use_feature
    feature: thorn_burst
    count: 1
- op: optional
  effect:
    op: cast_spell
    spell:
      key: dnd2024:spells/charm_monster
    via: spellcasting
    cast_level: spell_minimum
```

##### Spellcasting

<!-- splat:values path=splat_rule/parameters/features/spellcasting -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` cast_spell_choice ` |
| ` ability ` | ` charisma ` |
| ` components_waived ` | ` ["material"] ` |
| ` save_dc ` | ` 14 ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/features/spellcasting/spells
key: Spell
order:
- animal_friendship
- charm_monster
- druidcraft
- entangle
- pass_without_trace
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
  Overrides duration hours:
    path: overrides/duration/hours
    type: integer
    optional: true
  Overrides ends on:
    path: overrides/ends_on
    type: symbol
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

| Spell | Frequency | Cast level | Overrides duration hours | Overrides ends on | Uses | Resource scope | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [Animal Friendship](<../spells/animal_friendship.md> "dnd2024:spells/animal_friendship") | ` At will ` | ` Spell minimum ` |  |  |  |  |  |
| [Charm Monster](<../spells/charm_monster.md> "dnd2024:spells/charm_monster") | ` At will ` | ` Spell minimum ` | ` 24 ` | ` Self casts same spell again ` |  |  |  |
| [Druidcraft](<../spells/druidcraft.md> "dnd2024:spells/druidcraft") | ` At will ` | ` Spell minimum ` |  |  |  |  |  |
| [Entangle](<../spells/entangle.md> "dnd2024:spells/entangle") | ` Daily ` | ` Spell minimum ` |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |
| [Pass without Trace](<../spells/pass_without_trace.md> "dnd2024:spells/pass_without_trace") | ` Daily ` | ` Spell minimum ` |  |  | ` 1 ` | ` Per spell ` | ` Long rest ` |

##### Speak with beasts and plants

<!-- splat:yaml path=splat_rule/parameters/features/speak_with_beasts_and_plants -->

```yaml
economy: passive
communication:
  as_shared_language: true
  creature_types:
  - beast
  - plant
```

##### Tree stride

<!-- splat:yaml path=splat_rule/parameters/features/tree_stride -->

```yaml
economy: bonus_action
op: teleport
requires:
  tree_1_size_min: large
  tree_1_distance_max_feet: 5
destination:
  unoccupied: true
  tree_2_size_min: large
  tree_2_distance_max_feet: 5
  tree_2_different_from_tree_1: true
  tree_2_distance_from_tree_1_max_feet: 60
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
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
| ` speaks ` | ` ["elvish", "sylvan"] ` |
| ` understands ` | ` ["elvish", "sylvan"] ` |

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
