---
splat_schema: 1
splat_id: d84e740e-9522-58e4-b9d1-ecf40f3f5485
splat_kind: creature
splat_key: dnd2024:creatures/marilith
splat_name: Marilith
splat_origin:
  key: dnd2024:reference/creature/marilith
splat_sources:
- title: SRD 5.2.1
  locator: pp. 306
  url: https://www.dndbeyond.com/srd
---

# Marilith

## Source text

<!-- SRD page 306 -->

Large Fiend (Demon), Chaotic Evil\
AC 16 Initiative +10 (20)\
HP 220 (21d10 + 105)\
Speed 40 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +9 Dex 20 +5 +5 Con 20 +5 +10\
Int 18 +4 +4 Wis 16 +3 +8 Cha 20 +5 +10\
Skills Perception +8\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Truesight 120 ft.; Passive Perception 18\
Languages Abyssal; telepathy 120 ft.\
CR 16 (XP 15,000; PB +5)\

### Traits

Demonic Restoration. If the marilith dies outside the\
Abyss, its body dissolves into ichor, and it gains a new\
body instantly, reviving with all its Hit Points some-\
where in the Abyss.\
Magic Resistance. The marilith has Advantage on sav-\
ing throws against spells and other magical effects.\
Reactive. The marilith can take one Reaction on every\
turn of combat.\

### Actions

Multiattack. The marilith makes six Pact Blade attacks\
and uses Constrict.\
Pact Blade. Melee Attack Roll: +10, reach 5 ft. Hit:\
10 (1d10 + 5) Slashing damage plus 7 (2d6) Ne-\
crotic damage.\
Constrict. Strength Saving Throw: DC 17, one Medium\
or smaller creature the marilith can see within 5 feet.\
Failure: 15 (2d10 + 4) Bludgeoning damage. The target\
has the Grappled condition (escape DC 14), and it has\
the Restrained condition until the grapple ends.\

### Bonus Actions

Teleport (Recharge 5–6). The marilith teleports up to\
120 feet to an unoccupied space it can see.\

### Reactions

Parry. Trigger: The marilith is hit by a melee attack roll\
while holding a weapon. Response: The marilith adds 5\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/306|page 306]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Chaotic Evil ` |
| ` tags_text ` | ` (Demon) ` |
| ` movement_text ` | ` 40 ft., Climb 40 ft. ` |
| ` skills_text ` | ` Perception +8 ` |
| ` resistances_text ` | ` Cold, Fire, Lightning ` |
| ` immunities_text ` | ` Poison; Poisoned ` |
| ` senses_text ` | ` Truesight 120 ft.; Passive Perception 18 ` |
| ` languages_text ` | ` Abyssal; telepathy 120 ft. ` |
| ` cr_text ` | ` 16 (XP 15,000; PB +5) ` |
| ` hit_points_text ` | ` 220 (21d10 + 105) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 9 ` |
| ` Dexterity ` | ` 20 ` | ` 5 ` |
| ` Constitution ` | ` 20 ` | ` 10 ` |
| ` Intelligence ` | ` 18 ` | ` 4 ` |
| ` Wisdom ` | ` 16 ` | ` 8 ` |
| ` Charisma ` | ` 20 ` | ` 10 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 16 ` |  |
| ` max_hp ` | ` 220 ` |  |
| ` initiative ` | ` 10 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |
| ` climb_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/marilith ` |
| ` implementation ` | ` dnd2024:creature_marilith ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold", "fire", "lightning"] ` |
| ` damage_immunities ` | ` ["poison"] ` |
| ` condition_immunities ` | ` ["poisoned"] ` |
| ` passive_perception ` | ` 18 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 8 ` |

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
| ` known ` | ` ["abyssal"] ` |

#### Telepathy

<!-- splat:values path=splat_rule/parameters/telepathy -->

| Field | Value |
| --- | --- |
| ` range_ft ` | ` 120 ` |
| ` two_way ` | ` true ` |

#### Features

##### Demonic restoration

<!-- splat:yaml path=splat_rule/parameters/features/demonic_restoration -->

```yaml
economy: passive
trigger: actor.died
when:
  plane_not: abyss
body: disappears
revive:
  new_body: true
  plane: abyss
  location_choice: gm
  hp: maximum
  delay:
    amount: 0
    unit: second
```

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` roll.advantage ` |
| ` rolls ` | ` ["saving_throw"] ` |
| ` against ` | ` ["spell", "magical_effect"] ` |

##### Reactive

<!-- splat:values path=splat_rule/parameters/features/reactive -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` economy.reaction_capacity ` |
| ` count ` | ` 1 ` |
| ` refresh ` | ` each_combat_turn ` |

##### Pact blade

<!-- splat:yaml path=splat_rule/parameters/features/pact_blade -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 10
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
        count: 1
        sides: 10
      add: 5
    average: 10
  - type: necrotic
    amount:
      dice:
        count: 2
        sides: 6
      add: 0
    average: 7
```

##### Teleport

<!-- splat:yaml path=splat_rule/parameters/features/teleport -->

```yaml
economy: bonus_action
op: teleport
maximum_ft: 120
destination:
  unoccupied: true
  visible_to_actor: true
recharge:
  die_sides: 6
  success_min: 5
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Parry

<!-- splat:yaml path=splat_rule/parameters/features/parry -->

```yaml
economy: reaction
trigger: melee_attack.hit_actor
requires:
  holding_weapon: true
op: ac.add
amount: 5
scope: triggering_attack
recheck_hit: true
```

##### Constrict

<!-- splat:yaml path=splat_rule/parameters/features/constrict -->

```yaml
economy: action
op: saving_throw
ability: strength
dc: 17
targets:
  kind: creature
  count: 1
  range_ft: 5
  maximum_size: medium
  visible_to_actor: true
failure:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 2
        sides: 10
      add: 4
    average: 15
- op: grapple
  escape_dc: 14
  while_grappled:
  - op: condition.apply
    condition: restrained
success: []
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
| ` row_1 ` | ` Pact blade ` | ` 6 ` |
| ` row_2 ` | ` Constrict ` | ` 1 ` |

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
