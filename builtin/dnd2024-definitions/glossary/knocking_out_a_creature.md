---
splat_schema: 1
splat_id: 23c2a7d0-f633-56b9-b23d-c4008e65c4af
splat_kind: rule
splat_key: dnd2024:glossary/knocking_out_a_creature
splat_name: Knocking Out a Creature
splat_origin:
  key: dnd2024:reference/glossary/knocking_out_a_creature
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
---

# Knocking Out a Creature

## Source text

<!-- SRD page 184 -->

When you would reduce a creature to 0 Hit Points with a melee attack, you can instead reduce the creature to 1 Hit Point. The creature then has the Unconscious condition and starts a Short Rest.

The creature remains Unconscious until it regains any Hit Points or until someone uses an action to administer first aid to it, which requires a successful DC 10 Wisdom (Medicine) check.

Original layout: [[../../srd-5.2.1/pages/184|page 184]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/knocking_out_a_creature ` |
| ` implementation ` | ` dnd2024:glossary_knocking_out_a_creature ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` optional_attacker_choice ` | ` true ` |
| ` replacement_hit_points ` | ` 1 ` |
| ` apply_condition ` | ` unconscious ` |
| ` start_rest ` | ` short_rest ` |
| ` ends_unconscious_when_any ` | ` ["regains_any_hit_points", "successful_first_aid"] ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` attack_type ` | ` melee ` |
| ` would_reduce_target_to_hit_points ` | ` 0 ` |

#### First aid

<!-- splat:values path=splat_rule/parameters/first_aid -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` medicine ` |
| ` dc ` | ` 10 ` |
