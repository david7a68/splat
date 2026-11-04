---
splat_schema: 2
splat_id: 57bf9535-1847-5e11-b533-93abe71c4e10
splat_kind: creature
splat_key: dnd2024:creatures/wereboar
splat_name: Wereboar
splat_origin: dnd2024:reference/creature/wereboar
splat_sources:
- SRD 5.2.1, pp. 338, https://www.dndbeyond.com/srd
---

# Wereboar


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Monstrosity (Lycanthrope) |
| Alignment | Neutral Evil |
| Armor Class | 15 |
| Hit Points | 97 (15d8 + 30) |
| Initiative | +2 (12) |
| Speed | 30 ft., 40 ft. (boar form only) |
| Challenge | 4 (XP 1,100; PB +2) |
| Skills | Perception +2 |
| Senses | Passive Perception 12 |
| Languages | Common (can’t speak in boar form) |
| Gear | Javelins (6) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 10 | +0 | +0 |
| Con | 15 | +2 | +2 |
| Int | 10 | +0 | +0 |
| Wis | 11 | +0 | +0 |
| Cha | 8 | -1 | -1 |

%% splat shape=speed_max %%


## Source text

<!-- SRD page 338 -->

Medium or Small Monstrosity (Lycanthrope), Neutral Evil\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 15 Initiative +2 (12)\
> HP 97 (15d8 + 30)\
> Speed 30 ft., 40 ft. (boar form only)\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 10 +0 +0 Con 15 +2 +2\
> Int 10 +0 +0 Wis 11 +0 +0 Cha 8 −1 −1\
> Skills Perception +2\
> Gear Javelins (6)\
> Senses Passive Perception 12\
> Languages Common (can’t speak in boar form)\
> CR 4 (XP 1,100; PB +2)


### Actions

Multiattack. The wereboar makes two attacks, using Javelin or Tusk in any combination. It can replace one attack with a Gore attack.

Gore (Boar or Hybrid Form Only). Melee Attack Roll: +5, reach 5 ft. Hit: 12 (2d8 + 3) Piercing damage. If the target is a Humanoid, it is subjected to the following effect. Constitution Saving Throw: DC 12. Failure: The target is cursed. If the cursed target drops to 0 Hit Points, it instead becomes a Wereboar under the GM’s control and has 10 Hit Points. Success: The target is immune to this wereboar’s curse for 24 hours.

Javelin (Humanoid or Hybrid Form Only). Melee or Ranged Attack Roll: +5, reach 5 ft. or range 30/120 ft. Hit: 13 (3d6 + 3) Piercing damage.

Tusk (Boar or Hybrid Form Only). Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage. If the target is a Medium or smaller creature and the wereboar moved 20+ feet straight toward it immediately before the hit, the target takes an extra 7 (2d6) Piercing damage and has the Prone condition.

### Bonus Actions

Shape-Shift. The wereboar shape-shifts into a Medium boar-humanoid hybrid or a Small boar, or it returns to its true humanoid form. Its game statistics, other than its size, are the same in each form. Any equipment it is wearing or carrying isn’t transformed.

Original layout: [[../../srd-5.2.1/pages/338|page 338]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/wereboar ` |
| ` implementation ` | ` dnd2024:creature_wereboar ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - javelin
  - tusk
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: gore
```

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

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
              key: dnd2024:creatures/wereboar
            controller: gm
            hp: 10
    success:
      immunity:
        effect: this_creature_curse
        duration_seconds: 86400
allowed_forms:
- boar
- hybrid
```

##### Javelin

<!-- splat:yaml path=splat_rule/parameters/features/javelin -->

```yaml
economy: action
operation: attack
attack:
  mode: melee_or_ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 5
  range_ft: 30
  long_range_ft: 120
hit:
  damage:
  - average: 13
    roll:
      count: 3
      sides: 6
      modifier: 3
    type: piercing
allowed_forms:
- humanoid
- hybrid
```

##### Tusk

<!-- splat:yaml path=splat_rule/parameters/features/tusk -->

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
    type: piercing
  prone:
    condition: prone
    maximum_size: medium
    requires:
      immediately_preceding_straight_movement_toward_target_ft: 20
  conditional_damage:
    requires:
      maximum_target_size: medium
      immediately_preceding_straight_movement_toward_target_ft: 20
    damage:
    - roll:
        count: 2
        sides: 6
        modifier: 0
      type: piercing
allowed_forms:
- boar
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
| ` row_1 ` | ` Hybrid ` | ` Medium ` |
| ` row_2 ` | ` Boar ` | ` Small ` |
| ` row_3 ` | ` True humanoid ` | ` Original ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | quantity |
| --- | --- | --- |
| ` 0 ` | ` javelins ` | ` 6 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
understands: []
cannot_speak_forms:
- boar
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
