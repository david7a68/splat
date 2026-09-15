---
splat_schema: 1
splat_id: 403f8926-daa8-52ca-9d11-89b35d68de02
splat_kind: item
splat_key: dnd2024:items/horseshoes_of_speed
splat_name: Horseshoes of Speed
splat_origin:
  key: dnd2024:reference/magic_item/horseshoes_of_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 227
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/horseshoes_of_speed
  implementation: dnd2024:item_horseshoes_of_speed
  parameters:
    set_quantity: 4
    attach:
      action: magic
      target: horse_or_similar_creature_hoof
      range: touch
      count_per_action: 1
    remove:
      action: magic
      count_per_action: 1
    benefit:
      requires_all_four_same_creature: true
      speed_bonus_ft: 30
    attunement_required: false
splat_contracts:
- id: dnd2024:item_horseshoes_of_speed
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
  - item.activation_requested
  - item.equipment_changed
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: ed4576e24e4926244ecc2a734ca1bb101c8ef9962257d136306ab021f19482ce
  source_path: builtin/srd-5.2.1/magic_item/horseshoes_of_speed.md
  pages:
  - 227
  engine_gaps: &id001
  - dnd2024:item_horseshoes_of_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 512e7538791d333c197c8bbbf399777514c86c52094238505200e76455156514
splat_clauses:
- key: source_rules
  text: '# Horseshoes of Speed Wondrous Item, Rare These horseshoes come in a set of four. As a Magic
    action, you can touch one of the horseshoes to the hoof of a horse or similar creature, whereupon
    the horseshoe affixes itself to the hoof. Removing a horseshoe also takes a Magic action. While all
    four horseshoes are attached to the same creature, its Speed is increased by 30 feet.'
  sha256: 549780499e7fe795eb4e80d665e03baf508cb30c94207db285898fbff8cf63c3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Horseshoes of Speed

<!-- SRD page 227 -->

Wondrous Item, Rare\
These horseshoes come in a set of four. As a Magic\
action, you can touch one of the horseshoes to the\
hoof of a horse or similar creature, whereupon the\
horseshoe affixes itself to the hoof. Removing a\
horseshoe also takes a Magic action.\
While all four horseshoes are attached to the\
same creature, its Speed is increased by 30 feet.\

Original layout: [[../../srd-5.2.1/pages/227|page 227]].
