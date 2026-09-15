---
splat_schema: 1
splat_id: 1f3c4507-d7eb-5c94-8a92-f7d397a1b5a7
splat_kind: item
splat_key: dnd2024:items/boots_of_the_winterlands
splat_name: Boots of the Winterlands
splat_origin:
  key: dnd2024:reference/magic_item/boots_of_the_winterlands
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/boots_of_the_winterlands
  implementation: dnd2024:item_boots_of_the_winterlands
  parameters:
    equipped: worn
    resistance:
    - cold
    temperature_protection:
      fahrenheit_maximum: 0
      lower_limit: null
      additional_protection_required: false
    ignore_difficult_terrain:
    - ice
    - snow
    attunement_required: true
splat_contracts:
- id: dnd2024:item_boots_of_the_winterlands
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - item.equipment_changed
  - damage.pending
  - movement.cost_requested
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 9a6d1525b9b781b897e4cd8cd1c09ea31f9abb0dc518665e088175e7ca382cbe
  source_path: builtin/srd-5.2.1/magic_item/boots_of_the_winterlands.md
  pages:
  - 214
  engine_gaps: &id001
  - dnd2024:item_boots_of_the_winterlands
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: f07ad23b9453b3434055edd35f9f8d3734a1f3ed098d856fa03f1b25e4befeb9
splat_clauses:
- key: source_rules
  text: '# Boots of the Winterlands Wondrous Item, Uncommon (Requires Attunement) These furred boots are
    snug and feel warm. While wearing them, you gain the following benefits. Cold Resistance. You have
    Resistance to Cold damage and can tolerate temperatures of 0 degrees Fahrenheit or lower without any
    additional protection. Winter Strider. You ignore Difficult Terrain created by ice or snow.'
  sha256: 95b9f6d0043d0d7fdfd273ce658795e4441e1aaaab515ab763051bfb07a918ac
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boots of the Winterlands

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)\
These furred boots are snug and feel warm. While\
wearing them, you gain the following benefits.\
Cold Resistance. You have Resistance to Cold\
damage and can tolerate temperatures of 0 degrees\
Fahrenheit or lower without any additional\
protection.\
Winter Strider. You ignore Difficult Terrain cre-\
ated by ice or snow.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].
