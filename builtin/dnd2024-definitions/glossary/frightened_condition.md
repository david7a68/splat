---
splat_schema: 1
splat_id: 648d48b0-27a5-58ed-bd03-9a3d9b258460
splat_kind: rule
splat_key: dnd2024:glossary/frightened_condition
splat_name: Frightened [Condition]
splat_origin:
  key: dnd2024:reference/glossary/frightened_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
---

# Frightened [Condition]

## Source text

<!-- SRD page 182 -->

While you have the Frightened condition, you expe-\
rience the following effects.\
Ability Checks and Attacks Affected. You have\
Disadvantage on ability checks and attack rolls\
while the source of fear is within line of sight.\
Can’t Approach. You can’t willingly move closer to\
the source of fear.\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/frightened_condition ` |
| ` implementation ` | ` dnd2024:glossary_frightened_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` frightened ` |
| ` willing_movement_closer_to_source_allowed ` | ` false ` |

#### Source in line of sight

<!-- splat:values path=splat_rule/parameters/source_in_line_of_sight -->

| Field | Value |
| --- | --- |
| ` ability_checks ` | ` disadvantage ` |
| ` attack_rolls ` | ` disadvantage ` |
