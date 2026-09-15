---
splat_schema: 1
splat_id: b68a6254-8080-5093-abb4-54380a137949
splat_kind: item
splat_key: dnd2024:items/eversmoking_bottle
splat_name: Eversmoking Bottle
splat_origin:
  key: dnd2024:reference/magic_item/eversmoking_bottle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220, 221
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/eversmoking_bottle
  implementation: dnd2024:item_eversmoking_bottle
  parameters:
    toggle_action: magic
    open:
      cloud:
        shape: emanation
        origin: bottle
        radius_ft: 60
        obscurity: heavily_obscured
        growth_ft_per_minute: 10
        maximum_radius_ft: 120
    closed:
      cloud_location: fixed_at_closing
      disperse_after:
        amount: 10
        unit: minute
    strong_wind:
      disperse_after:
        amount: 1
        unit: minute
      examples:
      - key: dnd2024:spells/gust_of_wind
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_eversmoking_bottle
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - action.magic_pending
  - time.minute
  - wind.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f283d61f7fa654cf1df1a70327c158ebb459dc339c662f3104ae14521ce00a85
  source_path: builtin/srd-5.2.1/magic_item/eversmoking_bottle.md
  pages:
  - 220
  - 221
  engine_gaps: &id001
  - dnd2024:item_eversmoking_bottle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 111e7bb7b9d60f11727949ba661276187248a47d0d076a410cb33befbddb8be8
splat_clauses:
- key: source_rules
  text: '# Eversmoking Bottle Wondrous Item, Uncommon As a Magic action, you can open or close this bottle.
    Opening the bottle causes thick smoke to billow out, forming a cloud that fills a 60-foot Emanation
    originating from the bottle. The area within the smoke is Heavily Obscured. Each minute the bottle
    remains open, the size of the Emanation increases by 10 feet until it reaches its maximum size of
    120 feet. Closing the bottle causes the cloud to become fixed in place until it disperses after 10
    minutes. A strong wind (such as that created by the Gust of Wind spell) disperses the cloud after
    1 minute.'
  sha256: 22c040eeddbd829104796b5266a6598a9e75f42728bfb1d3f1b4428972a54cfd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eversmoking Bottle

<!-- SRD page 220 -->

Wondrous Item, Uncommon\
As a Magic action, you can open or close this bottle.\
Opening the bottle causes thick smoke to billow\
out, forming a cloud that fills a 60-foot Emanation\
originating from the bottle. The area within the\
smoke is Heavily Obscured.\
Each minute the bottle remains open, the size of\
the Emanation increases by 10 feet until it reaches\
its maximum size of 120 feet.\

<!-- SRD page 221 -->

Closing the bottle causes the cloud to become\
fixed in place until it disperses after 10 minutes.\
A strong wind (such as that created by the Gust of\
Wind spell) disperses the cloud after 1 minute.\

Original layout: [[../../srd-5.2.1/pages/220|page 220]], [[../../srd-5.2.1/pages/221|page 221]].
