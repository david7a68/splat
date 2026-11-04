---
splat_schema: 2
splat_id: 5654254d-8dca-55f6-a45a-c0d4c922c425
splat_kind: rule
splat_key: dnd2024:glossary/unconscious_condition
splat_name: Unconscious [Condition]
splat_origin: dnd2024:reference/glossary/unconscious_condition
splat_sources:
- SRD 5.2.1, pp. 191, https://www.dndbeyond.com/srd
---

# Unconscious [Condition]

## Source text

<!-- SRD page 191 -->

While you have the Unconscious condition, you experience the following effects.

Inert. You have the Incapacitated and Prone conditions, and you drop whatever you’re holding. When this condition ends, you remain Prone.

Speed 0. Your Speed is 0 and can’t increase.

Attacks Affected. Attack rolls against you have Advantage.

Saving Throws Affected. You automatically fail Strength and Dexterity saving throws.

Automatic Critical Hits. Any attack roll that hits you is a Critical Hit if the attacker is within 5 feet of you.

Unaware. You’re unaware of your surroundings.

Original layout: [[../../srd-5.2.1/pages/191|page 191]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/unconscious_condition ` |
| ` implementation ` | ` dnd2024:glossary_unconscious_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` unconscious ` |
| ` grants_condition ` | ` incapacitated ` |
| ` automatic_save_failure_abilities ` | ` ["strength", "dexterity"] ` |
| ` incoming_attack_rolls ` | ` advantage ` |
| ` automatic_critical_if_hit_attacker_within_feet ` | ` 5 ` |
| ` speed ` | ` 0 ` |
| ` speed_can_increase ` | ` false ` |
| ` also_apply_condition ` | ` prone ` |
| ` drop_all_held_items ` | ` true ` |
| ` prone_persists_after_end ` | ` true ` |
| ` aware_of_surroundings ` | ` false ` |
