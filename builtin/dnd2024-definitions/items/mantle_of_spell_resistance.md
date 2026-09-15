---
splat_schema: 1
splat_id: 862e1840-e112-5b5f-b1b9-61821712e71c
splat_kind: item
splat_key: dnd2024:items/mantle_of_spell_resistance
splat_name: Mantle of Spell Resistance
splat_origin:
  key: dnd2024:reference/magic_item/mantle_of_spell_resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/mantle_of_spell_resistance
  implementation: dnd2024:item_mantle_of_spell_resistance
  parameters:
    equipped: worn
    saving_throw_advantage:
      against: spells
    attunement_required: true
splat_contracts:
- id: dnd2024:item_mantle_of_spell_resistance
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
  - saving_throw.pending
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: cbeafb97ce393d2c39e77c0dff22cbd25b8055fc19d743ed8b95203e9b26fc6f
  source_path: builtin/srd-5.2.1/magic_item/mantle_of_spell_resistance.md
  pages:
  - 229
  engine_gaps: &id001
  - dnd2024:item_mantle_of_spell_resistance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: c7b476228a4404dcb43ed193769c5e8a41b8d4e6c91cbd858ee6cc8049b97d60
splat_clauses:
- key: source_rules
  text: '# Mantle of Spell Resistance Wondrous Item, Rare (Requires Attunement) You have Advantage on
    saving throws against spells while you wear this cloak.'
  sha256: 4d0a1557a9f4fa39910744dec16d046057cb3679defd508047f323f9f66455bd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mantle of Spell Resistance

<!-- SRD page 229 -->

Wondrous Item, Rare (Requires Attunement)\
You have Advantage on saving throws against spells\
while you wear this cloak.\

Original layout: [[../../srd-5.2.1/pages/229|page 229]].
