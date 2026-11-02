---
splat_schema: 1
splat_id: 6cae251e-c8df-5f4c-b400-af77014d3027
splat_kind: rule
splat_key: dnd2024:glossary/blinded_condition
splat_name: Blinded [Condition]
splat_origin:
  key: dnd2024:reference/glossary/blinded_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
---

# Blinded [Condition]

## Source text

<!-- SRD page 177 -->

While you have the Blinded condition, you experience the following effects.

Can’t See. You can’t see and automatically fail any ability check that requires sight.

Attacks Affected. Attack rolls against you have Advantage, and your attack rolls have Disadvantage.

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/blinded_condition ` |
| ` implementation ` | ` dnd2024:glossary_blinded_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` blinded ` |
| ` can_see ` | ` false ` |
| ` attack_rolls_against ` | ` advantage ` |
| ` own_attack_rolls ` | ` disadvantage ` |

#### Automatic failure

<!-- splat:values path=splat_rule/parameters/automatic_failure -->

| Field | Value |
| --- | --- |
| ` check_kind ` | ` ability_check ` |
| ` requires_sight ` | ` true ` |
