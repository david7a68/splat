---
splat_schema: 2
splat_id: a9727af5-4b2b-5b0e-a005-383c8eec0ac7
splat_kind: creature
splat_key: dnd2024:creatures/werebear
splat_name: Werebear
splat_origin: dnd2024:reference/creature/werebear
splat_sources:
- SRD 5.2.1, pp. 337, 338, https://www.dndbeyond.com/srd
---

# Werebear


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Monstrosity (Lycanthrope) |
| Alignment | Neutral Good |
| Armor Class | 15 |
| Hit Points | 135 (18d8 + 54) |
| Initiative | +3 (13) |
| Speed | 30 ft., 40 ft. (bear form only), Climb 30 ft. (bear form only) |
| Challenge | 5 (XP 1,800; PB +3) |
| Skills | Perception +7 |
| Senses | Darkvision 60 ft.; Passive Perception 17 |
| Languages | Common (can’t speak in bear form) |
| Gear | Handaxes (4) |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 10 | +0 | +0 |
| Con | 17 | +3 | +3 |
| Int | 11 | +0 | +0 |
| Wis | 12 | +1 | +1 |
| Cha | 12 | +1 | +1 |

%% splat shape=speed_max %%


## Source text

<!-- SRD page 337 -->

Medium or Small Monstrosity (Lycanthrope), Neutral Good\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 15 Initiative +3 (13)\
> HP 135 (18d8 + 54)\
> Speed 30 ft., 40 ft. (bear form only), Climb 30 ft. (bear form only)\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 19 +4 +4 Dex 10 +0 +0 Con 17 +3 +3\
> Int 11 +0 +0 Wis 12 +1 +1 Cha 12 +1 +1\
> Skills Perception +7\
> Gear Handaxes (4)\
> Senses Darkvision 60 ft.; Passive Perception 17\
> Languages Common (can’t speak in bear form)\
> CR 5 (XP 1,800; PB +3)


### Actions

Multiattack. The werebear makes two attacks, using Handaxe or Rend in any combination. It can replace one attack with a Bite attack.

Bite (Bear or Hybrid Form Only). Melee Attack Roll: +7, reach 5 ft. Hit: 17 (2d12 + 4) Piercing damage. If the target is a Humanoid, it is subjected to the following effect. Constitution Saving Throw: DC 14. Failure: The target is cursed. If the cursed target drops to 0 Hit Points, it instead becomes a Werebear under the GM’s control and has 10 Hit Points. Success: The target is immune to this werebear’s curse for 24 hours.

<!-- SRD page 338 -->

Handaxe (Humanoid or Hybrid Form Only). Melee or Ranged Attack Roll: +7, reach 5 ft or range 20/60 ft. Hit: 14 (3d6 + 4) Slashing damage.

Rend (Bear or Hybrid Form Only). Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 + 4) Slashing damage.

### Bonus Actions

Shape-Shift. The werebear shape-shifts into a Large bear-humanoid hybrid form or a Large bear, or it returns to its true humanoid form. Its game statistics, other than its size, are the same in each form. Any equipment it is wearing or carrying isn’t transformed.

Original layout: [[../../srd-5.2.1/pages/337|page 337]], [[../../srd-5.2.1/pages/338|page 338]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/werebear ` |
| ` implementation ` | ` dnd2024:creature_werebear ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 17 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 7 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | count |
| --- | --- | --- |
| ` 0 ` | ` handaxes ` | ` 4 ` |

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
| ` speech_prohibited_forms ` | ` ["bear"] ` |

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 12
      add: 4
    average: 17
- op: saving_throw
  ability: constitution
  dc: 14
  targets:
    kind: target
    creature_types:
    - humanoid
  failure:
  - op: curse.apply
    on_target_zero_hp:
      instead:
        op: stat_block.replace
        definition:
          key: dnd2024:creatures/werebear
        controller: gm
        hp: 10
  success:
  - op: feature.immunity
    source: actor
    feature: bite_curse
    duration:
      amount: 24
      unit: hour
  effect_identity: bite_curse
allowed_forms:
- bear
- hybrid
```

##### Handaxe

<!-- splat:yaml path=splat_rule/parameters/features/handaxe -->

```yaml
economy: action
op: attack
modes:
- melee
- ranged
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
range_ft:
  normal: 20
  long: 60
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 3
        sides: 6
      add: 4
    average: 14
allowed_forms:
- humanoid
- hybrid
```

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
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
        count: 2
        sides: 8
      add: 4
    average: 13
allowed_forms:
- bear
- hybrid
```

##### Shape shift

<!-- splat:values path=splat_rule/parameters/features/shape_shift -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` shapechange ` |
| ` equipment_transformed ` | ` false ` |
| ` statistics_preserved_except ` | ` ["size"] ` |

###### Choices

<!-- splat:table
path: splat_rule/parameters/features/shape_shift/choices
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
    optional: true
  Animal:
    path: animal
    type: symbol
    optional: true
-->

| Key | Form | Size | Animal |
| --- | --- | --- | --- |
| ` row_1 ` | ` Hybrid ` | ` Large ` | ` Bear ` |
| ` row_2 ` | ` Bear ` | ` Large ` |  |
| ` row_3 ` | ` True humanoid ` |  |  |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 2
  choices:
  - handaxe
  - rend
  combination: any
replace_attack:
  maximum: 1
  choices:
  - op: feature.use
    feature: bite
```

#### Form overrides

<!-- splat:yaml path=splat_rule/parameters/form_overrides -->

```yaml
bear:
  speed_ft: 40
  climb_speed_ft: 30
  can_speak: false
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> climb_speed:
>   unit: ft
>   value: 30
> ```

