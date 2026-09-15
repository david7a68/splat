---
splat_schema: 1
splat_id: ab984309-6e33-565e-a496-bdc20321584e
splat_kind: item
splat_key: dnd2024:items/universal_solvent
splat_name: Universal Solvent
splat_origin:
  key: dnd2024:reference/magic_item/universal_solvent
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/universal_solvent
  implementation: dnd2024:item_universal_solvent
  parameters:
    initial_ounces:
      roll:
        count: 1
        sides: 6
        add: 1
    apply:
      action: utilize
      ounces_minimum: 1
      surface: within_reach
      square_ft_adhesive_per_ounce: 1
      effect_timing: instantaneous
      operation: dissolve_adhesive
      includes:
        key: dnd2024:items/sovereign_glue
      consumes_applied_ounces: true
    attunement_required: false
splat_contracts:
- id: dnd2024:item_universal_solvent
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
  capabilities:
  - item_effects
  - dice
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 5c9e2d0251500a6890a591d11fbdf4a2e537617bd852d3ffa79a5e0f3abef671
  source_path: builtin/srd-5.2.1/magic_item/universal_solvent.md
  pages:
  - 250
  engine_gaps: &id001
  - dnd2024:item_universal_solvent
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 856f3194d403b3a05ceb716e12884eca53dc0c6dc730040a093b46549b4af15a
splat_clauses:
- key: source_rules
  text: '# Universal Solvent Wondrous Item, Legendary This tube holds milky liquid with a strong alcohol
    smell. When found, a tube contains 1d6 + 1 ounces. You can take a Utilize action to pour 1 or more
    ounces of solvent from the tube onto a surface within reach. Each ounce instantly dissolves up to
    1 square foot of adhesive it touches, including Sovereign Glue.'
  sha256: 3eee0cefdec501bc7a5d4b8c75f3c92be94aeb3a144e2d93d4902e04b48ec7d1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Universal Solvent

<!-- SRD page 250 -->

Wondrous Item, Legendary\
This tube holds milky liquid with a strong alcohol\
smell. When found, a tube contains 1d6 + 1 ounces.\
You can take a Utilize action to pour 1 or more\
ounces of solvent from the tube onto a surface\
within reach. Each ounce instantly dissolves up to 1\
square foot of adhesive it touches, including Sover-\
eign Glue.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]].
