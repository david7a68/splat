---
splat_schema: 2
splat_id: 817b77a6-8def-514e-b530-416e9ede4a56
splat_kind: rule
splat_key: dnd2024:rules/magic_items_multiple_items_of_the_same_kind
splat_name: Multiple Items of the Same Kind
splat_origin: dnd2024:reference/pages/103
splat_sources:
- SRD 5.2.1, pp. 103, https://www.dndbeyond.com/srd
---

# Multiple Items of the Same Kind

## Source text

<!-- SRD page 103 -->

You can’t wear more than one of certain magic items. You can’t normally wear more than one pair of footwear, one pair of gloves or gauntlets, one pair of bracers, one suit of armor, one item of headwear, or one cloak. The GM might make exceptions.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_items_multiple_items_of_the_same_kind ` |
| ` implementation ` | ` dnd2024:rule_magic_items_multiple_items_of_the_same_kind ` |

### Parameters

#### Worn category limits

<!-- splat:values path=splat_rule/parameters/worn_category_limits -->

| Field | Value |
| --- | --- |
| ` footwear_pair ` | ` 1 ` |
| ` gloves_or_gauntlets_pair ` | ` 1 ` |
| ` bracers_pair ` | ` 1 ` |
| ` armor_suit ` | ` 1 ` |
| ` headwear ` | ` 1 ` |
| ` cloak ` | ` 1 ` |

#### Exceptions

<!-- splat:values path=splat_rule/parameters/exceptions -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` scope ` | ` wearing_multiple_items ` |
