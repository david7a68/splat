---
splat_schema: 1
splat_id: bf199fa5-ef53-5f73-83c3-ad4ff5d66cd7
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items
splat_name: Sentient Magic Items
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
---

# Sentient Magic Items

## Source text

<!-- SRD page 207 -->

Some magic items have sentience and personality. Such an item might be possessed, haunted by the spirit of a previous owner, or self-aware thanks to the magic used to create it. A sentient item might be a cherished ally to its wielder or a continual thorn in the side.

Most sentient items are weapons, but other kinds of items can manifest sentience. Single-use items such as potions and scrolls are never sentient.

The GM controls sentient magic items and their activated properties. A bearer who maintains a good relationship with the item can access those properties. If the relationship is strained, a conflict can ensue (see “Conflict” below).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sentient_magic_items ` |
| ` implementation ` | ` dnd2024:rules_sentient_magic_items ` |

### Parameters

#### Sentience

<!-- splat:values path=splat_rule/parameters/sentience -->

| Field | Value |
| --- | --- |
| ` single_use_item_allowed ` | ` false ` |
| ` controller ` | ` gm ` |
| ` activated_properties_controller ` | ` gm ` |
| ` good_relationship ` | ` bearer_access ` |
| ` strained_relationship ` | ` possible_conflict ` |

#### Conflict rules

<!-- splat:values path=splat_rule/parameters/conflict_rules -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/sentient_magic_items_conflict ` |
