---
splat_schema: 1
splat_id: 2335a66c-5d9e-568f-bcc0-994e8ea396d0
splat_kind: item
splat_key: dnd2024:items/ring_of_three_wishes
splat_name: Ring of Three Wishes
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_three_wishes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: legendary
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/ring_of_three_wishes
  implementation: dnd2024:item_ring_of_three_wishes
  parameters:
    equipment_state: worn_on_finger_or_similar_digit
    charges:
      maximum: 3
      initial: 3
    spell_grants:
    - spell:
        key: dnd2024:spells/wish
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    last_charge:
      magical: false
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rings
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_ring_of_three_wishes
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
  - spell.cast_pending
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 4a252c46a17d52fc0a53987d8d8fbd8c5ba57a446839e0e78a636c06314816b8
  source_path: builtin/srd-5.2.1/magic_item/ring_of_three_wishes.md
  pages:
  - 239
  engine_gaps: &id001
  - dnd2024:item_ring_of_three_wishes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: a7c323043a7c4ae7374873f996613148b9606d860b3acde9f735db4a18c65f8e
splat_clauses:
- key: source_rules
  text: '# Ring of Three Wishes Ring, Legendary While wearing this ring, you can expend 1 of its 3 charges
    to cast Wish from it. The ring becomes nonmagical when you use the last charge.'
  sha256: 864c0020754eae66a966d067beaf232f79341e5ab2cc1e69a4cc7af53f2c3a88
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Three Wishes

<!-- SRD page 239 -->

Ring, Legendary\
While wearing this ring, you can expend 1 of its 3\
charges to cast Wish from it. The ring becomes non-\
magical when you use the last charge.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].
