---
splat_schema: 1
splat_id: 938946e7-6520-5556-87f0-621c883cc5ce
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_2_channel_divinity
splat_name: 'Level 2: Channel Divinity'
splat_origin:
  key: dnd2024:reference/pages/037
splat_sources:
- title: SRD 5.2.1
  locator: pp. 37
  url: https://www.dndbeyond.com/srd
---

# Level 2: Channel Divinity

## Source text

<!-- SRD page 37 -->

You can channel divine energy directly from the Outer Planes to fuel magical effects. You start with two such effects: Divine Spark and Turn Undead, each of which is described below. Each time you use this class’s Channel Divinity, choose which Channel Divinity effect from this class to create. You gain additional effect options at higher Cleric levels.

You can use this class’s Channel Divinity twice. You regain one of its expended uses when you finish a Short Rest, and you regain all expended uses when you finish a Long Rest. You gain additional uses when you reach certain Cleric levels, as shown in the Channel Divinity column of the Cleric Features table.

If a Channel Divinity effect requires a saving throw, the DC equals the spell save DC from this class’s Spellcasting feature.

Divine Spark. As a Magic action, you point your Holy Symbol at another creature you can see within 30 feet of yourself and focus divine energy at it. Roll 1d8 and add your Wisdom modifier. You either restore Hit Points to the creature equal to that total or force the creature to make a Constitution saving throw. On a failed save, the creature takes Necrotic or Radiant damage (your choice) equal to that total. On a successful save, the creature takes half as much damage (round down).

You roll an additional d8 when you reach Cleric levels 7 (2d8), 13 (3d8), and 18 (4d8).

Turn Undead. As a Magic action, you present your Holy Symbol and censure Undead creatures. Each Undead of your choice within 30 feet of you must make a Wisdom saving throw. If the creature fails its save, it has the Frightened and Incapacitated conditions for 1 minute. For that duration, it tries to move as far from you as it can on its turns. This effect ends early on the creature if it takes any damage, if you have the Incapacitated condition, or if you die.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_2_channel_divinity ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_2_channel_divinity ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost_per_use ` | ` 1 ` |
| ` choose_effect ` | ` 1 ` |

#### Resource

<!-- splat:values path=splat_rule/parameters/resource -->

| Field | Value |
| --- | --- |
| ` id ` | ` cleric.channel_divinity ` |

##### Maximum

<!-- splat:yaml path=splat_rule/parameters/resource/maximum -->

```yaml
table:
  key: dnd2024:tables/cleric_progression
row:
  var: actor.class_levels.cleric
column: channel_divinity
```

##### Recovery

<!-- splat:table
path: splat_rule/parameters/resource/recovery
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Details:
    type: section
    optional: true
-->

| Key | Event | Details |
| --- | --- | --- |
| ` row_1 ` | ` rest.short_completed ` | [Rules](#^splat-d8de689e-row_1 "section:row_1") |
| ` row_2 ` | ` rest.long_completed ` | [Rules](#^splat-d8de689e-row_2 "section:row_2") |

###### Row 1

^splat-d8de689e-row_1

###### Amount

<!-- splat:yaml path=splat_rule/parameters/resource/recovery/@row_1/amount -->

```yaml
1
```

###### Row 2

^splat-d8de689e-row_2

###### Amount

<!-- splat:yaml path=splat_rule/parameters/resource/recovery/@row_2/amount -->

```yaml
all
```

#### Dc

<!-- splat:yaml path=splat_rule/parameters/dc -->

```yaml
op: add
args:
- 8
- var: actor.abilities.wisdom.modifier
- var: actor.proficiency_bonus
```

#### Options

##### Divine spark

<!-- splat:values path=splat_rule/parameters/options/divine_spark -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` requires_item ` | ` holy_symbol ` |
| ` choice ` | ` ["heal", "damage"] ` |

###### Target

<!-- splat:values path=splat_rule/parameters/options/divine_spark/target -->

| Field | Value |
| --- | --- |
| ` type ` | ` creature ` |
| ` self ` | ` false ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 30 ` |

###### Amount

<!-- splat:values path=splat_rule/parameters/options/divine_spark/amount -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

###### Args

<!-- splat:table
path: splat_rule/parameters/options/divine_spark/amount/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Dice sides:
    path: dice/sides
    type: integer
    optional: true
  Var:
    path: var
    type: string
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Dice sides | Var | Details |
| --- | --- | --- | --- |
| ` row_1 ` | ` 8 ` |  | [Rules](#^splat-ad2792df-row_1 "section:row_1") |
| ` row_2 ` |  | ` actor.abilities.wisdom.modifier ` |  |

###### Row 1

^splat-ad2792df-row_1

###### Count by cleric level

<!-- splat:yaml path=splat_rule/parameters/options/divine_spark/amount/args/@row_1/dice/count_by_cleric_level -->

```yaml
'2': 1
'7': 2
'13': 3
'18': 4
```

###### Damage

<!-- splat:values path=splat_rule/parameters/options/divine_spark/damage -->

| Field | Value |
| --- | --- |
| ` types_choice ` | ` ["necrotic", "radiant"] ` |
| ` save ` | ` constitution ` |
| ` success_multiplier ` | ` 0.5 ` |
| ` rounding ` | ` down ` |

##### Turn undead

<!-- splat:yaml path=splat_rule/parameters/options/turn_undead -->

```yaml
action: magic
requires_item: holy_symbol
targets:
  type: undead
  selection: actor_choice
  range_feet: 30
save: wisdom
failure:
  conditions:
  - frightened
  - incapacitated
  duration:
    minutes: 1
  on_target_turn: move_as_far_as_possible_from_actor
  ends_on:
  - target.damage_taken
  - actor.incapacitated
  - actor.died
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` minimum_level ` | ` 2 ` |
