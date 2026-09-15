---
splat_schema: 1
splat_id: e885cce0-ca1c-502c-8043-9887e57a7c4f
splat_kind: rule
splat_key: dnd2024:glossary/disengage_action
splat_name: Disengage [Action]
splat_origin:
  key: dnd2024:reference/glossary/disengage_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
---

# Disengage [Action]

## Source text

<!-- SRD page 181 -->

If you take the Disengage action, your movement\
doesn’t provoke Opportunity Attacks for the rest of\
the current turn.\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/disengage_action ` |
| ` implementation ` | ` dnd2024:glossary_disengage_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` duration ` | ` remainder_of_current_turn ` |
| ` movement_provokes_opportunity_attacks ` | ` false ` |
