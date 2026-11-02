---
splat_schema: 1
splat_id: 8c1b2f53-03f0-5262-bebd-66ac70912d60
splat_kind: rule
splat_key: dnd2024:rules/activating_a_magic_item_consumable_items
splat_name: Consumable Items
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
---

# Consumable Items

## Source text

<!-- SRD page 206 -->

Some items are consumed—used up, in other words—when they are activated. A Potion of Healing must be swallowed, for example, while the writing vanishes from a scroll when it is read. Once used, a consumable item loses its magic.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/activating_a_magic_item_consumable_items ` |
| ` implementation ` | ` dnd2024:rules_activating_a_magic_item_consumable_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` consumed_on_activation ` | ` true ` |

#### On consumed

<!-- splat:values path=splat_rule/parameters/on_consumed -->

| Field | Value |
| --- | --- |
| ` magic_lost ` | ` true ` |
