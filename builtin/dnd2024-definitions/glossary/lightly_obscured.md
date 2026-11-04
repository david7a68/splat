---
splat_schema: 2
splat_id: 37c01dc0-bb92-5edb-b32f-c67cddbdcacc
splat_kind: rule
splat_key: dnd2024:glossary/lightly_obscured
splat_name: Lightly Obscured
splat_origin: dnd2024:reference/glossary/lightly_obscured
splat_sources:
- SRD 5.2.1, pp. 184, https://www.dndbeyond.com/srd
---

# Lightly Obscured

## Source text

<!-- SRD page 184 -->

You have Disadvantage on Wisdom (Perception) checks to see something in a Lightly Obscured space. See also “Dim Light” and “Playing the Game” (“Exploration”).

Original layout: [[../../srd-5.2.1/pages/184|page 184]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/lightly_obscured ` |
| ` implementation ` | ` dnd2024:glossary_lightly_obscured ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` target_in_lightly_obscured_space ` | ` true ` |
| ` roll_mode ` | ` disadvantage ` |

#### Check

<!-- splat:values path=splat_rule/parameters/check -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` perception ` |
| ` requires_sight ` | ` true ` |
