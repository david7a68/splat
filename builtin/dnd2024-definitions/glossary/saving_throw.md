---
splat_schema: 1
splat_id: a6b4c0c6-56b5-5ae7-8916-92358e545e95
splat_kind: rule
splat_key: dnd2024:glossary/saving_throw
splat_name: Saving Throw
splat_origin:
  key: dnd2024:reference/glossary/saving_throw
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
---

# Saving Throw

## Source text

<!-- SRD page 187 -->

A saving throw—also called a save—represents an attempt to avoid or resist a threat. You normally make a saving throw only when a rule requires you to do so, but you can decide to fail the save without rolling. The result of a save is detailed in the effect that allowed it. If a target is forced to make a save and lacks the ability score used by it, the target automatically fails. See also “Playing the Game” (“D20 Tests”).

Original layout: [[../../srd-5.2.1/pages/187|page 187]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/saving_throw ` |
| ` implementation ` | ` dnd2024:glossary_saving_throw ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` normally_requires_rule ` | ` true ` |
| ` missing_relevant_ability_score ` | ` automatic_failure ` |
| ` result_effects ` | ` requiring_effect_defined ` |

#### Voluntary failure

<!-- splat:values path=splat_rule/parameters/voluntary_failure -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` subject ` |
| ` rolling_required ` | ` false ` |
