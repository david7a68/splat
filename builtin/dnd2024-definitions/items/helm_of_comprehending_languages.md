---
splat_schema: 1
splat_id: d43cd3df-bdc0-5be3-89bd-eb7e12f12d68
splat_kind: item
splat_key: dnd2024:items/helm_of_comprehending_languages
splat_name: Helm of Comprehending Languages
splat_origin:
  key: dnd2024:reference/magic_item/helm_of_comprehending_languages
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/helm_of_comprehending_languages
  implementation: dnd2024:item_helm_of_comprehending_languages
  parameters:
    equipment_state: worn
    spell_grants:
    - spell:
        key: dnd2024:spells/comprehend_languages
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_helm_of_comprehending_languages
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
  source_sha256: b340aa995b64f336bef6999336160aab955bcdfbeb1dbbbb0d031de67e574885
  source_path: builtin/srd-5.2.1/magic_item/helm_of_comprehending_languages.md
  pages:
  - 226
  engine_gaps: &id001
  - dnd2024:item_helm_of_comprehending_languages
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: db452d6c884adccbe0d283578a631aa8126f0b9a82d73fc9baab586ca9680c22
splat_clauses:
- key: source_rules
  text: '# Helm of Comprehending Languages Wondrous Item, Uncommon While wearing this helm, you can cast
    Comprehend Languages from it.'
  sha256: 2c8f660aae8ca19f70197792ad81a6e1d2a555d3fd17ef04271d9315f4b266a0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Helm of Comprehending Languages

<!-- SRD page 226 -->

Wondrous Item, Uncommon\
While wearing this helm, you can cast Comprehend\
Languages from it.\

Original layout: [[../../srd-5.2.1/pages/226|page 226]].
