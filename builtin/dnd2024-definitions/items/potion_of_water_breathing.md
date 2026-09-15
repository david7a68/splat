---
splat_schema: 1
splat_id: 52803957-ef91-5928-b69d-66c179ae2818
splat_kind: item
splat_key: dnd2024:items/potion_of_water_breathing
splat_name: Potion of Water Breathing
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_water_breathing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236, 237
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_water_breathing
  implementation: dnd2024:item_potion_of_water_breathing
  parameters:
    consume: drink
    duration:
      amount: 24
      unit: hour
    breathe_underwater: true
    consume_action: bonus_action
    administer_action: bonus_action
    effect_target: drinker
    consumed_quantity: 1
    effect_starts: immediately_after_consumption
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_potions
splat_contracts:
- id: dnd2024:item_potion_of_water_breathing
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
  - item.consumed
  - time.elapsed
  capabilities:
  - inventory
  - effects
  - resources_and_time
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 90769249c20615ad3e36b9660bc5d200ceb672808ecb7d884db2721b31bf9101
  source_path: builtin/srd-5.2.1/magic_item/potion_of_water_breathing.md
  pages:
  - 236
  - 237
  engine_gaps: &id001
  - dnd2024:item_potion_of_water_breathing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: f3653e1ff78399e17435529aa15c261f8558e920a795934aaa47bbe235aad646
splat_clauses:
- key: source_rules
  text: '# Potion of Water Breathing Potion, Uncommon You can breathe underwater for 24 hours after drinking
    this potion. This potion’s cloudy green fluid smells of the sea and has a jellyfish-like bubble floating
    in it.'
  sha256: 45059f694f18a67e2bb16a4b8c5d25c71d98fd62b3c7eacb600015e6e43216e7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Water Breathing

<!-- SRD page 236 -->

Potion, Uncommon\
You can breathe underwater for 24 hours after\
drinking this potion.\

<!-- SRD page 237 -->

This potion’s cloudy green fluid smells of the sea\
and has a jellyfish-like bubble floating in it.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]], [[../../srd-5.2.1/pages/237|page 237]].
