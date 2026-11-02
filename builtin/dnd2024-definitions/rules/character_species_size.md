---
splat_schema: 1
splat_id: 04416556-8bb8-5721-876a-6c88dbbfe67a
splat_kind: rule
splat_key: dnd2024:rules/character_species_size
splat_name: Size
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83, 84
  url: https://www.dndbeyond.com/srd
---

# Size

## Source text

<!-- SRD page 83 -->

Your character’s species determines the character’s size. Individuals within a species cover a wide range of heights, and some species include such diversity of size that you can choose whether your character is Small or Medium.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/character_species_size ` |
| ` implementation ` | ` dnd2024:rules_character_species_size ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` size_source ` | ` species_definition ` |

#### If species allows size choice

<!-- splat:values path=splat_rule/parameters/if_species_allows_size_choice -->

| Field | Value |
| --- | --- |
| ` choose_count ` | ` 1 ` |
| ` options ` | ` ["small", "medium"] ` |
