---
splat_schema: 1
splat_id: 046fba86-2d45-5ef1-9c04-dc66e7ec1760
splat_kind: item
splat_key: dnd2024:items/helm_of_teleportation
splat_name: Helm of Teleportation
splat_origin:
  key: dnd2024:reference/magic_item/helm_of_teleportation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/helm_of_teleportation
  implementation: dnd2024:item_helm_of_teleportation
  parameters:
    equipment_state: worn
    charges:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 3
          bonus: 0
    spells:
    - spell:
        key: dnd2024:spells/teleport
      cost: 1
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_helm_of_teleportation
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    locations:
      required: false
      type: location_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  - spell.cast_completed
  capabilities:
  - equipment
  - item_resources
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 530a39444572032653adc6d0b23f4d6851eb6612b60b1459babe706e91db40d9
  source_path: builtin/srd-5.2.1/magic_item/helm_of_teleportation.md
  pages:
  - 226
  engine_gaps: &id001
  - dnd2024:item_helm_of_teleportation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 7a5b7c3eeea1298e1b83d718ad0aa9905928d9a236aaeb99ff5996ddf821c926
splat_clauses:
- key: source_rules
  text: '# Helm of Teleportation Wondrous Item, Rare (Requires Attunement) This helm has 3 charges. While
    wearing it, you can expend 1 charge to cast Teleport from it. The helm regains 1d3 expended charges
    daily at dawn.'
  sha256: 94909fa88f24146dad6b9a5d692b55c341d263e6dcec35ce062cabeda7324432
  status: contract_specified
splat_requires_mechanics: *id001
---

# Helm of Teleportation

<!-- SRD page 226 -->

Wondrous Item, Rare (Requires Attunement)\
This helm has 3 charges. While wearing it, you can\
expend 1 charge to cast Teleport from it. The helm\
regains 1d3 expended charges daily at dawn.\

Original layout: [[../../srd-5.2.1/pages/226|page 226]].
