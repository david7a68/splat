---
splat_schema: 1
splat_id: 58b4c331-4c93-59f2-91fc-df243d8d664c
splat_kind: creature
splat_key: dnd2024:creatures/death_dog
splat_name: Death Dog
splat_origin:
  key: dnd2024:reference/creature/death_dog
splat_sources:
- title: SRD 5.2.1
  locator: pp. 279
  url: https://www.dndbeyond.com/srd
---

# Death Dog

## Source text

<!-- SRD page 279 -->

Medium Monstrosity, Neutral Evil\
AC 12 Initiative +2 (12)\
HP 39 (6d8 + 12)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +2 Con 14 +2 +2\
Int 3 −4 −4 Wis 13 +1 +1 Cha 6 −2 −2\
Skills Perception +5, Stealth +4\
Immunities Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious\
Senses Darkvision 120 ft.; Passive Perception 15\
Languages None\
CR 1 (XP 200; PB +2)

### Actions

Multiattack. The death dog makes two Bite attacks.

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage. If the target is a creature, it is subjected to the following effect. Constitution Saving Throw: DC 12. First Failure: The target has the Poisoned condition. While Poisoned, the target’s Hit Point maximum doesn’t return to normal when finishing a Long Rest, and it repeats the save every 24 hours that elapse, ending the effect on itself on a success. Subsequent Failures: The Poisoned target’s Hit Point maximum decreases by 5 (1d10).

Original layout: [[../../srd-5.2.1/pages/279|page 279]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 40 ft. ` |
| ` skills_text ` | ` Perception +5, Stealth +4 ` |
| ` immunities_text ` | ` Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious ` |
| ` senses_text ` | ` Darkvision 120 ft.; Passive Perception 15 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 39 (6d8 + 12) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 2 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 14 ` | ` 2 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 39 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/death_dog ` |
| ` implementation ` | ` dnd2024:creature_death_dog ` |

### Parameters

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
attack_modes:
- melee
attack_bonus: 4
targets:
  count: 1
hit:
- op: damage
  average: 4
  amount:
    dice:
      count: 1
      sides: 4
    add: 2
  damage_type: piercing
- op: if
  test:
    target_kind: creature
  then:
  - op: save
    ability: constitution
    dc: 12
    targets:
      participant: target
    failure:
    - op: choose_by_predicate
      test:
        target_has_source_poison_effect: true
      then:
      - op: reduce_hit_point_maximum
        average: 5
        amount:
          dice:
            count: 1
            sides: 10
          add: 0
      else:
      - op: condition
        condition: poisoned
        ends:
          event: repeated_save.success
        while_active:
          long_rest_restores_hp_maximum: false
          repeat_save:
            ability: constitution
            dc: 12
            interval:
              hours: 24
            success: end_effect
            failure:
            - op: reduce_hit_point_maximum
              average: 5
              amount:
                dice:
                  count: 1
                  sides: 10
                add: 0
    success: []
reach_feet: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: sequence
steps:
- op: use_feature
  feature: bite
  count: 2
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 4 ` |

#### Defenses

##### Immunities

<!-- splat:values path=splat_rule/parameters/defenses/immunities -->

| Field | Value |
| --- | --- |
| ` conditions ` | ` ["blinded", "charmed", "deafened", "frightened", "stunned", "unconscious"] ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

##### Special

<!-- splat:rows path=splat_rule/parameters/senses/special -->

| Row | sense | range_feet |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 120 ` |

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
