---
splat_schema: 1
splat_id: 9cc03f70-bcf5-54c3-a203-39a4bde2c3ac
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_1_expertise
splat_name: 'Level 1: Expertise'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
---

# Level 1: Expertise

## Source text

<!-- SRD page 61 -->

You gain Expertise in two of your skill proficiencies\
of your choice. Sleight of Hand and Stealth are rec-\
ommended if you have proficiency in them.\
At Rogue level 6, you gain Expertise in two more\
of your skill proficiencies of your choice.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_1_expertise ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_1_expertise ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` eligibility ` | ` own_skill_proficiencies ` |

#### Expertise choices

<!-- splat:rows path=splat_rule/parameters/expertise_choices -->

| Row | rogue_level | count |
| --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 2 ` |
| ` 1 ` | ` 6 ` | ` 2 ` |
