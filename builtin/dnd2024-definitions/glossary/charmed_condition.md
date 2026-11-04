---
splat_schema: 2
splat_id: af7ddf38-e3b0-5c31-9d7e-1a34cd4ba1f1
splat_kind: rule
splat_key: dnd2024:glossary/charmed_condition
splat_name: Charmed [Condition]
splat_origin: dnd2024:reference/glossary/charmed_condition
splat_sources:
- SRD 5.2.1, pp. 178, https://www.dndbeyond.com/srd
---

# Charmed [Condition]

## Source text

<!-- SRD page 178 -->

While you have the Charmed condition, you experience the following effects.

Can’t Harm the Charmer. You can’t attack the charmer or target the charmer with damaging abilities or magical effects.

Social Advantage. The charmer has Advantage on any ability check to interact with you socially.

Original layout: [[../../srd-5.2.1/pages/178|page 178]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/charmed_condition ` |
| ` implementation ` | ` dnd2024:glossary_charmed_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` charmed ` |
| ` forbidden_against_charmer ` | ` ["attack", "target_with_damaging_ability", "target_with_damaging_magical_effect"] ` |
| ` charmer_social_ability_checks ` | ` advantage ` |
