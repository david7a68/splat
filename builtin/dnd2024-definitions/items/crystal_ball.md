---
splat_schema: 1
splat_id: 1e646203-063c-5a1b-b49f-6eadf351e7e7
splat_kind: item
splat_key: dnd2024:items/crystal_ball
splat_name: Crystal Ball
splat_origin:
  key: dnd2024:reference/magic_item/crystal_ball
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/crystal_ball
  implementation: dnd2024:item_crystal_ball
  parameters:
    equipment_state: touching
    spell_grants:
    - spell:
        key: dnd2024:spells/scrying
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      save_dc: 17
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_crystal_ball
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
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: c12f6b04987f2c6f5e038a4b785a7684659a66dc32b62c9d5e38c118578f5ccf
  source_path: builtin/srd-5.2.1/magic_item/crystal_ball.md
  pages:
  - 216
  engine_gaps: &id001
  - dnd2024:item_crystal_ball
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 057e628e7e58e62caf2a6f32941495f028eb6b9fc90f3699fdff78bf991b03a8
splat_clauses:
- key: source_rules
  text: '# Crystal Ball Wondrous Item, Very Rare (Requires Attunement) While touching this crystal orb,
    you can cast Scrying (save DC 17) with it.'
  sha256: 8d712291e8de3bf529ee455109dc7b142a3f15f415976cc5d7505f93e1bd9fd6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crystal Ball

<!-- SRD page 216 -->

Wondrous Item, Very Rare (Requires Attunement)\
While touching this crystal orb, you can cast Scrying\
(save DC 17) with it.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].
