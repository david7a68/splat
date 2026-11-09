---
splat_schema: 2
splat_id: cf229e93-690c-5692-a3b5-ed157c48597e
splat_kind: creature
splat_key: dnd2024:creatures/werewolf
splat_name: Werewolf
splat_origin: dnd2024:reference/creature/werewolf
splat_sources:
- SRD 5.2.1, pp. 339, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 327, Habitat
---

# Werewolf


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Monstrosity (Lycanthrope) |
| Alignment | Chaotic Evil |
| Armor Class | 15 |
| Hit Points | 71 (11d8 + 22) |
| Initiative | +4 (14) |
| Speed | 30 ft., 40 ft. (wolf form only) |
| Challenge | 3 (XP 700; PB +2) |
| Skills | Perception +4, Stealth +4 |
| Senses | Darkvision 60 ft.; Passive Perception 14 |
| Languages | Common (can’t speak in wolf form) |
| Gear | Longbow |
| Habitat | Forest, Hill |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 14 | +2 | +2 |
| Con | 14 | +2 | +2 |
| Int | 10 | +0 | +0 |
| Wis | 11 | +0 | +0 |
| Cha | 10 | +0 | +0 |

%% splat shape=speed_max %%


## Source text

<!-- SRD page 339 -->

Medium or Small Monstrosity (Lycanthrope), Chaotic Evil\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 15 Initiative +4 (14)\
> HP 71 (11d8 + 22)\
> Speed 30 ft., 40 ft. (wolf form only)\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2\
> Int 10 +0 +0 Wis 11 +0 +0 Cha 10 +0 +0\
> Skills Perception +4, Stealth +4\
> Gear Longbow\
> Senses Darkvision 60 ft.; Passive Perception 14\
> Languages Common (can’t speak in wolf form)\
> CR 3 (XP 700; PB +2)


### Traits

Pack Tactics. The werewolf has Advantage on an attack roll against a creature if at least one of the werewolf’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Multiattack. The werewolf makes two attacks, using Scratch or Longbow in any combination. It can replace one attack with a Bite attack.

Bite (Wolf or Hybrid Form Only). Melee Attack Roll: +5, reach 5 ft. Hit: 12 (2d8 + 3) Piercing damage. If the target is a Humanoid, it is subjected to the following effect. Constitution Saving Throw: DC 12. Failure: The target is cursed. If the cursed target drops to 0 Hit Points, it instead becomes a Werewolf under the GM’s control and has 10 Hit Points. Success: The target is immune to this werewolf’s curse for 24 hours.

Scratch. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage.

Longbow (Humanoid or Hybrid Form Only). Ranged Attack Roll: +4, range 150/600 ft. Hit: 11 (2d8 + 2) Piercing damage.

### Bonus Actions

Shape-Shift. The werewolf shape-shifts into a Large wolf-humanoid hybrid or a Medium wolf, or it returns to its true humanoid form. Its game statistics, other than its size, are the same in each form. Any equipment it is wearing or carrying isn’t transformed.

Original layout: [[../../srd-5.2.1/pages/339|page 339]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/werewolf ` |
| ` implementation ` | ` dnd2024:creature_werewolf ` |

### Parameters

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: trait
operation: passive_rule
attack_advantage:
  target_kind: creature
  ally_within_target_ft: 5
  ally_excludes_conditions:
  - incapacitated
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - scratch
  - longbow
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: bite
```

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 5
hit:
  damage:
  - average: 12
    roll:
      count: 2
      sides: 8
      modifier: 3
    type: piercing
  save:
    operation: saving_throw
    ability: constitution
    dc: 12
    targets:
      kind: humanoid
    failure:
      curse:
        on_target_zero_hp:
          replace:
            definition:
              key: dnd2024:creatures/werewolf
            controller: gm
            hp: 10
    success:
      immunity:
        effect: this_creature_curse
        duration_seconds: 86400
allowed_forms:
- wolf
- hybrid
```

##### Scratch

<!-- splat:yaml path=splat_rule/parameters/features/scratch -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 5
hit:
  damage:
  - average: 10
    roll:
      count: 2
      sides: 6
      modifier: 3
    type: slashing
```

##### Longbow

<!-- splat:yaml path=splat_rule/parameters/features/longbow -->

```yaml
economy: action
operation: attack
attack:
  mode: ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  range_ft: 150
  long_range_ft: 600
hit:
  damage:
  - average: 11
    roll:
      count: 2
      sides: 8
      modifier: 2
    type: piercing
allowed_forms:
- humanoid
- hybrid
```

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` operation ` | ` shape_shift ` |
| ` statistics_preserved_except ` | ` ["size", "form_specific_speed"] ` |
| ` equipment_transformed ` | ` false ` |

###### Forms

<!-- splat:table
path: splat_rule/parameters/features/shape_shift/forms
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Form:
    path: form
    type: symbol
  Size:
    path: size
    type: symbol
-->

| Key | Form | Size |
| --- | --- | --- |
| ` row_1 ` | ` Hybrid ` | ` Large ` |
| ` row_2 ` | ` Wolf ` | ` Medium ` |
| ` row_3 ` | ` True humanoid ` | ` Original ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |
| ` stealth ` | ` 4 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | quantity |
| --- | --- | --- |
| ` 0 ` | ` longbow ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 14 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
understands: []
cannot_speak_forms:
- wolf
```

#### Form rules

<!-- splat:yaml path=splat_rule/parameters/form_rules -->

```yaml
beast:
  speed_ft: 40
  can_speak: false
true_humanoid:
  speed_ft: 30
hybrid:
  speed_ft: 30
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```
