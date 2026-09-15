---
splat_schema: 1
splat_id: 6baed617-b0e7-5653-835c-59bd1b1418b7
splat_kind: item
splat_key: dnd2024:items/medallion_of_thoughts
splat_name: Medallion of Thoughts
splat_origin:
  key: dnd2024:reference/magic_item/medallion_of_thoughts
splat_sources:
- title: SRD 5.2.1
  locator: pp. 230
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/medallion_of_thoughts
  implementation: dnd2024:item_medallion_of_thoughts
  parameters:
    equipped: worn
    charges:
      initial: 5
      maximum: 5
      recovery:
        event: time.dawn
        amount:
          roll:
            count: 1
            sides: 4
        cap_at_maximum: true
    spells:
    - spell:
        key: dnd2024:spells/detect_thoughts
      save_dc: 13
      charge_cost: 1
    attunement_required: true
splat_contracts:
- id: dnd2024:item_medallion_of_thoughts
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
  - spell.cast_pending
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - item_charges
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e0a20fb01bdb5f41de61b5e75602f12e5b77c20a91b9465f75e7ff18f2a327c5
  source_path: builtin/srd-5.2.1/magic_item/medallion_of_thoughts.md
  pages:
  - 230
  engine_gaps: &id001
  - dnd2024:item_medallion_of_thoughts
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 3ad59bc44f0159cd56725c45d595350f88316ca6995e1519fff6b0ca5cef228f
splat_clauses:
- key: source_rules
  text: '# Medallion of Thoughts Wondrous Item, Uncommon (Requires Attunement) The medallion has 5 charges.
    While wearing it, you can expend 1 charge to cast Detect Thoughts (save DC 13) from it. The medallion
    regains 1d4 expended charges daily at dawn.'
  sha256: d03d33778a7f0cef91487829b02786d03fcb066377368eac2f38e0b793bccdc0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Medallion of Thoughts

<!-- SRD page 230 -->

Wondrous Item, Uncommon (Requires Attunement)\
The medallion has 5 charges. While wearing it, you\
can expend 1 charge to cast Detect Thoughts (save\
DC 13) from it. The medallion regains 1d4 expended\
charges daily at dawn.\

Original layout: [[../../srd-5.2.1/pages/230|page 230]].
