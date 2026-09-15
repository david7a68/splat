---
splat_schema: 1
splat_id: dc1a41b8-fe17-5e55-9061-81c41e8570cb
splat_kind: item
splat_key: dnd2024:items/mithral_armor
splat_name: Mithral Armor
splat_origin:
  key: dnd2024:reference/magic_item/mithral_armor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 231
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/mithral_armor
  implementation: dnd2024:item_mithral_armor
  parameters:
    wear_under_normal_clothes: true
    remove_stealth_disadvantage: true
    remove_strength_requirement: true
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_armor
splat_contracts:
- id: dnd2024:item_mithral_armor
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
  - check.pending
  capabilities:
  - inventory
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 865ab55404ea5f52bc224275b760cc20e37b8baaefef0695259dcdec7661c863
  source_path: builtin/srd-5.2.1/magic_item/mithral_armor.md
  pages:
  - 231
  engine_gaps: &id001
  - dnd2024:item_mithral_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 6a265ad4aebd34d259aa18b085cc5a87a1552ac0f985494da1f744f2302b4a23
splat_clauses:
- key: source_rules
  text: '# Mithral Armor Armor (Any Medium or Heavy, Except Hide Armor), Uncommon Mithral is a light,
    flexible metal. Armor made of this substance can be worn under normal clothes. If the armor normally
    imposes Disadvantage on Dexterity (Stealth) checks or has a Strength requirement, the mithral version
    of the armor doesn’t.'
  sha256: 8378280047aa62c44bc4fc293c10eb30c2aa6a066d47399165c1abae2c2c782f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mithral Armor

<!-- SRD page 231 -->

Armor (Any Medium or Heavy, Except Hide Armor),\
Uncommon\
Mithral is a light, flexible metal. Armor made of this\
substance can be worn under normal clothes. If the\
armor normally imposes Disadvantage on Dexterity\
(Stealth) checks or has a Strength requirement, the\
mithral version of the armor doesn’t.\

Original layout: [[../../srd-5.2.1/pages/231|page 231]].
