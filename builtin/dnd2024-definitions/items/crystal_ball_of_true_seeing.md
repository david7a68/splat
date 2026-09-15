---
splat_schema: 1
splat_id: dba28332-7e38-5d3e-9fcd-0a534695c20a
splat_kind: item
splat_key: dnd2024:items/crystal_ball_of_true_seeing
splat_name: Crystal Ball of True Seeing
splat_origin:
  key: dnd2024:reference/magic_item/crystal_ball_of_true_seeing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/crystal_ball_of_true_seeing
  implementation: dnd2024:item_crystal_ball_of_true_seeing
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
    while_scrying:
      truesight_ft: 120
      origin: spell_sensor
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_crystal_ball_of_true_seeing
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
  - perception.pending
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
  source_sha256: a1f07c647ce6194155e4e7354afa2241a0c912825321f7f27a9c278725173da1
  source_path: builtin/srd-5.2.1/magic_item/crystal_ball_of_true_seeing.md
  pages:
  - 216
  engine_gaps: &id001
  - dnd2024:item_crystal_ball_of_true_seeing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: b86144fba301ae0828dcddec28a3ddfaa44cfbf79145aba6efbb58f52ced659d
splat_clauses:
- key: source_rules
  text: '# Crystal Ball of True Seeing Wondrous Item, Legendary (Requires Attunement) While touching this
    crystal orb, you can cast Scrying (save DC 17) with it. In addition, you have Truesight with a range
    of 120 feet centered on the spell’s sensor.'
  sha256: 394f84ad5aee8035ac52d44c573b82a05e47a469728f5d1879d40513da2e0205
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crystal Ball of True Seeing

<!-- SRD page 216 -->

Wondrous Item, Legendary (Requires Attunement)\
While touching this crystal orb, you can cast Scrying\
(save DC 17) with it. In addition, you have Truesight\
with a range of 120 feet centered on the spell’s\
sensor.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].
