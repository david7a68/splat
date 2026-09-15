---
splat_schema: 1
splat_id: 95cf6ee6-059f-5661-8aeb-127435d11c94
splat_kind: item
splat_key: dnd2024:items/amulet_of_proof_against_detection_and_location
splat_name: Amulet of Proof against Detection and Location
splat_origin:
  key: dnd2024:reference/magic_item/amulet_of_proof_against_detection_and_location
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/amulet_of_proof_against_detection_and_location
  implementation: dnd2024:item_amulet_of_proof_against_detection_and_location
  parameters:
    equipment_state: worn
    prevent:
      spell_targeting:
        school: divination
      perception_by: magical_scrying_sensors
    exception:
      chooser: wearer
      allow: true
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_amulet_of_proof_against_detection_and_location
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
  - spell.target_pending
  - scrying.perception_pending
  - equipment.changed
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 756024690d214ed729717b80eda0182382668a62651649c0177425125025d969
  source_path: builtin/srd-5.2.1/magic_item/amulet_of_proof_against_detection_and_location.md
  pages:
  - 209
  engine_gaps: &id001
  - dnd2024:item_amulet_of_proof_against_detection_and_location
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: b4bd743f2711067e5b4a9761b3d7f4b3852d1fe5df67668b2334cdb148dc21fa
splat_clauses:
- key: source_rules
  text: '# Amulet of Proof against Detection and Location Wondrous Item, Uncommon (Requires Attunement)
    While wearing this amulet, you can’t be targeted by Divination spells or perceived through magical
    scrying sensors unless you allow it.'
  sha256: 6aa258242ae1a997791b12e6f0311d89d34e795c64e7fcbd0976d77ad912ea56
  status: contract_specified
splat_requires_mechanics: *id001
---

# Amulet of Proof against Detection and Location

<!-- SRD page 209 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this amulet, you can’t be targeted\
by Divination spells or perceived through magical\
scrying sensors unless you allow it.\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].
