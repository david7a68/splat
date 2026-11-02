---
splat_schema: 1
splat_id: b9d9ae32-e7fc-5ddc-a4c2-14c1d4f47ee0
splat_kind: creature
splat_key: dnd2024:creatures/pirate_captain
splat_name: Pirate Captain
splat_origin:
  key: dnd2024:reference/creature/pirate_captain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 314
  url: https://www.dndbeyond.com/srd
---

# Pirate Captain

## Source text

<!-- SRD page 314 -->

Medium or Small Humanoid, Neutral\
AC 17  Initiative +7 (17)\
HP 84 (13d8 + 26)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +3 Dex 18 +4 +7 Con 14 +2 +2\
Int 10 +0 +0 Wis 14 +2 +5 Cha 17 +3 +6\
Skills Acrobatics +7, Perception +5\
Gear Pistol, Rapier\
Senses Passive Perception 15\
Languages Common plus one other language\
CR 6 (XP 2,300; PB +3)

### Actions

Multiattack. The pirate makes three attacks, using Rapier or Pistol in any combination.

Rapier. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 + 4) Piercing damage, and the pirate has Advantage on the next attack roll it makes before the end of this turn.

Pistol. Ranged Attack Roll: +7, range 30/90 ft. Hit: 15 (2d10 + 4) Piercing damage.

### Bonus Actions

Captain’s Charm. Wisdom Saving Throw: DC 14, one creature the pirate can see within 30 feet. Failure: The target has the Charmed condition until the start of the pirate’s next turn.

### Reactions

Riposte. Trigger: The pirate is hit by a melee attack roll while holding a weapon. Response: The pirate adds 3 to its AC against that attack, possibly causing it to miss. On a miss, the pirate makes one Rapier attack against the triggering creature if within range.

Original layout: [[../../srd-5.2.1/pages/314|page 314]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Acrobatics +7, Perception +5 ` |
| ` gear_text ` | ` Pistol, Rapier ` |
| ` senses_text ` | ` Passive Perception 15 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 6 (XP 2,300; PB +3) ` |
| ` hit_points_text ` | ` 84 (13d8 + 26) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 3 ` |
| ` Dexterity ` | ` 18 ` | ` 7 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 14 ` | ` 5 ` |
| ` Charisma ` | ` 17 ` | ` 6 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 84 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/pirate_captain ` |
| ` implementation ` | ` dnd2024:creature_pirate_captain ` |

### Parameters

#### Features

##### Rapier

<!-- splat:yaml path=splat_rule/parameters/features/rapier -->

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
  average: 13
  amount:
    dice:
      count: 2
      sides: 8
    add: 4
  damage_type: piercing
- op: advantage
  participant: self
  roll: attack
  count: 1
  ends:
    event: turn.end
    participant: self
    occurrence: current
reach_feet: 5
```

##### Pistol

<!-- splat:yaml path=splat_rule/parameters/features/pistol -->

```yaml
economy: action
op: attack
attack_modes:
- ranged
attack_bonus: 7
targets:
  count: 1
hit:
- op: damage
  average: 15
  amount:
    dice:
      count: 2
      sides: 10
    add: 4
  damage_type: piercing
range_feet: 30
long_range_feet: 90
```

##### Multiattack

<!-- splat:values path=splat_rule/parameters/features/multiattack -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` op ` | ` repeat_choice ` |
| ` count ` | ` 3 ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/multiattack/choices
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
-->

| Key | Feature |
| --- | --- |
| ` row_1 ` | ` Rapier ` |
| ` row_2 ` | ` Pistol ` |

##### Captain s charm

<!-- splat:yaml path=splat_rule/parameters/features/captain_s_charm -->

```yaml
economy: bonus_action
op: save
ability: wisdom
dc: 14
targets:
  kind: creature
  count: 1
  range_feet: 30
  visible_to: self
failure:
- op: condition
  condition: charmed
  ends:
    event: turn.start
    participant: self
    occurrence: next
  source: self
success: []
```

##### Riposte

<!-- splat:yaml path=splat_rule/parameters/features/riposte -->

```yaml
economy: reaction
trigger: melee_attack.hits_self
requires:
  holding_weapon: true
effects:
- op: modify_ac
  add: 3
  scope: triggering_attack
  reevaluate_hit: true
- op: if
  test:
    triggering_attack_misses_after_ac_bonus: true
    attacker_within_rapier_range: true
  then:
  - op: use_feature
    feature: rapier
    count: 1
    target: triggering_attacker
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` acrobatics ` | ` 7 ` |
| ` perception ` | ` 5 ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Item name:
    path: item_name
    type: string
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item name | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Pistol ` | ` 1 ` |
| ` row_2 ` | ` Rapier ` | ` 1 ` |

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
| ` count ` | ` 1 ` |
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
