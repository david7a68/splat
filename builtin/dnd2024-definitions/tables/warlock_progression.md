---
splat_schema: 1
splat_id: ff00ca23-2b98-5663-bd18-04b6c3208cd5
splat_kind: rule
splat_key: dnd2024:tables/warlock_progression
splat_name: Warlock Progression
splat_origin:
  key: dnd2024:reference/pages/071
splat_sources:
- title: SRD 5.2.1
  locator: pp. 71
  url: https://www.dndbeyond.com/srd
---

# Warlock Progression

## Source text

From SRD 5.2.1 page 71.

     Warlock Features
            Proficiency                                          Eldritch               Prepared     Spell     Slot
      Level Bonus       Class Features                         Invocations   Cantrips    Spells      Slots    Level
        1         +2      Eldritch Invocations, Pact Magic         1            2          2           1        1
        2         +2      Magical Cunning                          3            2          3           2        1
        3         +2      Warlock Subclass                         3            2          4           2        2
        4         +2      Ability Score Improvement                3            3          5           2        2
        5         +3      —                                        5            3          6           2        3
        6         +3      Subclass feature                         5            3          7           2        3
        7         +3      —                                        6            3          8           2        4
        8         +3      Ability Score Improvement                6            3          9           2        4
        9         +4      Contact Patron                           7            3          10          2        5
       10         +4      Subclass feature                         7            4          10          2        5
       11         +4      Mystic Arcanum (level 6 spell)           7            4          11          3        5
       12         +4      Ability Score Improvement                8            4          11          3        5
       13         +5      Mystic Arcanum (level 7 spell)           8            4          12          3        5
       14         +5      Subclass feature                         8            4          12          3        5
       15         +5      Mystic Arcanum (level 8 spell)           9            4          13          3        5
       16         +5      Ability Score Improvement                9            4          13          3        5
       17         +6      Mystic Arcanum (level 9 spell)           9            4          14          4        5
       18         +6      —                                        10           4          14          4        5
       19         +6      Epic Boon                                10           4          15          4        5
       20         +6      Eldritch Master                          10           4          15          4        5


     qualify. You can’t replace an invocation if it’s a pre-     Spell Slots. The Warlock Features table shows
     requisite for another invocation that you have.           how many spell slots you have to cast your Warlock
       When you gain certain Warlock levels, you gain          spells of levels 1–5. The table also shows the level
     more invocations of your choice, as shown in the In-      of those slots, all of which are the same level. You
     vocations column of the Warlock Features table.           regain all expended Pact Magic spell slots when you
       You can’t pick the same invocation more than once       finish a Short or Long Rest.
     unless its description says otherwise.                      For example, when you’re a level 5 Warlock, you
                                                               have two level 3 spell slots. To cast the level 1 spell
     Level 1: Pact Magic                                       Charm Person, you must spend one of those slots,
     Through occult ceremony, you have formed a pact           and you cast it as a level 3 spell.
     with a mysterious entity to gain magical powers.            Prepared Spells of Level 1+. You prepare the
     The entity is a voice in the shadows—its identity         list of level 1+ spells that are available for you
     unclear—but its boon to you is concrete: the ability      to cast with this feature. To start, choose two
     to cast spells. See “Spells” for the rules on spell-      level 1 Warlock spells. Charm Person and Hex are
     casting. The information below details how you use        recommended.
     those rules with Warlock spells, which appear in the        The number of spells on your list increases as you
     Warlock spell list later in the class’s description.      gain Warlock levels, as shown in the Prepared Spells
       Cantrips. You know two Warlock cantrips of your         column of the Warlock Features table. Whenever
     choice. Eldritch Blast and Prestidigitation are rec-      that number increases, choose additional War-
     ommended. Whenever you gain a Warlock level, you          lock spells until the number of spells on your list
     can replace one of your cantrips from this feature        matches the number in the table. The chosen spells
     with another Warlock cantrip of your choice.              must be of a level no higher than what’s shown in
       When you reach Warlock levels 4 and 10, you             the table’s Slot Level column for your level. When
     learn another Warlock cantrip of your choice, as          you reach level 6, for example, you learn a new War-
     shown in the Cantrips column of the Warlock Fea-          lock spell, which can be of levels 1–3.
     tures table.                                                If another Warlock feature gives you spells that
                                                               you always have prepared, those spells don’t count


71   System Reference Document 5.2.1

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:tables/warlock_progression ` |
| ` implementation ` | ` dnd2024:class_progression_warlock_progression ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |

#### Rows

<!-- splat:rows path=splat_rule/parameters/rows -->

