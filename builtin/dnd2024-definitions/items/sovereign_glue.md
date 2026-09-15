---
splat_schema: 1
splat_id: ee140b38-12f7-5951-bda1-66881e7118fc
splat_kind: item
splat_key: dnd2024:items/sovereign_glue
splat_name: Sovereign Glue
splat_origin:
  key: dnd2024:reference/magic_item/sovereign_glue
splat_sources:
- title: SRD 5.2.1
  locator: pp. 244
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/sovereign_glue
  implementation: dnd2024:item_sovereign_glue
  parameters:
    initial_ounces:
      roll:
        count: 1
        sides: 6
        add: 1
    storage:
      container_types:
      - jar
      - flask
      interior_coating:
        key: dnd2024:items/oil_of_slipperiness
    apply:
      action: utilize
      ounces_per_action: 1
      square_ft_per_ounce: 1
      bond_targets: two_objects
      set_time:
        amount: 1
        unit: minutes
      consumes_applied_ounces: true
    bond:
      permanent: true
      broken_only_by:
      - key: dnd2024:items/universal_solvent
      - key: dnd2024:items/oil_of_etherealness
      - key: dnd2024:spells/wish
    attunement_required: false
splat_contracts:
- id: dnd2024:item_sovereign_glue
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
  - time.elapsed
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - inventory_containers
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: a4b1c044cf5d0cb419aff18a3c9eb574e875aa86efdb41b11fdef413d134d331
  source_path: builtin/srd-5.2.1/magic_item/sovereign_glue.md
  pages:
  - 244
  engine_gaps: &id001
  - dnd2024:item_sovereign_glue
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: c7fb9d87f9fcd22dc69496ca5b244eb30aa9540e040afb93de7650298d3c7913
splat_clauses:
- key: source_rules
  text: '# Sovereign Glue Wondrous Item, Legendary This viscous, milky-white substance can form a permanent
    adhesive bond between any two objects. It must be stored in a jar or flask that has been coated inside
    with Oil of Slipperiness. When found, a container contains 1d6 + 1 ounces. One ounce of the glue can
    cover a 1-foot square surface. Applying an ounce of Sovereign Glue takes a Utilize action, and the
    applied glue takes 1 minute to set. Once it has done so, the bond it creates can be broken only by
    the application of Universal Solvent or Oil of Etherealness, or with a Wish spell.'
  sha256: 4ad777f6a559d053f1878ee6b13fee5678f9f395ee4650f45eb22f3b417d4895
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sovereign Glue

<!-- SRD page 244 -->

Wondrous Item, Legendary\
This viscous, milky-white substance can form a per-\
manent adhesive bond between any two objects. It\
must be stored in a jar or flask that has been coated\
inside with Oil of Slipperiness. When found, a con-\
tainer contains 1d6 + 1 ounces.\
One ounce of the glue can cover a 1-foot square\
surface. Applying an ounce of Sovereign Glue takes\
a Utilize action, and the applied glue takes 1 minute\
to set. Once it has done so, the bond it creates can be\
broken only by the application of Universal Solvent\
or Oil of Etherealness, or with a Wish spell.\

Original layout: [[../../srd-5.2.1/pages/244|page 244]].
