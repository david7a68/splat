---
splat_schema: 2
splat_id: e511a026-8cb1-521b-b740-dc415548ebf3
splat_kind: rule
splat_key: dnd2024:glossary/dodge_action
splat_name: Dodge [Action]
splat_origin: dnd2024:reference/glossary/dodge_action
splat_sources:
- SRD 5.2.1, pp. 181, https://www.dndbeyond.com/srd
---

# Dodge [Action]

## Source text

<!-- SRD page 181 -->

If you take the Dodge action, you gain the following benefits: until the start of your next turn, any attack roll made against you has Disadvantage if you can see the attacker, and you make Dexterity saving throws with Advantage.

You lose these benefits if you have the Incapacitated condition or if your Speed is 0.

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/dodge_action ` |
| ` implementation ` | ` dnd2024:glossary_dodge_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` ends ` | ` own_next_turn_start ` |
| ` dexterity_saves ` | ` advantage ` |
| ` lose_benefits_when_any ` | ` ["incapacitated", "speed_zero"] ` |

#### Incoming attacks

<!-- splat:values path=splat_rule/parameters/incoming_attacks -->

| Field | Value |
| --- | --- |
| ` disadvantage ` | ` true ` |
| ` requires_subject_sees_attacker ` | ` true ` |
