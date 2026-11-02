---
splat_schema: 1
splat_id: 552a4ef9-25fc-50b3-be7d-860815b208d9
splat_kind: rule
splat_key: dnd2024:glossary/incapacitated_condition
splat_name: Incapacitated [Condition]
splat_origin:
  key: dnd2024:reference/glossary/incapacitated_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
---

# Incapacitated [Condition]

## Source text

<!-- SRD page 184 -->

While you have the Incapacitated condition, you experience the following effects.

Inactive. You can’t take any action, Bonus Action, or Reaction.

No Concentration. Your Concentration is broken.

Speechless. You can’t speak.

Surprised. If you’re Incapacitated when you roll Initiative, you have Disadvantage on the roll.

Original layout: [[../../srd-5.2.1/pages/184|page 184]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/incapacitated_condition ` |
| ` implementation ` | ` dnd2024:glossary_incapacitated_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` incapacitated ` |
| ` forbidden_economy ` | ` ["action", "bonus_action", "reaction"] ` |
| ` concentration_broken ` | ` true ` |
| ` can_speak ` | ` false ` |
| ` initiative_roll ` | ` disadvantage ` |
