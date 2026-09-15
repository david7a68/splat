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

You gain Expertise (see “Rules Glossary”) in two of\
your skill proficiencies of your choice. Performance\
and Persuasion are recommended if you have profi-\
ciency in them.\
At Bard level 9, you gain Expertise in two more of\
your skill proficiencies of your choice.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_2_expertise ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_2_expertise ` |

### Parameters

#### Expertise grants

<!-- splat:rows path=splat_rule/parameters/expertise_grants -->

| Row | bard_level | count | grant |
| --- | --- | --- | --- |
| ` 0 ` | ` 2 ` | ` 2 ` | ` expertise ` |
| ` 1 ` | ` 9 ` | ` 2 ` | ` expertise ` |

##### Eligible

<!-- splat:values path=splat_rule/parameters/expertise_grants/0/eligible -->

| Field | Value |
| --- | --- |
| ` proficient ` | ` true ` |
| ` expertise ` | ` false ` |

##### Eligible

<!-- splat:values path=splat_rule/parameters/expertise_grants/1/eligible -->

| Field | Value |
| --- | --- |
| ` proficient ` | ` true ` |
| ` expertise ` | ` false ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` minimum_level ` | ` 2 ` |
