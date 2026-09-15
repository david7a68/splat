---
splat_schema: 1
splat_id: 20d2330b-7804-5912-8881-10546ede8919
splat_kind: rule
splat_key: dnd2024:rules/crafting_magic_items_time_and_cost
splat_name: Time and Cost
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
---

# Time and Cost

## Source text

<!-- SRD page 207 -->

Crafting a magic item takes an amount of time and\
money based on the item’s rarity as shown in the\
Magic Item Crafting Time and Cost table.\
Work per Day. For each day of crafting, you must\
work for 8 hours. If an item requires multiple days,\
those days needn’t be consecutive.\
Assistants. Characters can combine their efforts\
to shorten the crafting time. Divide the time needed\
to create an item by the number of characters work-\
ing on it. Normally, only one other character can\
assist you, but the GM might allow more assistants.\
Raw Materials. The cost in the table represents\
the raw materials needed to make a magic item. The\
GM determines whether appropriate raw materials\
are available. In a city, there is a 75 percent chance\
that the materials are available, and in any other\
settlement, that chance is 25 percent. If materials\
aren’t available, you must wait at least 7 days before\
checking on the availability again.\
If a magic item incorporates an item that has a\
purchase cost (such as a weapon or a suit of armor),\
you must also pay that entire cost or craft that item\
using the rules in “Equipment.” For example, to\
make +1 Armor (Plate Armor), you must pay 3,500\
GP or pay 2,000 GP and craft the armor.\
Magic Item Crafting Time and Cost\
Item Rarity Time* Cost*\
Common 5 days 50 GP\
Uncommon 10 days 200 GP\
Rare 50 days 2,000 GP\
Very Rare 125 days 20,000 GP\
Legendary 250 days 100,000 GP\
*The time and cost are halved for a consumable item other than\
a Spell Scroll, whose crafting time and cost are given in “Equipment.”\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/crafting_magic_items_time_and_cost ` |
| ` implementation ` | ` dnd2024:rule_crafting_magic_items_time_and_cost ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` work_hours_per_day ` | ` 8 ` |
| ` consecutive_days_required ` | ` false ` |

#### Rarity table

<!-- splat:table
path: splat_rule/parameters/rarity_table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
columns:
  Key:
    type: key
  Rarity:
    path: rarity
    type: symbol
  Days:
    path: days
    type: integer
  Raw material gp:
    path: raw_material_gp
    type: integer
-->

| Key | Rarity | Days | Raw material gp |
| --- | --- | --- | --- |
| ` row_1 ` | ` Common ` | ` 5 ` | ` 50 ` |
| ` row_2 ` | ` Uncommon ` | ` 10 ` | ` 200 ` |
| ` row_3 ` | ` Rare ` | ` 50 ` | ` 2000 ` |
| ` row_4 ` | ` Very rare ` | ` 125 ` | ` 20000 ` |
| ` row_5 ` | ` Legendary ` | ` 250 ` | ` 100000 ` |

#### Assistants

<!-- splat:values path=splat_rule/parameters/assistants -->

| Field | Value |
| --- | --- |
| ` default_maximum ` | ` 1 ` |
| ` gm_can_allow_more ` | ` true ` |
| ` time_divisor ` | ` total_workers ` |

#### Consumables

<!-- splat:values path=splat_rule/parameters/consumables -->

| Field | Value |
| --- | --- |
| ` time_multiplier ` | ` 0.5 ` |
| ` cost_multiplier ` | ` 0.5 ` |
| ` exclude ` | ` spell_scroll ` |
| ` spell_scroll_rules ` | ` equipment_scribing_rules ` |

#### Raw material availability

<!-- splat:values path=splat_rule/parameters/raw_material_availability -->

| Field | Value |
| --- | --- |
| ` determined_by ` | ` gm ` |
| ` city_chance_percent ` | ` 75 ` |
| ` other_settlement_chance_percent ` | ` 25 ` |
| ` failed_check_retry_minimum_days ` | ` 7 ` |

#### Incorporated mundane item

<!-- splat:values path=splat_rule/parameters/incorporated_mundane_item -->

| Field | Value |
| --- | --- |
| ` additional_requirement_choice ` | ` ["pay_entire_purchase_cost", "craft_under_nonmagical_rules"] ` |

#### Plate plus one example

<!-- splat:values path=splat_rule/parameters/plate_plus_one_example -->

| Field | Value |
| --- | --- |
| ` buy_plate_and_enchant_gp ` | ` 3500 ` |
| ` craft_plate_and_enchant_gp ` | ` 2000 ` |
| ` plate_crafting_required_for_second_option ` | ` true ` |
