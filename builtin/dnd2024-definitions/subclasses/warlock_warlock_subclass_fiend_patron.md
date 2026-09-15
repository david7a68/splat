---
splat_schema: 1
splat_id: ab33196c-b0ce-5509-b8f0-134faa497055
splat_kind: rule_group
splat_key: dnd2024:subclasses/warlock_warlock_subclass_fiend_patron
splat_name: 'Warlock Subclass: Fiend Patron'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
---

# Warlock Subclass: Fiend Patron

## Source text

<!-- SRD page 76 -->

Make a Deal with the Lower Planes\
Your pact draws on the Lower Planes, the realms of\
perdition. You might forge a bargain with a demon\
lord, an archdevil, or another fiend that is especially\
mighty. That patron’s aims are evil—the corruption\
or destruction of all things, ultimately including\
you—and your path is defined by the extent to\
which you strive against those aims.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/warlock_warlock_subclass_fiend_patron ` |
| ` implementation ` | ` dnd2024:subclasses_warlock_warlock_subclass_fiend_patron ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |
| ` minimum_class_level ` | ` 3 ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | class_level |
| --- | --- |
| ` 0 ` | ` 10 ` |
| ` 1 ` | ` 14 ` |
| ` 2 ` | ` 3 ` |
| ` 3 ` | ` 3 ` |
| ` 4 ` | ` 6 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fiend_patron_level_10_fiendish_resilience ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fiend_patron_level_14_hurl_through_hell ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fiend_patron_level_3_fiend_spells ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fiend_patron_level_6_dark_one_s_own_luck ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |
