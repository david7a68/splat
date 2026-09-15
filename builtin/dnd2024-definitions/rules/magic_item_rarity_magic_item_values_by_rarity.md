---
splat_schema: 1
splat_id: 9d5e32e9-2b8d-578b-86b7-28ac79008c67
splat_kind: rule
splat_key: dnd2024:rules/magic_item_rarity_magic_item_values_by_rarity
splat_name: Magic Item Values by Rarity
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205, 206
  url: https://www.dndbeyond.com/srd
---

# Magic Item Values by Rarity

## Source text

<!-- SRD page 205 -->

Common magic items can often be bought in a\
town or city. Uncommon and Rare magic items are\
usually found only in cities, and rarer magic items\
might be sold only in wondrous locations, such as a\
city on another plane of existence. If you allow char-\
acters to buy and sell magic items in your campaign,\
rarity can help you set prices for those items. Gold\
Piece values are provided in the Magic Item Rarities\
and Values table, though a seller might ask for a ser-\
vice rather than coin as payment.\
If a magic item incorporates an item that has a\
purchase cost in “Equipment” (such as a weapon or\
a suit of armor), add that item’s cost to the magic\
item’s value. For example, +1 Armor (Plate Armor)\

<!-- SRD page 206 -->

has a value of 5,500 GP, which is the sum of a Rare\
magic item’s value (4,000 GP) and the cost of Plate\
Armor (1,500 GP).\
Magic Item Rarities and Values\
Rarity Value* Rarity Value*\
Common 100 GP Very Rare 40,000 GP\
Uncommon 400 GP Legendary 200,000 GP\
Rare 4,000 GP Artifact Priceless\
*Halve the value for a consumable item other than a Spell Scroll.\
The value of a Spell Scroll is double what it costs to scribe the scroll\
(as specified in the “Scribing Spell Scrolls” section of “Equipment”).\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_rarity_magic_item_values_by_rarity ` |
| ` implementation ` | ` dnd2024:rules_magic_item_rarity_magic_item_values_by_rarity ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magic_trade_enabled_by ` | ` gm ` |
| ` artifact_valuation ` | ` priceless ` |
| ` consumable_except_spell_scroll_multiplier ` | ` 0.5 ` |
| ` incorporated_equipment_cost ` | ` add_full_purchase_cost ` |
| ` seller_payment_choice ` | ` ["coin", "service"] ` |
| ` seller_service_terms_authority ` | ` gm ` |

#### Typical market locations

<!-- splat:values path=splat_rule/parameters/typical_market_locations -->

| Field | Value |
| --- | --- |
| ` common ` | ` ["town", "city"] ` |
| ` uncommon ` | ` ["city"] ` |
| ` rare ` | ` ["city"] ` |
| ` rarer ` | ` ["wondrous_location"] ` |

#### Rarity base value gp

<!-- splat:values path=splat_rule/parameters/rarity_base_value_gp -->

| Field | Value |
| --- | --- |
| ` common ` | ` 100 ` |
| ` uncommon ` | ` 400 ` |
| ` rare ` | ` 4000 ` |
| ` very_rare ` | ` 40000 ` |
| ` legendary ` | ` 200000 ` |
| ` artifact ` | ` null ` |

#### Spell scroll value

<!-- splat:yaml path=splat_rule/parameters/spell_scroll_value -->

```yaml
op: multiply
args:
- 2
- var: scroll.scribing_cost_gp
```
