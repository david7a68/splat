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

You have unearthed Eldritch Invocations, pieces of\
forbidden knowledge that imbue you with an abid-\
ing magical ability or other lessons. You gain one in-\
vocation of your choice, such as Pact of the Tome. In-\
vocations are described in the “Eldritch Invocation\
Options” section later in this class’s description.\
Prerequisites. If an invocation has a prerequi-\
site, you must meet it to learn that invocation. For\
example, if an invocation requires you to be a level\
5+ Warlock, you can select the invocation once you\
reach Warlock level 5.\
Replacing and Gaining Invocations. Whenever\
you gain a Warlock level, you can replace one of\
your invocations with another one for which you\

<!-- SRD page 71 -->

Warlock Features\
Proficiency\
Level Bonus Class Features\
1 +2 Eldritch Invocations, Pact Magic\
2 +2 Magical Cunning\
3 +2 Warlock Subclass\
4 +2 Ability Score Improvement\
5 +3 —\
6 +3 Subclass feature\
7 +3 —\
8 +3 Ability Score Improvement\
9 +4 Contact Patron\
10 +4 Subclass feature\
11 +4 Mystic Arcanum (level 6 spell)\
12 +4 Ability Score Improvement\
13 +5 Mystic Arcanum (level 7 spell)\
14 +5 Subclass feature\
15 +5 Mystic Arcanum (level 8 spell)\
16 +5 Ability Score Improvement\
17 +6 Mystic Arcanum (level 9 spell)\
18 +6 —\
19 +6 Epic Boon\
20 +6 Eldritch Master\
qualify. You can’t replace an invocation if it’s a pre-\
requisite for another invocation that you have.\
When you gain certain Warlock levels, you gain\
more invocations of your choice, as shown in the In-\
vocations column of the Warlock Features table.\
You can’t pick the same invocation more than once\
unless its description says otherwise.\

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
