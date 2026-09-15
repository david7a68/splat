---
splat_schema: 1
splat_id: 2b2754cf-08c3-58ac-bb4c-ab067f7cd300
splat_kind: item
splat_key: dnd2024:items/elven_chain
splat_name: Elven Chain
splat_origin:
  key: dnd2024:reference/magic_item/elven_chain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/elven_chain
  implementation: dnd2024:item_elven_chain
  parameters:
    equipped: worn
    ac_bonus: 1
    armor_training:
      applies_to: this_item
      granted: true
      requires_prior_training: false
    attunement_required: false
splat_contracts:
- id: dnd2024:item_elven_chain
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
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 5a68994be740ac3437b0c31d5ca70d7a166a162feef1815ca87993c9a89172e4
  source_path: builtin/srd-5.2.1/magic_item/elven_chain.md
  pages:
  - 220
  engine_gaps: &id001
  - dnd2024:item_elven_chain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 9b685653de74ff131195a8b81f42b2315fb4552924a95c0a1c8069d003301f00
splat_clauses:
- key: source_rules
  text: '# Elven Chain Armor (Chain Mail or Chain Shirt), Rare You gain a +1 bonus to Armor Class while
    you wear this armor. You are considered trained with this armor even if you lack training with Medium
    or Heavy armor.'
  sha256: 259803559d03903d6a3c5cd0b0e9b5645f02f06083879e6dfdbb9b6d76f44c7e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Elven Chain

<!-- SRD page 220 -->

Armor (Chain Mail or Chain Shirt), Rare\
You gain a +1 bonus to Armor Class while you wear\
this armor. You are considered trained with this ar-\
mor even if you lack training with Medium or Heavy\
armor.\

Original layout: [[../../srd-5.2.1/pages/220|page 220]].
