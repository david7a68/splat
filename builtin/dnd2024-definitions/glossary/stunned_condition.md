---
splat_schema: 1
splat_id: 0a971496-ae3b-5b45-91b8-87841b2e2ae3
splat_kind: rule
splat_key: dnd2024:glossary/stunned_condition
splat_name: Stunned [Condition]
splat_origin:
  key: dnd2024:reference/glossary/stunned_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 189
  url: https://www.dndbeyond.com/srd
---

# Stunned [Condition]

## Source text

<!-- SRD page 189 -->

While you have the Stunned condition, you experience the following effects.

Incapacitated. You have the Incapacitated condition.

Saving Throws Affected. You automatically fail Strength and Dexterity saving throws.

Attacks Affected. Attack rolls against you have Advantage.

Original layout: [[../../srd-5.2.1/pages/189|page 189]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/stunned_condition ` |
| ` implementation ` | ` dnd2024:glossary_stunned_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` stunned ` |
| ` grants_condition ` | ` incapacitated ` |
| ` automatic_save_failure_abilities ` | ` ["strength", "dexterity"] ` |
| ` incoming_attack_rolls ` | ` advantage ` |
