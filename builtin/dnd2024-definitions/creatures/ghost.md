---
splat_schema: 1
splat_id: 03807394-3280-5142-bdda-03ebdf020faa
splat_kind: creature
splat_key: dnd2024:creatures/ghost
splat_name: Ghost
splat_origin:
  key: dnd2024:reference/creature/ghost
splat_sources:
- title: SRD 5.2.1
  locator: pp. 287, 288
  url: https://www.dndbeyond.com/srd
---

# Ghost

## Source text

<!-- SRD page 287 -->

Medium Undead, Neutral\
AC 11 Initiative +1 (11)\
HP 45 (10d8)\
Speed 5 ft., Fly 40 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 13 +1 +1 Con 10 +0 +0\
Int 10 +0 +0 Wis 12 +1 +1 Cha 17 +3 +3\
Resistances Acid, Bludgeoning, Cold, Fire, Lightning,\
Piercing, Slashing, Thunder\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Frightened, Grappled, Paralyzed, Petrified, Poisoned,\
Prone, Restrained\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Common plus one other language\
CR 4 (XP 1,100; PB +2)\

### Traits

Ethereal Sight. The ghost can see 60 feet into the Ethe-\
real Plane when it is on the Material Plane.\
Incorporeal Movement. The ghost can move through\
other creatures and objects as if they were Difficult\
Terrain. It takes 5 (1d10) Force damage if it ends its turn\
inside an object.\

<!-- SRD page 288 -->


### Actions

Multiattack. The ghost makes two Withering\
Touch attacks.\
Withering Touch. Melee Attack Roll: +5, reach 5 ft.\
Hit: 19 (3d10 + 3) Necrotic damage.\
Etherealness. The ghost casts the Etherealness spell,\
requiring no spell components and using Charisma as\
the spellcasting ability. The ghost is visible on the Mate-\
rial Plane while on the Border Ethereal and vice versa,\
but it can’t affect or be affected by anything on the\
other plane.\
Horrific Visage. Wisdom Saving Throw: DC 13, each\
creature in a 60-foot Cone that can see the ghost and\
isn’t an Undead. Failure: 10 (2d6 + 3) Psychic damage,\
and the target has the Frightened condition until the\
start of the ghost’s next turn. Success: The target is im-\
mune to this ghost’s Horrific Visage for 24 hours.\
Possession (Recharge 6). Charisma Saving Throw: DC\
13, one Humanoid the ghost can see within 5 feet. Fail-\
ure: The target is possessed by the ghost; the ghost dis-\
appears, and the target has the Incapacitated condition\
and loses control of its body. The ghost now controls\
the body, but the target retains awareness. The ghost\
can’t be targeted by any attack, spell, or other effect,\
except ones that specifically target Undead. The ghost’s\
game statistics are the same, except it uses the pos-\
sessed target’s Speed, as well as the target’s Strength,\
Dexterity, and Constitution modifiers.\
The possession lasts until the body drops to 0 Hit\
Points or the ghost leaves as a Bonus Action. When the\
possession ends, the ghost appears in an unoccupied\
space within 5 feet of the target, and the target is im-\
mune to this ghost’s Possession for 24 hours.\
Success: The target is immune to this ghost’s Posses-\
sion for 24 hours.\

Original layout: [[../../srd-5.2.1/pages/287|page 287]], [[../../srd-5.2.1/pages/288|page 288]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` undead ` |
| ` alignment ` | ` Neutral ` |
| ` movement_text ` | ` 5 ft., Fly 40 ft. (hover) ` |
| ` resistances_text ` | ` Acid, Bludgeoning, Cold, Fire, Lightning, Piercing, Slashing, Thunder ` |
| ` immunities_text ` | ` Necrotic, Poison; Charmed, Exhaustion, Frightened, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 11 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 45 (10d8) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 7 ` | ` -2 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 17 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 45 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 5 ` | ` ft ` |
| ` fly_speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ghost ` |
| ` implementation ` | ` dnd2024:creature_ghost ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["acid", "bludgeoning", "cold", "fire", "lightning", "piercing", "slashing", "thunder"] ` |
| ` damage_immunities ` | ` ["necrotic", "poison"] ` |
| ` condition_immunities ` | ` ["charmed", "exhaustion", "frightened", "grappled", "paralyzed", "petrified", "poisoned", "prone", "restrained"] ` |
| ` passive_perception ` | ` 11 ` |

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
| ` known ` | ` ["common"] ` |
| ` additional_choice_count ` | ` 1 ` |

#### Features

##### Ethereal sight

<!-- splat:values path=splat_rule/parameters/features/ethereal_sight -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` vision.cross_plane ` |
| ` from ` | ` material ` |
| ` to ` | ` ethereal ` |
| ` range_ft ` | ` 60 ` |

##### Incorporeal movement

<!-- splat:yaml path=splat_rule/parameters/features/incorporeal_movement -->

```yaml
economy: passive
op: movement.pass_through
entities:
- creature
- object
cost: difficult_terrain
end_turn_inside_object:
- op: damage
  components:
  - type: force
    amount:
      dice:
        count: 1
        sides: 10
      add: 0
    average: 5
```

##### Withering touch

<!-- splat:yaml path=splat_rule/parameters/features/withering_touch -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 5
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: necrotic
    amount:
      dice:
        count: 3
        sides: 10
      add: 3
    average: 19
```

##### Etherealness

<!-- splat:yaml path=splat_rule/parameters/features/etherealness -->

```yaml
economy: action
op: spell.cast
spell:
  key: dnd2024:spells/etherealness
ability: charisma
waived_components:
- verbal
- somatic
- material
overrides:
  visible_across_planes:
  - material
  - border_ethereal
  cross_plane_effects_allowed: false
```

##### Horrific visage

<!-- splat:yaml path=splat_rule/parameters/features/horrific_visage -->

```yaml
economy: action
op: saving_throw
ability: wisdom
dc: 13
targets:
  kind: creature
  count: all
  area:
    shape: cone
    origin: actor
    size_ft: 60
  must_see_actor: true
  exclude_creature_types:
  - undead
failure:
- op: damage
  components:
  - type: psychic
    amount:
      dice:
        count: 2
        sides: 6
      add: 3
    average: 10
- op: condition.apply
  condition: frightened
  duration:
    until: actor.next_turn_start
success:
- op: feature.immunity
  source: actor
  feature: horrific_visage
  duration:
    amount: 24
    unit: hour
```

##### Possession

<!-- splat:yaml path=splat_rule/parameters/features/possession -->

```yaml
economy: action
op: saving_throw
ability: charisma
dc: 13
targets:
  kind: creature
  count: 1
  range_ft: 5
  visible_to_actor: true
  creature_types:
  - humanoid
failure:
- op: possession.start
  actor_disappears: true
  target_conditions:
  - incapacitated
  body_control: actor
  target_retains_awareness: true
  actor_targetable_only_by: effects_specifically_targeting_undead
  actor_stat_overrides_from_target:
  - speed
  - strength_modifier
  - dexterity_modifier
  - constitution_modifier
  end_when_any:
  - body_hp_zero
  - actor.bonus_action_leave
  end_effects:
  - op: actor.appear
    within_target_ft: 5
    unoccupied: true
  - op: feature.immunity
    source: actor
    feature: possession
    duration:
      amount: 24
      unit: hour
success:
- op: feature.immunity
  source: actor
  feature: possession
  duration:
    amount: 24
    unit: hour
leave_economy: bonus_action
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
  feature: withering_touch
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
