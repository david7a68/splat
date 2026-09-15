---
splat_schema: 1
splat_id: cd5a5303-9e81-5ecc-b785-cf57fa501e3b
splat_kind: rule
splat_key: dnd2024:tables/ranger_progression
splat_name: Ranger Progression
splat_origin:
  key: dnd2024:reference/pages/058
splat_sources:
- title: SRD 5.2.1
  locator: pp. 58
  url: https://www.dndbeyond.com/srd
---

# Ranger Progression

## Source text

From SRD 5.2.1 page 58.

     Ranger Features
              Proficiency                                    Favored   Prepared    —Spell Slots per Spell Level—
      Level     Bonus       Class Features                    Enemy     Spells     1      2      3      4      5
        1         +2        Spellcasting, Favored Enemy,       2          2        2     —      —      —      —
                            Weapon Mastery
        2         +2        Deft Explorer, Fighting Style      2          3        2     —      —      —      —
        3         +2        Ranger Subclass                     2         4        3     —      —      —      —
        4         +2        Ability Score Improvement           2         5        3     —      —      —      —
        5         +3        Extra Attack                        3         6        4      2     —      —      —
        6         +3        Roving                              3         6        4      2     —      —      —
        7         +3        Subclass feature                    3         7        4      3     —      —      —
        8         +3        Ability Score Improvement           3         7        4      3     —      —      —
        9         +4        Expertise                           4         9        4      3      2     —      —
       10         +4        Tireless                            4         9        4      3      2     —      —
       11         +4        Subclass feature                    4         10       4      3      3     —      —
       12         +4        Ability Score Improvement           4         10       4      3      3     —      —
       13         +5        Relentless Hunter                   5         11       4      3      3      1     —
       14         +5        Nature’s Veil                       5         11       4      3      3      1     —
       15         +5        Subclass feature                    5         12       4      3      3      2     —
       16         +5        Ability Score Improvement           5         12       4      3      3      2     —
       17         +6        Precise Hunter                      6         14       4      3      3      3      1
       18         +6        Feral Senses                        6         14       4      3      3      3      1
       19         +6        Epic Boon                           6         15       4      3      3      3      2
       20         +6        Foe Slayer                          6         15       4      3      3      3      2


       Prepared Spells of Level 1+. You prepare the            Changing Your Prepared Spells. Whenever you
     list of level 1+ spells that are available for you to   finish a Long Rest, you can replace one spell on your
     cast with this feature. To start, choose two level 1    list with another Ranger spell for which you have
     Ranger spells. Cure Wounds and Ensnaring Strike are     spell slots.
     recommended.                                              Spellcasting Ability. Wisdom is your spellcasting
       The number of spells on your list increases as you    ability for your Ranger spells.
     gain Ranger levels, as shown in the Prepared Spells       Spellcasting Focus. You can use a Druidic Focus
     column of the Ranger Features table. Whenever that      as a Spellcasting Focus for your Ranger spells.
     number increases, choose additional Ranger spells
     until the number of spells on your list matches the     Level 1: Favored Enemy
     number in the Ranger Features table. The chosen         You always have the Hunter’s Mark spell prepared.
     spells must be of a level for which you have spell      You can cast it twice without expending a spell slot,
     slots. For example, if you’re a level 5 Ranger, your    and you regain all expended uses of this ability
     list of prepared spells can include six Ranger spells   when you finish a Long Rest.
     of level 1 or 2 in any combination.                       The number of times you can cast the spell with-
       If another Ranger feature gives you spells that       out a spell slot increases when you reach certain
     you always have prepared, those spells don’t count      Ranger levels, as shown in the Favored Enemy col-
     against the number of spells you can prepare with       umn of the Ranger Features table.
     this feature, but those spells otherwise count as       Level 1: Weapon Mastery
     Ranger spells for you.                                  Your training with weapons allows you to use the
                                                             mastery properties of two kinds of weapons of your




58   System Reference Document 5.2.1

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:tables/ranger_progression ` |
| ` implementation ` | ` dnd2024:class_progression_ranger_progression ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |

#### Rows

<!-- splat:rows path=splat_rule/parameters/rows -->

