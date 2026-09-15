---
splat_schema: 1
splat_id: 09fe0a5d-af15-5b04-8c16-627584d991bf
splat_kind: item
splat_key: dnd2024:items/periapt_of_proof_against_poison
splat_name: Periapt of Proof against Poison
splat_origin:
  key: dnd2024:reference/magic_item/periapt_of_proof_against_poison
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/periapt_of_proof_against_poison
  implementation: dnd2024:item_periapt_of_proof_against_poison
  parameters:
    equipment_state: worn
    condition_immunities:
    - poisoned
    damage_immunities:
    - poison
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_periapt_of_proof_against_poison
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
  - condition.pending
  - damage.pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f8cd279c9972a42106131cc442ffc19032ee8f03d8374552319689e5e5c7928d
  source_path: builtin/srd-5.2.1/magic_item/periapt_of_proof_against_poison.md
  pages:
  - 234
  engine_gaps: &id001
  - dnd2024:item_periapt_of_proof_against_poison
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: e8b060b7670d00e8b684454ba1eb74f70a154553de99483356d57770dd16aaa3
splat_clauses:
- key: source_rules
  text: '# Periapt of Proof against Poison Wondrous Item, Rare (Requires Attunement) This delicate silver
    chain has a brilliant-cut black gem pendant. While you wear it, you have Immunity to the Poisoned
    condition and Poison damage.'
  sha256: 6126ce4b3a5bbea19a80d38e370191e8eb4cd8ca318ecdb2d5273b61249fd8ca
  status: contract_specified
splat_requires_mechanics: *id001
---

# Periapt of Proof against Poison

<!-- SRD page 234 -->

Wondrous Item, Rare (Requires Attunement)\
This delicate silver chain has a brilliant-cut black\
gem pendant. While you wear it, you have Immunity\
to the Poisoned condition and Poison damage.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].
