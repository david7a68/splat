---
splat_schema: 1
splat_id: f910f9d1-71dd-568c-894a-d4fea2b45e0a
splat_kind: item
splat_key: dnd2024:items/winged_boots
splat_name: Winged Boots
splat_origin:
  key: dnd2024:reference/magic_item/winged_boots
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/winged_boots
  implementation: dnd2024:item_winged_boots
  parameters:
    equipment_state: worn
    charges:
      maximum: 4
      initial: 4
      recovery:
        event: dawn
        amount:
          expression: 1d4
          scope: use
        cap: maximum
    activate:
      action: magic
      charge_cost: 1
      fly_speed_ft: 30
      duration:
        amount: 1
        unit: hour
    expires_while_flying:
      descent_ft_per_round: 30
      until: landed
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_winged_boots
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
  - time.dawn
  - time.elapsed
  - round.ended
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - resources_and_time
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 5df1ccba9a40efbe300195b5896b52ffd8b9e9b78f399febb08d1c28e4b79d4c
  source_path: builtin/srd-5.2.1/magic_item/winged_boots.md
  pages:
  - 253
  engine_gaps: &id001
  - dnd2024:item_winged_boots
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 53dd4f38f689f07bc3044f295b72c80283c1b518fc9c20ad8c6371883485cdb0
splat_clauses:
- key: source_rules
  text: '# Winged Boots Wondrous Item, Uncommon (Requires Attunement) These boots have 4 charges and regain
    1d4 expended charges daily at dawn. While wearing the boots, you can take a Magic action to expend
    1 charge, gaining a Fly Speed of 30 feet for 1 hour. If you are flying when the duration expires,
    you descend at a rate of 30 feet per round until you land.'
  sha256: 4d5bf8fc04a7eb67905902abce09dd4b7611c04e9b3ac039f804b7c0a559fb02
  status: contract_specified
splat_requires_mechanics: *id001
---

# Winged Boots

<!-- SRD page 253 -->

Wondrous Item, Uncommon (Requires Attunement)\
These boots have 4 charges and regain 1d4 ex-\
pended charges daily at dawn. While wearing the\
boots, you can take a Magic action to expend 1\
charge, gaining a Fly Speed of 30 feet for 1 hour. If\
you are flying when the duration expires, you de-\
scend at a rate of 30 feet per round until you land.\

Original layout: [[../../srd-5.2.1/pages/253|page 253]].
