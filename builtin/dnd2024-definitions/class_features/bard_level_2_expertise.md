---
splat_schema: 1
splat_id: 2133ca17-23c2-5d11-bfa7-3f6a5a89856a
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_2_expertise
splat_name: 'Level 2: Expertise'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
---

# Level 2: Expertise

## Source text

<!-- SRD page 32 -->

You gain Expertise (see “Rules Glossary”) in two of your skill proficiencies of your choice. Performance and Persuasion are recommended if you have proficiency in them.

At Bard level 9, you gain Expertise in two more of your skill proficiencies of your choice.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_2_expertise ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_2_expertise ` |

### Parameters

#### Expertise grants

<!-- splat:table
path: splat_rule/parameters/expertise_grants
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Bard level:
    path: bard_level
    type: integer
  Count:
    path: count
    type: integer
  Eligible proficient:
    path: eligible/proficient
    type: boolean
  Eligible expertise:
    path: eligible/expertise
    type: boolean
  Grant:
    path: grant
    type: symbol
-->

| Key | Bard level | Count | Eligible proficient | Eligible expertise | Grant |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 2 ` | ` 2 ` | true | false | ` Expertise ` |
| ` row_2 ` | ` 9 ` | ` 2 ` | true | false | ` Expertise ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` minimum_level ` | ` 2 ` |