| Row | level | proficiency_bonus | features_text | eldritch_invocations | cantrips | prepared_spells | pact_slots | pact_slot_level |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 2 ` | ` Eldritch Invocations, Pact Magic ` | ` 1 ` | ` 2 ` | ` 2 ` | ` 1 ` | ` 1 ` |
| ` 1 ` | ` 2 ` | ` 2 ` | ` Magical Cunning ` | ` 3 ` | ` 2 ` | ` 3 ` | ` 2 ` | ` 1 ` |
| ` 2 ` | ` 3 ` | ` 2 ` | ` Warlock Subclass ` | ` 3 ` | ` 2 ` | ` 4 ` | ` 2 ` | ` 2 ` |
| ` 3 ` | ` 4 ` | ` 2 ` | ` Ability Score Improvement ` | ` 3 ` | ` 3 ` | ` 5 ` | ` 2 ` | ` 2 ` |
| ` 4 ` | ` 5 ` | ` 3 ` | ` — ` | ` 5 ` | ` 3 ` | ` 6 ` | ` 2 ` | ` 3 ` |
| ` 5 ` | ` 6 ` | ` 3 ` | ` Subclass feature ` | ` 5 ` | ` 3 ` | ` 7 ` | ` 2 ` | ` 3 ` |
| ` 6 ` | ` 7 ` | ` 3 ` | ` — ` | ` 6 ` | ` 3 ` | ` 8 ` | ` 2 ` | ` 4 ` |
| ` 7 ` | ` 8 ` | ` 3 ` | ` Ability Score Improvement ` | ` 6 ` | ` 3 ` | ` 9 ` | ` 2 ` | ` 4 ` |
| ` 8 ` | ` 9 ` | ` 4 ` | ` Contact Patron ` | ` 7 ` | ` 3 ` | ` 10 ` | ` 2 ` | ` 5 ` |
| ` 9 ` | ` 10 ` | ` 4 ` | ` Subclass feature ` | ` 7 ` | ` 4 ` | ` 10 ` | ` 2 ` | ` 5 ` |
| ` 10 ` | ` 11 ` | ` 4 ` | ` Mystic Arcanum (level 6 spell) ` | ` 7 ` | ` 4 ` | ` 11 ` | ` 3 ` | ` 5 ` |
| ` 11 ` | ` 12 ` | ` 4 ` | ` Ability Score Improvement ` | ` 8 ` | ` 4 ` | ` 11 ` | ` 3 ` | ` 5 ` |
| ` 12 ` | ` 13 ` | ` 5 ` | ` Mystic Arcanum (level 7 spell) ` | ` 8 ` | ` 4 ` | ` 12 ` | ` 3 ` | ` 5 ` |
| ` 13 ` | ` 14 ` | ` 5 ` | ` Subclass feature ` | ` 8 ` | ` 4 ` | ` 12 ` | ` 3 ` | ` 5 ` |
| ` 14 ` | ` 15 ` | ` 5 ` | ` Mystic Arcanum (level 8 spell) ` | ` 9 ` | ` 4 ` | ` 13 ` | ` 3 ` | ` 5 ` |
| ` 15 ` | ` 16 ` | ` 5 ` | ` Ability Score Improvement ` | ` 9 ` | ` 4 ` | ` 13 ` | ` 3 ` | ` 5 ` |
| ` 16 ` | ` 17 ` | ` 6 ` | ` Mystic Arcanum (level 9 spell) ` | ` 9 ` | ` 4 ` | ` 14 ` | ` 4 ` | ` 5 ` |
| ` 17 ` | ` 18 ` | ` 6 ` | ` — ` | ` 10 ` | ` 4 ` | ` 14 ` | ` 4 ` | ` 5 ` |
| ` 18 ` | ` 19 ` | ` 6 ` | ` Epic Boon ` | ` 10 ` | ` 4 ` | ` 15 ` | ` 4 ` | ` 5 ` |
| ` 19 ` | ` 20 ` | ` 6 ` | ` Eldritch Master ` | ` 10 ` | ` 4 ` | ` 15 ` | ` 4 ` | ` 5 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/0/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 1 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_1_eldritch_invocations ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_1_pact_magic ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/1/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 2 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/1/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_2_magical_cunning ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/2/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` choose_subclass ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/2/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_3_warlock_subclass ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/3/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 4 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/3/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:yaml path=splat_rule/parameters/rows/4/grants -->

```yaml
[]
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/5/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 6 ` |

##### Grants

<!-- splat:yaml path=splat_rule/parameters/rows/6/grants -->

```yaml
[]
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/7/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 8 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/7/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/8/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 9 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/8/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_9_contact_patron ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/9/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 10 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/10/grants -->

| Row | operation | at_class_level | arcanum_spell_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 11 ` | ` 6 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/10/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/11/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 12 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/11/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/12/grants -->

| Row | operation | at_class_level | arcanum_spell_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 13 ` | ` 7 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/12/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/13/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 14 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/14/grants -->

| Row | operation | at_class_level | arcanum_spell_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 15 ` | ` 8 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/14/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/15/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 16 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/15/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/16/grants -->

| Row | operation | at_class_level | arcanum_spell_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 17 ` | ` 9 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/16/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |

##### Grants

<!-- splat:yaml path=splat_rule/parameters/rows/17/grants -->

```yaml
[]
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/18/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 19 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/18/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_19_epic_boon ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/19/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 20 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/19/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_20_eldritch_master ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/warlock ` |

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
cantrips:
  kind: selection_capacity
  selection: cantrips
  eligibility: owning_feature_contract
eldritch_invocations:
  kind: selection_capacity
  selection: eldritch_invocations
  eligibility: owning_feature_contract
pact_slot_level:
  kind: spell_level
  applies_to: pact_slots
pact_slots:
  kind: resource_capacity
  resource: pact_slots
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
```
