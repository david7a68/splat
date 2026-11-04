---
splat_schema: 2
splat_id: 7552d5cf-8cfc-5cba-b222-925d7d0af491
splat_kind: rule
splat_key: dnd2024:rules/exploration_obscured_areas
splat_name: Obscured Areas
splat_origin: dnd2024:reference/pages/011
splat_sources:
- SRD 5.2.1, pp. 11, https://www.dndbeyond.com/srd
---

# Obscured Areas

## Source text

<!-- SRD page 11 -->

An area might be Lightly or Heavily Obscured. In a Lightly Obscured area—such as an area with Dim Light, patchy fog, or moderate foliage—you have Disadvantage on Wisdom (Perception) checks that rely on sight.

A Heavily Obscured area—such as an area with Darkness, heavy fog, or dense foliage—is opaque. You have the Blinded condition (see “Rules Glossary”) when trying to see something there.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_obscured_areas ` |
| ` implementation ` | ` dnd2024:rule_exploration_obscured_areas ` |

### Parameters

#### Lightly obscured

<!-- splat:values path=splat_rule/parameters/lightly_obscured -->

| Field | Value |
| --- | --- |
| ` sight_based_wisdom_perception ` | ` disadvantage ` |

#### Heavily obscured

<!-- splat:values path=splat_rule/parameters/heavily_obscured -->

| Field | Value |
| --- | --- |
| ` opaque ` | ` true ` |
| ` condition_when_attempting_to_see_inside ` | ` blinded ` |
