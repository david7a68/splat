---
splat_schema: 1
splat_id: 7472060e-1a44-5892-a340-780185d900b2
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_1_eldritch_invocations
splat_name: 'Level 1: Eldritch Invocations'
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70, 71
  url: https://www.dndbeyond.com/srd
---

# Level 1: Eldritch Invocations

## Source text

<!-- SRD page 70 -->

You have unearthed Eldritch Invocations, pieces of forbidden knowledge that imbue you with an abiding magical ability or other lessons. You gain one invocation of your choice, such as Pact of the Tome. Invocations are described in the “Eldritch Invocation Options” section later in this class’s description.

Prerequisites. If an invocation has a prerequisite, you must meet it to learn that invocation. For example, if an invocation requires you to be a level 5+ Warlock, you can select the invocation once you reach Warlock level 5.

Replacing and Gaining Invocations. Whenever you gain a Warlock level, you can replace one of your invocations with another one for which you qualify. You can’t replace an invocation if it’s a prerequisite for another invocation that you have.

**Warlock Features**

| Level | Proficiency Bonus | Class Features | Eldritch Invocations | Cantrips | Prepared Spells | Spell Slots | Slot Level |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | +2 | Eldritch Invocations, Pact Magic | 1 | 2 | 2 | 1 | 1 |
| 2 | +2 | Magical Cunning | 3 | 2 | 3 | 2 | 1 |
| 3 | +2 | Warlock Subclass | 3 | 2 | 4 | 2 | 2 |
| 4 | +2 | Ability Score Improvement | 3 | 3 | 5 | 2 | 2 |
| 5 | +3 | — | 5 | 3 | 6 | 2 | 3 |
| 6 | +3 | Subclass feature | 5 | 3 | 7 | 2 | 3 |
| 7 | +3 | — | 6 | 3 | 8 | 2 | 4 |
| 8 | +3 | Ability Score Improvement | 6 | 3 | 9 | 2 | 4 |
| 9 | +4 | Contact Patron | 7 | 3 | 10 | 2 | 5 |
| 10 | +4 | Subclass feature | 7 | 4 | 10 | 2 | 5 |
| 11 | +4 | Mystic Arcanum (level 6 spell) | 7 | 4 | 11 | 3 | 5 |
| 12 | +4 | Ability Score Improvement | 8 | 4 | 11 | 3 | 5 |
| 13 | +5 | Mystic Arcanum (level 7 spell) | 8 | 4 | 12 | 3 | 5 |
| 14 | +5 | Subclass feature | 8 | 4 | 12 | 3 | 5 |
| 15 | +5 | Mystic Arcanum (level 8 spell) | 9 | 4 | 13 | 3 | 5 |
| 16 | +5 | Ability Score Improvement | 9 | 4 | 13 | 3 | 5 |
| 17 | +6 | Mystic Arcanum (level 9 spell) | 9 | 4 | 14 | 4 | 5 |
| 18 | +6 | — | 10 | 4 | 14 | 4 | 5 |
| 19 | +6 | Epic Boon | 10 | 4 | 15 | 4 | 5 |
| 20 | +6 | Eldritch Master | 10 | 4 | 15 | 4 | 5 |

<!-- SRD page 71 -->

When you gain certain Warlock levels, you gain more invocations of your choice, as shown in the Invocations column of the Warlock Features table.

You can’t pick the same invocation more than once unless its description says otherwise.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warlock_level_1_eldritch_invocations ` |
| ` implementation ` | ` dnd2024:class_features_warlock_level_1_eldritch_invocations ` |

### Parameters

#### Invocation choice

<!-- splat:yaml path=splat_rule/parameters/invocation_choice -->

```yaml
source:
  key: dnd2024:rules/warlock_eldritch_invocation_options
initial_count: 1
count:
  table:
    key: dnd2024:tables/warlock_progression
  row:
    stat: classes.warlock.level
  column: eldritch_invocations
requires_prerequisites: true
repeat_only_if_option_repeatable: true
```

#### Replace

<!-- splat:values path=splat_rule/parameters/replace -->

| Field | Value |
| --- | --- |
| ` event ` | ` character.warlock_level_gained ` |
| ` maximum ` | ` 1 ` |
| ` new_requires_prerequisites ` | ` true ` |
| ` cannot_remove_if_prerequisite_of_retained_invocation ` | ` true ` |
