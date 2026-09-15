---
splat_schema: 1
splat_id: 184d7087-e103-52ea-9a0a-026f7e09b6bb
splat_kind: item
splat_key: dnd2024:items/headband_of_intellect
splat_name: Headband of Intellect
splat_origin:
  key: dnd2024:reference/magic_item/headband_of_intellect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 225
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/headband_of_intellect
  implementation: dnd2024:item_headband_of_intellect
  parameters:
    equipped: worn
    ability_floor:
      ability: intelligence
      score: 19
      requires_unmodified_score_below: 19
    attunement_required: true
splat_contracts:
- id: dnd2024:item_headband_of_intellect
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
  - ability_score.changed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f9269e69dd10ccdb82b0db146b34a1c1f50c570139fc2a4de9cba1f5a88dcb2b
  source_path: builtin/srd-5.2.1/magic_item/headband_of_intellect.md
  pages:
  - 225
  engine_gaps: &id001
  - dnd2024:item_headband_of_intellect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 750b771aaf18b1343e32ab9c9ab203290ff2f76ecc44b971757e64128dde4bac
splat_clauses:
- key: source_rules
  text: '# Headband of Intellect Wondrous Item, Uncommon (Requires Attunement) Your Intelligence is 19
    while you wear this headband. It has no effect on you if your Intelligence is 19 or higher without
    it.'
  sha256: 633b929c0176287a6890e058ad610a9455eff75ac2b28a8633154ba2140b5de7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Headband of Intellect

<!-- SRD page 225 -->

Wondrous Item, Uncommon (Requires Attunement)\
Your Intelligence is 19 while you wear this head-\
band. It has no effect on you if your Intelligence is\
19 or higher without it.\

Original layout: [[../../srd-5.2.1/pages/225|page 225]].
