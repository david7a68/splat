---
splat_schema: 2
splat_id: fd17661e-1f14-550d-9166-b4cadbc82034
splat_kind: rule
splat_key: dnd2024:glossary/attack_roll
splat_name: Attack Roll
splat_origin: dnd2024:reference/glossary/attack_roll
splat_sources:
- SRD 5.2.1, pp. 177, https://www.dndbeyond.com/srd
---

# Attack Roll

## Source text

<!-- SRD page 177 -->

An attack roll is a D20 Test that represents making an attack with a weapon, an Unarmed Strike, or a spell. See also “Playing the Game” (“D20 Tests”).

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/attack_roll ` |
| ` implementation ` | ` dnd2024:glossary_attack_roll ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` test_kind ` | ` attack_roll ` |
| ` sources ` | ` ["weapon", "unarmed_strike", "spell"] ` |

#### Base rule

<!-- splat:values path=splat_rule/parameters/base_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/d20_test ` |