| Row | level | proficiency_bonus | features_text | favored_enemy | prepared_spells | spell_slots_1 | spell_slots_2 | spell_slots_3 | spell_slots_4 | spell_slots_5 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 2 ` | ` Spellcasting, Favored Enemy, Weapon Mastery ` | ` 2 ` | ` 2 ` | ` 2 ` | ` 0 ` | ` 0 ` | ` 0 ` | ` 0 ` |
| ` 1 ` | ` 2 ` | ` 2 ` | ` Deft Explorer, Fighting Style ` | ` 2 ` | ` 3 ` | ` 2 ` | ` 0 ` | ` 0 ` | ` 0 ` | ` 0 ` |
| ` 2 ` | ` 3 ` | ` 2 ` | ` Ranger Subclass ` | ` 2 ` | ` 4 ` | ` 3 ` | ` 0 ` | ` 0 ` | ` 0 ` | ` 0 ` |
| ` 3 ` | ` 4 ` | ` 2 ` | ` Ability Score Improvement ` | ` 2 ` | ` 5 ` | ` 3 ` | ` 0 ` | ` 0 ` | ` 0 ` | ` 0 ` |
| ` 4 ` | ` 5 ` | ` 3 ` | ` Extra Attack ` | ` 3 ` | ` 6 ` | ` 4 ` | ` 2 ` | ` 0 ` | ` 0 ` | ` 0 ` |
| ` 5 ` | ` 6 ` | ` 3 ` | ` Roving ` | ` 3 ` | ` 6 ` | ` 4 ` | ` 2 ` | ` 0 ` | ` 0 ` | ` 0 ` |
| ` 6 ` | ` 7 ` | ` 3 ` | ` Subclass feature ` | ` 3 ` | ` 7 ` | ` 4 ` | ` 3 ` | ` 0 ` | ` 0 ` | ` 0 ` |
| ` 7 ` | ` 8 ` | ` 3 ` | ` Ability Score Improvement ` | ` 3 ` | ` 7 ` | ` 4 ` | ` 3 ` | ` 0 ` | ` 0 ` | ` 0 ` |
| ` 8 ` | ` 9 ` | ` 4 ` | ` Expertise ` | ` 4 ` | ` 9 ` | ` 4 ` | ` 3 ` | ` 2 ` | ` 0 ` | ` 0 ` |
| ` 9 ` | ` 10 ` | ` 4 ` | ` Tireless ` | ` 4 ` | ` 9 ` | ` 4 ` | ` 3 ` | ` 2 ` | ` 0 ` | ` 0 ` |
| ` 10 ` | ` 11 ` | ` 4 ` | ` Subclass feature ` | ` 4 ` | ` 10 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 0 ` | ` 0 ` |
| ` 11 ` | ` 12 ` | ` 4 ` | ` Ability Score Improvement ` | ` 4 ` | ` 10 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 0 ` | ` 0 ` |
| ` 12 ` | ` 13 ` | ` 5 ` | ` Relentless Hunter ` | ` 5 ` | ` 11 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 1 ` | ` 0 ` |
| ` 13 ` | ` 14 ` | ` 5 ` | ` Nature’s Veil ` | ` 5 ` | ` 11 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 1 ` | ` 0 ` |
| ` 14 ` | ` 15 ` | ` 5 ` | ` Subclass feature ` | ` 5 ` | ` 12 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 2 ` | ` 0 ` |
| ` 15 ` | ` 16 ` | ` 5 ` | ` Ability Score Improvement ` | ` 5 ` | ` 12 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 2 ` | ` 0 ` |
| ` 16 ` | ` 17 ` | ` 6 ` | ` Precise Hunter ` | ` 6 ` | ` 14 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 3 ` | ` 1 ` |
| ` 17 ` | ` 18 ` | ` 6 ` | ` Feral Senses ` | ` 6 ` | ` 14 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 3 ` | ` 1 ` |
| ` 18 ` | ` 19 ` | ` 6 ` | ` Epic Boon ` | ` 6 ` | ` 15 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 3 ` | ` 2 ` |
| ` 19 ` | ` 20 ` | ` 6 ` | ` Foe Slayer ` | ` 6 ` | ` 15 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 3 ` | ` 2 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/0/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 1 ` |
| ` 2 ` | ` grant_feature ` | ` 1 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_1_spellcasting ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_1_favored_enemy ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_1_weapon_mastery ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/1/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 2 ` |
| ` 1 ` | ` grant_feature ` | ` 2 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/1/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_2_deft_explorer ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/1/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_2_fighting_style ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/2/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` choose_subclass ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/2/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_3_ranger_subclass ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/3/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 4 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/3/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/4/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 5 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/4/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_5_extra_attack ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/5/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 6 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/5/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_6_roving ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/6/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 7 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/7/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 8 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/7/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/8/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 9 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/8/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_9_expertise ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/9/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 10 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/9/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_10_tireless ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/10/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 11 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/11/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 12 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/11/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/12/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 13 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/12/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_13_relentless_hunter ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/13/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 14 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/13/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_14_nature_s_veil ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/14/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 15 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/15/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 16 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/15/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/16/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 17 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/16/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_17_precise_hunter ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/17/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 18 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/17/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_18_feral_senses ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/18/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 19 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/18/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_19_epic_boon ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/19/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 20 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/19/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_20_foe_slayer ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/ranger ` |

#### Lookup

<!-- splat:values path=splat_rule/parameters/lookup -->

| Field | Value |
| --- | --- |
| ` input ` | ` class_level ` |
| ` minimum ` | ` 1 ` |
| ` maximum ` | ` 20 ` |
| ` match ` | ` exact ` |
| ` numeric_columns ` | ` current_capacity_or_value ` |
| ` feature_grants ` | ` all_crossed_levels ` |
| ` repeat_application ` | ` idempotent_per_class_level_and_feature ` |

#### Multiclass

<!-- splat:yaml path=splat_rule/parameters/multiclass -->

```yaml
proficiency_bonus_source: total_character_level
spell_slot_override_rule:
  key: dnd2024:rules/multiclassing_spellcasting
pact_magic_pool_separate: true
```

#### Column semantics

<!-- splat:yaml path=splat_rule/parameters/column_semantics -->

```yaml
favored_enemy:
  kind: resource_capacity
  resource: favored_enemy
  recovery: owning_feature_contract
  level_change_refills: false
prepared_spells:
  kind: selection_capacity
  selection: prepared_spells
  eligibility: owning_feature_contract
proficiency_bonus:
  kind: bonus
  applies_to: proficiency
  use_class_row_only_if_single_class: true
spell_slots_1:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 1
  recovery: long_rest
spell_slots_2:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 2
  recovery: long_rest
spell_slots_3:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 3
  recovery: long_rest
spell_slots_4:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 4
  recovery: long_rest
spell_slots_5:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 5
  recovery: long_rest
```
