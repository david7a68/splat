---
splat_schema: 1
splat_id: 9cafd985-1efb-5510-a0be-df8fedfab882
splat_kind: rule
splat_key: dnd2024:glossary/paralyzed_condition
splat_name: Paralyzed [Condition]
splat_origin:
  key: dnd2024:reference/glossary/paralyzed_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
---

# Paralyzed [Condition]

## Source text

<!-- SRD page 186 -->

While you have the Paralyzed condition, you experience the following effects.

Incapacitated. You have the Incapacitated condition.

Speed 0. Your Speed is 0 and can’t increase.

Saving Throws Affected. You automatically fail Strength and Dexterity saving throws.

Attacks Affected. Attack rolls against you have Advantage.

Automatic Critical Hits. Any attack roll that hits you is a Critical Hit if the attacker is within 5 feet of you.

Original layout: [[../../srd-5.2.1/pages/186|page 186]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/paralyzed_condition ` |
| ` implementation ` | ` dnd2024:glossary_paralyzed_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` paralyzed ` |
| ` grants_condition ` | ` incapacitated ` |
| ` automatic_save_failure_abilities ` | ` ["strength", "dexterity"] ` |
| ` incoming_attack_rolls ` | ` advantage ` |
| ` automatic_critical_if_hit_attacker_within_feet ` | ` 5 ` |
| ` speed ` | ` 0 ` |
| ` speed_can_increase ` | ` false ` |
