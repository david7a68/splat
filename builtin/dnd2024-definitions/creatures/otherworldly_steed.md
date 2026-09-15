---
splat_schema: 1
splat_id: b0baef5c-6b59-58df-8e5d-5c7da31d6d49
splat_kind: creature
splat_key: dnd2024:creatures/otherworldly_steed
splat_name: Otherworldly Steed
splat_origin:
  key: dnd2024:reference/creature/otherworldly_steed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 131
  url: https://www.dndbeyond.com/srd
---

# Otherworldly Steed

## Source text

<!-- SRD page 131 -->

Large Celestial, Fey, or Fiend (Your Choice), Neutral\
AC 10 + 1 per spell level\
HP 5 + 10 per spell level (the steed has a number of Hit\
Dice [d10s] equal to the spell’s level)\
Speed 60 ft., Fly 60 ft. (requires level 4+ spell)\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 12 +1 +1 Con14 +2 +2\
Int 6 −2 −2 Wis 12 +1 +1 Cha 8 −1 −1\
Senses Passive Perception 11\
Languages Telepathy 1 mile (works only with you)\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Life Bond. When you regain Hit Points from a level 1+\
spell, the steed regains the same number of Hit Points if\
you’re within 5 feet of it.\

### Actions

Otherworldly Slam. Melee Attack Roll: Bonus equals\
your spell attack modifier, reach 5 ft. Hit: 1d8 plus the\
spell’s level of Radiant (Celestial), Psychic (Fey), or Ne-\
crotic (Fiend) damage.\

### Bonus Actions

Fell Glare (Fiend Only; Recharges after a Long Rest).\
Wisdom Saving Throw: DC equals your spell save DC,\
one creature within 60 feet the steed can see. Failure:\
The target has the Frightened condition until the end of\
your next turn.\
Fey Step (Fey Only; Recharges after a Long Rest). The\
steed teleports, along with its rider, to an unoccupied\
space of your choice up to 60 feet away from itself.\
Healing Touch (Celestial Only; Recharges after a Long\
Rest). One creature within 5 feet of the steed regains a\
number of Hit Points equal to 2d8 plus the spell’s level.\

Original layout: [[../../srd-5.2.1/pages/131|page 131]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` celestial ` |
| ` alignment ` | ` Fey, or Fiend (Your Choice), Neutral ` |
| ` movement_text ` | ` 60 ft., Fly 60 ft. (requires level 4+ spell) ` |
| ` senses_text ` | ` Passive Perception 11 ` |
| ` languages_text ` | ` Telepathy 1 mile (works only with you) ` |
| ` cr_text ` | ` None (XP 0; PB equals your Proficiency Bonus) ` |
| ` hit_points_text ` | ` 5 + 10 per spell level (the steed has a number of Hit Dice [d10s] equal to the spell’s level) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 6 ` | ` -2 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 8 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 10 ` |  |
| ` speed ` | ` 60 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/otherworldly_steed ` |
| ` implementation ` | ` dnd2024:creature_otherworldly_steed ` |

### Parameters

#### Features

##### Life bond

<!-- splat:yaml path=splat_rule/parameters/features/life_bond -->

```yaml
economy: passive
trigger: summoner.hit_points_regained
requires:
  source_spell_level_min: 1
  summoner_distance_max_feet: 5
op: heal
amount:
  event_value: summoner_hp_regained
```

##### Otherworldly slam

<!-- splat:yaml path=splat_rule/parameters/features/otherworldly_slam -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus:
  input: summoner_spell_attack_modifier
reach_feet: 5
targets:
  count: 1
hit:
- op: damage
  damage_type:
    by_self_type:
      celestial: radiant
      fey: psychic
      fiend: necrotic
  amount:
    add:
    - dice:
        count: 1
        sides: 8
      add: 0
    - input: spell_level
```

##### Fell glare

<!-- splat:yaml path=splat_rule/parameters/features/fell_glare -->

```yaml
economy: bonus_action
requires_type:
- fiend
resource:
  capacity: 1
  cost: 1
  recovery: long_rest
op: save
ability: wisdom
dc:
  input: summoner_spell_save_dc
targets:
  kind: creature
  count: 1
  range_feet: 60
  visible_to: self
failure:
- op: condition
  condition: frightened
  ends:
    event: turn.end
    participant: summoner
    occurrence: next
success: []
```

##### Fey step

<!-- splat:yaml path=splat_rule/parameters/features/fey_step -->

```yaml
economy: bonus_action
resource:
  capacity: 1
  cost: 1
  recovery: long_rest
requires_type:
- fey
op: teleport
targets:
- self
- rider
range_feet: 60
destination:
  unoccupied: true
  chooser: summoner
```

##### Healing touch

<!-- splat:values path=splat_rule/parameters/features/healing_touch -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` requires_type ` | ` ["celestial"] ` |
| ` op ` | ` heal ` |

###### Resource

<!-- splat:values path=splat_rule/parameters/features/healing_touch/resource -->

| Field | Value |
| --- | --- |
| ` capacity ` | ` 1 ` |
| ` cost ` | ` 1 ` |
| ` recovery ` | ` long_rest ` |

###### Targets

<!-- splat:values path=splat_rule/parameters/features/healing_touch/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` range_feet ` | ` 5 ` |

###### Amount

###### Add

<!-- splat:table
path: splat_rule/parameters/features/healing_touch/amount/add
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Dice count:
    path: dice/count
    type: integer
    optional: true
  Dice sides:
    path: dice/sides
    type: integer
    optional: true
  Add:
    path: add
    type: integer
    optional: true
  Input:
    path: input
    type: symbol
    optional: true
-->

| Key | Dice count | Dice sides | Add | Input |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 2 ` | ` 8 ` | ` 0 ` |  |
| ` row_2 ` |  |  |  | ` Spell level ` |

#### Defenses

<!-- splat:yaml path=splat_rule/parameters/defenses -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

##### Special

<!-- splat:yaml path=splat_rule/parameters/senses/special -->

```yaml
[]
```

#### Communication

##### Speaks

<!-- splat:yaml path=splat_rule/parameters/communication/speaks -->

```yaml
[]
```

##### Understands

<!-- splat:yaml path=splat_rule/parameters/communication/understands -->

```yaml
[]
```

##### Telepathy

<!-- splat:values path=splat_rule/parameters/communication/telepathy -->

| Field | Value |
| --- | --- |
| ` range_feet ` | ` 5280 ` |
| ` target ` | ` summoner ` |

#### Proficiency bonus

<!-- splat:values path=splat_rule/parameters/proficiency_bonus -->

| Field | Value |
| --- | --- |
| ` input ` | ` summoner_proficiency_bonus ` |

#### Scaling stats

<!-- splat:yaml path=splat_rule/parameters/scaling_stats -->

```yaml
ac:
  add:
  - 10
  - input: spell_level
hit_points:
  add:
  - 5
  - multiply:
    - 10
    - input: spell_level
hit_dice:
  count:
    input: spell_level
  sides: 10
fly_speed_feet:
  if:
    spell_level_min: 4
  then: 60
  else: 0
```

#### Type choice

<!-- splat:values path=splat_rule/parameters/type_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` summoner ` |
| ` options ` | ` ["celestial", "fey", "fiend"] ` |

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
