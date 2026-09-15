---
splat_schema: 1
splat_id: 0a6c8f2a-a31d-54c2-84f6-c2804a638acf
splat_kind: rule
splat_key: dnd2024:rules/activating_a_magic_item_charges
splat_name: Charges
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
---

# Charges

## Source text

<!-- SRD page 206 -->

Some magic items have charges that must be ex-\
pended to activate their properties. The number\
of charges an item has remaining is revealed when\
the Identify spell is cast on it. A creature attuned to\
an item knows how many charges the item has and\
how many it regains.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/activating_a_magic_item_charges ` |
| ` implementation ` | ` dnd2024:rules_activating_a_magic_item_charges ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` activation_expends_specified_charges ` | ` true ` |
| ` identify_reveals ` | ` remaining_charges ` |
| ` attuned_bearer_knows ` | ` ["remaining_charges", "charge_recovery"] ` |
