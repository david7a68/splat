---
splat_schema: 1
splat_id: 172a6671-e997-51ae-b11a-9885c15c3af9
splat_kind: creature
splat_key: dnd2024:creatures/bearded_devil
splat_name: Bearded Devil
splat_origin:
  key: dnd2024:reference/creature/bearded_devil
splat_sources:
- title: SRD 5.2.1
  locator: pp. 262, 263
  url: https://www.dndbeyond.com/srd
---

# Bearded Devil

## Source text

<!-- SRD page 262 -->

Medium Fiend (Devil), Lawful Evil\
AC 13 Initiative +2 (12)\
HP 58 (9d8 + 18)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +5 Dex 15 +2 +2 Con 15 +2 +4\
Int 9 −1 −1 Wis 11 +0 +0 Cha 14 +2 +4\
Resistances Cold\
Immunities Fire, Poison; Frightened, Poisoned\
Senses Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 10\
Languages Infernal; telepathy 120 ft.\
CR 3 (XP 700; PB +2)

<!-- SRD page 263 -->

### Traits

Magic Resistance. The devil has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The devil makes one Beard attack and one Infernal Glaive attack.

Beard. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage, and the target has the Poisoned condition until the start of the devil’s next turn. Until this poison ends, the target can’t regain Hit Points.

Infernal Glaive. Melee Attack Roll: +5, reach 10 ft. Hit: 8 (1d10 + 3) Slashing damage. If the target is a creature and doesn’t already have an infernal wound, it is subjected to the following effect. Constitution Saving Throw: DC 12. Failure: The target receives an infernal wound. While wounded, the target loses 5 (1d10) Hit Points at the start of each of its turns. The wound closes after 1 minute, after a spell restores Hit Points to the target, or after the target or a creature within 5 feet of it takes an action to stanch the wound, doing so by succeeding on a DC 12 Wisdom (Medicine) check.

Original layout: [[../../srd-5.2.1/pages/262|page 262]], [[../../srd-5.2.1/pages/263|page 263]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Lawful Evil ` |
| ` tags_text ` | ` (Devil) ` |
| ` movement_text ` | ` 30 ft. ` |
| ` resistances_text ` | ` Cold ` |
| ` immunities_text ` | ` Fire, Poison; Frightened, Poisoned ` |
| ` senses_text ` | ` Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 10 ` |
| ` languages_text ` | ` Infernal; telepathy 120 ft. ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 58 (9d8 + 18) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 5 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 15 ` | ` 4 ` |
| ` Intelligence ` | ` 9 ` | ` -1 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 14 ` | ` 4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 58 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/bearded_devil ` |
| ` implementation ` | ` dnd2024:creature_bearded_devil ` |

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

##### Beard

<!-- splat:yaml path=splat_rule/parameters/features/beard -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 7
  amount:
    dice:
      count: 1
      sides: 8
    add: 3
  damage_type: piercing
- op: condition
  condition: poisoned
  ends:
    event: turn.start
    participant: self
    occurrence: next
  while_active:
    regain_hit_points: false
reach_feet: 5
```

##### Infernal glaive

<!-- splat:yaml path=splat_rule/parameters/features/infernal_glaive -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 5
targets:
  count: 1
hit:
- op: damage
  average: 8
  amount:
    dice:
      count: 1
      sides: 10
    add: 3
  damage_type: slashing
- op: if
  test:
    target_kind: creature
    target_has_infernal_wound: false
  then:
  - op: save
    ability: constitution
    dc: 12
    targets:
      participant: target
    failure:
    - op: infernal_wound
      stacking: one_per_target
      periodic:
        event: target.turn.start
        effects:
        - op: lose_hit_points
          average: 5
          amount:
            dice:
              count: 1
              sides: 10
            add: 0
      ends_any:
      - minutes: 1
      - event: spell.restores_hit_points
        target: affected
      - economy: action
        actor: target_or_creature_within_5_feet
        check:
          ability: wisdom
          skill: medicine
          dc: 12
        outcome: success
    success: []
reach_feet: 10
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: beard
  count: 1
- op: use_feature
  feature: infernal_glaive
  count: 1
```

#### Defenses

##### Resistances

<!-- splat:values path=splat_rule/parameters/defenses/resistances -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["cold"] ` |

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` damage_types ` | ` ["fire", "poison"] ` |
| ` conditions ` | ` ["frightened", "poisoned"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet | magical_darkness_unimpeded |
| --- | --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` | ` true ` |

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` speaks ` | ` ["infernal"] ` |
| ` understands ` | ` ["infernal"] ` |

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
