---
splat_schema: 1
splat_id: 745adcb4-6a98-5fdf-8b2f-b5e1d7565d27
splat_kind: rule
splat_key: dnd2024:glossary/invisible_condition
splat_name: Invisible [Condition]
splat_origin:
  key: dnd2024:reference/glossary/invisible_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
---

# Invisible [Condition]

## Source text

<!-- SRD page 184 -->

While you have the Invisible condition, you experience the following effects.

Surprise. If you’re Invisible when you roll Initiative, you have Advantage on the roll.

Concealed. You aren’t affected by any effect that requires its target to be seen unless the effect’s creator can somehow see you. Any equipment you are wearing or carrying is also concealed.

Attacks Affected. Attack rolls against you have Disadvantage, and your attack rolls have Advantage. If a creature can somehow see you, you don’t gain this benefit against that creature.

Original layout: [[../../srd-5.2.1/pages/184|page 184]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/invisible_condition ` |
| ` implementation ` | ` dnd2024:glossary_invisible_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` invisible ` |
| ` initiative_roll ` | ` advantage ` |

#### Concealment

<!-- splat:values path=splat_rule/parameters/concealment -->

| Field | Value |
| --- | --- |
| ` unaffected_by_requires_seen_target_effects ` | ` true ` |
| ` exception ` | ` effect_creator_can_see_subject ` |
| ` includes_worn_and_carried_equipment ` | ` true ` |

#### Attacks

<!-- splat:values path=splat_rule/parameters/attacks -->

| Field | Value |
| --- | --- |
| ` incoming ` | ` disadvantage ` |
| ` outgoing ` | ` advantage ` |
| ` exception ` | ` other_creature_can_see_subject ` |
