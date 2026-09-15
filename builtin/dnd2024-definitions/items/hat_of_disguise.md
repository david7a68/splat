---
splat_schema: 1
splat_id: c10b97e2-c908-52a4-96b5-95ad3403733b
splat_kind: item
splat_key: dnd2024:items/hat_of_disguise
splat_name: Hat of Disguise
splat_origin:
  key: dnd2024:reference/magic_item/hat_of_disguise
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/hat_of_disguise
  implementation: dnd2024:item_hat_of_disguise
  parameters:
    equipment_state: worn
    spells:
    - spell:
        key: dnd2024:spells/disguise_self
      end_on:
      - hat_removed
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_hat_of_disguise
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
  source_sha256: 7c06996abac739944f920ba131ca9aa38a1d7d8dfcb93eb52ff9c968c3377d43
  source_path: builtin/srd-5.2.1/magic_item/hat_of_disguise.md
  pages:
  - 224
  engine_gaps: &id001
  - dnd2024:item_hat_of_disguise
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: d899988acba2d39235e370ff78fe22cec45ca6260ce811b8a1cfcf52f8471b22
splat_clauses:
- key: source_rules
  text: '# Hat of Disguise Wondrous Item, Uncommon (Requires Attunement) While wearing this hat, you can
    cast the Disguise Self spell. The spell ends if the hat is removed.'
  sha256: 761b0bacd42555571f20fe4afd7d752530813662fc7fcbcd57f8c48f772b6165
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hat of Disguise

<!-- SRD page 224 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this hat, you can cast the Disguise\
Self spell. The spell ends if the hat is removed.\

Original layout: [[../../srd-5.2.1/pages/224|page 224]].
