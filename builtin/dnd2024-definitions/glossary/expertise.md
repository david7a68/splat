---
splat_schema: 1
splat_id: 85bb3099-8f3a-5296-b590-eb7ed9e2d595
splat_kind: rule
splat_key: dnd2024:glossary/expertise
splat_name: Expertise
splat_origin:
  key: dnd2024:reference/glossary/expertise
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
---

# Expertise

## Source text

<!-- SRD page 182 -->

Expertise is a feature that enhances your use of a\
skill proficiency. When you make an ability check\
with a skill proficiency in which you have Expertise,\
your Proficiency Bonus is doubled for that check un-\
less the bonus is doubled by another feature.\
If you gain Expertise, you gain it in one skill in\
which you have proficiency. You can’t have Exper-\
tise in the same skill proficiency more than once.\
See also “Playing the Game” (“Proficiency”).\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/expertise ` |
| ` implementation ` | ` dnd2024:glossary_expertise ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` proficiency_bonus_multiplier ` | ` 2 ` |
| ` scope ` | ` ability_check_with_selected_skill ` |
| ` stacks_with_other_doubling ` | ` false ` |
| ` same_skill_multiple_expertise ` | ` false ` |

#### Selection

<!-- splat:values path=splat_rule/parameters/selection -->

| Field | Value |
| --- | --- |
| ` skill_count ` | ` 1 ` |
| ` requires_skill_proficiency ` | ` true ` |
| ` existing_expertise_disallowed ` | ` true ` |
