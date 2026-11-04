---
splat_schema: 2
splat_id: 54122f4f-d930-54c4-83a6-391ba48a52e7
splat_kind: rule
splat_key: dnd2024:glossary/deafened_condition
splat_name: Deafened [Condition]
splat_origin: dnd2024:reference/glossary/deafened_condition
splat_sources:
- SRD 5.2.1, pp. 181, https://www.dndbeyond.com/srd
---

# Deafened [Condition]

## Source text

<!-- SRD page 181 -->

While you have the Deafened condition, you experience the following effect.

Can’t Hear. You can’t hear and automatically fail any ability check that requires hearing.

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/deafened_condition ` |
| ` implementation ` | ` dnd2024:glossary_deafened_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` deafened ` |
| ` can_hear ` | ` false ` |

#### Automatic failure

<!-- splat:values path=splat_rule/parameters/automatic_failure -->

| Field | Value |
| --- | --- |
| ` check_kind ` | ` ability_check ` |
| ` requires_hearing ` | ` true ` |
