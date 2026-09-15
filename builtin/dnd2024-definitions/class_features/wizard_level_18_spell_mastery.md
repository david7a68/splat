---
splat_schema: 1
splat_id: 8d2200b6-0421-535c-bf26-c7b29fc62227
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_18_spell_mastery
splat_name: 'Level 18: Spell Mastery'
splat_origin:
  key: dnd2024:reference/pages/079
splat_sources:
- title: SRD 5.2.1
  locator: pp. 79
  url: https://www.dndbeyond.com/srd
---

# Level 18: Spell Mastery

## Source text

<!-- SRD page 79 -->

You have achieved such mastery over certain spells\
that you can cast them at will. Choose a level 1 and\
a level 2 spell in your spellbook that have a casting\
time of an action. You always have those spells pre-\
pared, and you can cast them at their lowest level\
without expending a spell slot. To cast either spell at\
a higher level, you must expend a spell slot.\
Whenever you finish a Long Rest, you can study\
your spellbook and replace one of those spells with\
an eligible spell of the same level from the book.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/wizard_level_18_spell_mastery ` |
| ` implementation ` | ` dnd2024:class_features_wizard_level_18_spell_mastery ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` always_prepared ` | ` true ` |
| ` counts_against_prepared_limit ` | ` false ` |

#### Choose spells

<!-- splat:table
path: splat_rule/parameters/choose_spells
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Count:
    path: count
    type: integer
  Level:
    path: level
    type: integer
-->

| Key | Count | Level |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 1 ` |
| ` row_2 ` | ` 1 ` | ` 2 ` |

#### Eligibility

<!-- splat:values path=splat_rule/parameters/eligibility -->

| Field | Value |
| --- | --- |
| ` in_spellbook ` | ` true ` |
| ` casting_time ` | ` action ` |

#### Free cast

<!-- splat:values path=splat_rule/parameters/free_cast -->

| Field | Value |
| --- | --- |
| ` at_will ` | ` true ` |
| ` cast_level ` | ` base_spell_level ` |
| ` expend_slot ` | ` false ` |

#### Higher level casting

<!-- splat:values path=splat_rule/parameters/higher_level_casting -->

| Field | Value |
| --- | --- |
| ` expend_slot ` | ` true ` |

#### Replace

<!-- splat:values path=splat_rule/parameters/replace -->

| Field | Value |
| --- | --- |
| ` event ` | ` rest.long_completed ` |
| ` requires ` | ` study_spellbook ` |
| ` maximum ` | ` 1 ` |
| ` same_level ` | ` true ` |
| ` same_eligibility ` | ` true ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` wizard ` |
| ` minimum_level ` | ` 18 ` |
