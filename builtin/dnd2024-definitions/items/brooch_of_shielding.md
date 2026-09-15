---
splat_schema: 1
splat_id: da47ba02-6568-5a39-b99b-082f0309b34d
splat_kind: item
splat_key: dnd2024:items/brooch_of_shielding
splat_name: Brooch of Shielding
splat_origin:
  key: dnd2024:reference/magic_item/brooch_of_shielding
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
  mechanic: dnd2024:items/brooch_of_shielding
  implementation: dnd2024:item_brooch_of_shielding
  parameters:
    equipment_state: worn
    damage_resistance: force
    damage_immunity_from_spells:
    - key: dnd2024:spells/magic_missile
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_brooch_of_shielding
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
  - equipment.changed
  - damage.pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 252af130331cfb9bee5b5728a6475e9d10832f0a9c8f7e779f1a26c324770b8e
  source_path: builtin/srd-5.2.1/magic_item/brooch_of_shielding.md
  pages:
  - 214
  engine_gaps: &id001
  - dnd2024:item_brooch_of_shielding
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: fe76a62efe08f71608d6bd4b2419eb4d0808f39d2738c2277b4f4d7f110b3917
splat_clauses:
- key: source_rules
  text: '# Brooch of Shielding Wondrous Item, Uncommon (Requires Attunement) While wearing this brooch,
    you have Resistance to Force damage, and you have Immunity to damage from the Magic Missile spell.'
  sha256: 49fb83c8b4e4d9c40e8aceceb541b2095e1e15114149b036cd4e4144b90959ce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Brooch of Shielding

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this brooch, you have Resistance to\
Force damage, and you have Immunity to damage\
from the Magic Missile spell.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].
