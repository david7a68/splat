---
splat_schema: 2
splat_id: 1cbdda58-5f19-5a08-a3df-a91b20c6817a
splat_kind: rule
splat_key: dnd2024:glossary/round_down
splat_name: Round Down
splat_origin: dnd2024:reference/glossary/round_down
splat_sources:
- SRD 5.2.1, pp. 187, https://www.dndbeyond.com/srd
---

# Round Down

## Source text

<!-- SRD page 187 -->

Whenever you divide or multiply a number in the game, round down if you end up with a fraction, even if the fraction is one-half or greater. Some rules make an exception and tell you to round up.

Original layout: [[../../srd-5.2.1/pages/187|page 187]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/round_down ` |
| ` implementation ` | ` dnd2024:glossary_round_down ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` operations ` | ` ["multiply", "divide"] ` |
| ` fractional_result_rounding ` | ` floor ` |
| ` fraction_magnitude_irrelevant ` | ` true ` |
| ` explicit_round_up_rule_overrides ` | ` true ` |
