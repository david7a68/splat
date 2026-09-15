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
splat_rule:
  mechanic: dnd2024:rules/crafting_magic_items_time_and_cost
  parameters:
    rarity_table:
    - rarity: common
      days: 5
      raw_material_gp: 50
    - rarity: uncommon
      days: 10
      raw_material_gp: 200
    - rarity: rare
      days: 50
      raw_material_gp: 2000
    - rarity: very_rare
      days: 125
      raw_material_gp: 20000
    - rarity: legendary
      days: 250
      raw_material_gp: 100000
    work_hours_per_day: 8
    consecutive_days_required: false
    assistants:
      default_maximum: 1
      gm_can_allow_more: true
      time_divisor: total_workers
    consumables:
      time_multiplier: 0.5
      cost_multiplier: 0.5
      exclude: spell_scroll
      spell_scroll_rules: equipment_scribing_rules
    raw_material_availability:
      determined_by: gm
      city_chance_percent: 75
      other_settlement_chance_percent: 25
      failed_check_retry_minimum_days: 7
    incorporated_mundane_item:
      additional_requirement_choice:
      - pay_entire_purchase_cost
      - craft_under_nonmagical_rules
    plate_plus_one_example:
      buy_plate_and_enchant_gp: 3500
      craft_plate_and_enchant_gp: 2000
      plate_crafting_required_for_second_option: true
  implementation: dnd2024:rule_crafting_magic_items_time_and_cost
splat_contracts:
- id: dnd2024:rule_crafting_magic_items_time_and_cost
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - crafted_items
  events: []
  capabilities:
  - crafting
  - economy
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 207
  engine_gaps: &id001
  - dnd2024:rule_crafting_magic_items_time_and_cost
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: f9f2100617cedd7b3e1371182acfb54162bba05494e5ffdd05d790f633e9c069
  parent_name: Crafting Magic Items
  line_span:
    first: 10698
    count: 35
splat_clauses:
- key: source_rules
  text: '# Time and Cost Crafting a magic item takes an amount of time and money based on the item’s rarity
    as shown in the Magic Item Crafting Time and Cost table. Work per Day. For each day of crafting, you
    must work for 8 hours. If an item requires multiple days, those days needn’t be consecutive. Assistants.
    Characters can combine their efforts to shorten the crafting time. Divide the time needed to create
    an item by the number of characters working on it. Normally, only one other character can assist you,
    but the GM might allow more assistants. Raw Materials. The cost in the table represents the raw materials
    needed to make a magic item. The GM determines whether appropriate raw materials are available. In
    a city, there is a 75 percent chance that the materials are available, and in any other settlement,
    that chance is 25 percent. If materials aren’t available, you must wait at least 7 days before checking
    on the availability again. If a magic item incorporates an item that has a purchase cost (such as
    a weapon or a suit of armor), you must also pay that entire cost or craft that item using the rules
    in “Equipment.” For example, to make +1 Armor (Plate Armor), you must pay 3,500 GP or pay 2,000 GP
    and craft the armor. Magic Item Crafting Time and Cost Item Rarity Time* Cost* Common 5 days 50 GP
    Uncommon 10 days 200 GP Rare 50 days 2,000 GP Very Rare 125 days 20,000 GP Legendary 250 days 100,000
    GP *The time and cost are halved for a consumable item other than a Spell Scroll, whose crafting time
    and cost are given in “Equipment.”'
  sha256: eb6e162238620bf0c1cc8a09e88bc01be5af5cdc7ff6b30c6657a32540a8e111
  status: contract_specified
splat_requires_mechanics: *id001
---

# Time and Cost

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
