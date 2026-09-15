---
splat_schema: 1
splat_id: 0da660ab-67e1-52d2-a3f6-5656c40c6fd8
splat_kind: creature
splat_key: dnd2024:creatures/weretiger
splat_name: Weretiger
splat_origin:
  key: dnd2024:reference/creature/weretiger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 339
  url: https://www.dndbeyond.com/srd
---

# Weretiger

## Source text

<!-- SRD page 339 -->

Medium or Small Monstrosity (Lycanthrope), Neutral\
AC 12 Initiative +2 (12)\
HP 120 (16d8 + 48)\
Speed 30 ft., 40 ft. (tiger form only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 15 +2 +2 Con 16 +3 +3\
Int 10 +0 +0 Wis 13 +1 +1 Cha 11 +0 +0\
Skills Perception +5, Stealth +4\
Gear Longbow\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages Common (can’t speak in tiger form)\
CR 4 (XP 1,100; PB +2)\

### Actions

Multiattack. The weretiger makes two attacks, using\
Scratch or Longbow in any combination. It can replace\
one attack with a Bite attack.\
Bite (Tiger or Hybrid Form Only). Melee Attack Roll:\
+5, reach 5 ft. Hit: 12 (2d8 + 3) Piercing damage. If\
the target is a Humanoid, it is subjected to the follow-\
ing effect. Constitution Saving Throw: DC 13. Failure:\
The target is cursed. If the cursed target drops to 0 Hit\
Points, it instead becomes a Weretiger under the GM’s\
control and has 10 Hit Points. Success: The target is im-\
mune to this weretiger’s curse for 24 hours.\
Scratch. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Slashing damage.\
Longbow (Humanoid or Hybrid Form Only). Ranged\
Attack Roll: +4, range 150/600 ft. Hit: 11 (2d8 + 2)\
Piercing damage.\

### Bonus Actions

Prowl (Tiger or Hybrid Form Only). The weretiger\
moves up to its Speed without provoking Opportunity\
Attacks. At the end of this movement, the weretiger can\
take the Hide action.\
Shape-Shift. The weretiger shape-shifts into a Large\
tiger-humanoid hybrid or a Large tiger, or it returns to\
its true humanoid form. Its game statistics, other than\
its size, are the same in each form. Any equipment it is\
wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/339|page 339]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` tags_text ` | ` (Lycanthrope) ` |
| ` movement_text ` | ` 30 ft., 40 ft. (tiger form only) ` |
| ` skills_text ` | ` Perception +5, Stealth +4 ` |
| ` gear_text ` | ` Longbow ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 15 ` |
| ` languages_text ` | ` Common (can’t speak in tiger form) ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 120 (16d8 + 48) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 17 ` | ` 3 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 11 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 120 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/weretiger ` |
| ` implementation ` | ` dnd2024:creature_weretiger ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 4 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | count |
| --- | --- | --- |
| ` 0 ` | ` longbow ` | ` 1 ` |

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
| ` speech_prohibited_forms ` | ` ["tiger"] ` |

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 8
      add: 3
    average: 12
- op: saving_throw
  ability: constitution
  dc: 13
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
          key: dnd2024:creatures/weretiger
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
- tiger
- hybrid
```

##### Scratch

<!-- splat:yaml path=splat_rule/parameters/features/scratch -->

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
  - type: slashing
    amount:
      dice:
        count: 2
        sides: 6
      add: 3
    average: 10
```

##### Longbow

<!-- splat:yaml path=splat_rule/parameters/features/longbow -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 150
  long: 600
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 8
      add: 2
    average: 11
allowed_forms:
- humanoid
- hybrid
```

##### Prowl

<!-- splat:yaml path=splat_rule/parameters/features/prowl -->

```yaml
economy: bonus_action
allowed_forms:
- tiger
- hybrid
op: sequence
steps:
- op: movement
  maximum:
    multiply:
    - 1
    - actor_stat: speed
  opportunity_attacks: false
- op: action.use
  action: hide
  optional: true
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
| ` row_1 ` | ` Hybrid ` | ` Large ` | ` Tiger ` |
| ` row_2 ` | ` Tiger ` | ` Large ` |  |
| ` row_3 ` | ` True humanoid ` |  |  |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
attacks:
  count: 2
  choices:
  - scratch
  - longbow
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
tiger:
  speed_ft: 40
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
