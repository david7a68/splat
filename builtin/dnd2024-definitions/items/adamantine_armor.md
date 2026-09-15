---
splat_schema: 1
splat_id: 40eb362b-1778-5ef8-a9ca-62e90cc1c81f
splat_kind: item
splat_key: dnd2024:items/adamantine_armor
splat_name: Adamantine Armor
splat_origin:
  key: dnd2024:reference/magic_item/adamantine_armor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/adamantine_armor
  implementation: dnd2024:item_adamantine_armor
  parameters:
    equipment_state: worn
    critical_hits_received:
      replace: normal_hit
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_adamantine_armor
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
  outputs:
  - modifiers
  events:
  - equipment.state_changed
  capabilities:
  - equipment
  - derived_statistics
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: c4639eed16e1475e8bf1e248130f70cff5bd01f3c41dfdfe1b04e4af50cb2e88
  source_path: builtin/srd-5.2.1/magic_item/adamantine_armor.md
  pages:
  - 209
  engine_gaps: &id001
  - dnd2024:item_adamantine_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 75d05e87b156630bb4518cbf6094d4585733ccead8b1190e55f1f620ec69b3c8
splat_clauses:
- key: source_rules
  text: '# Adamantine Armor Armor (Any Medium or Heavy, Except Hide Armor), Uncommon This suit of armor
    is reinforced with adamantine, one of the hardest substances in existence. While you’re wearing it,
    any Critical Hit against you becomes a normal hit.'
  sha256: 3d4019f94c17367d39934ebc58f234ab1f2802960a571c5c40cb6a1e59277b6c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adamantine Armor

<!-- SRD page 209 -->

Armor (Any Medium or Heavy, Except Hide Armor),\
Uncommon\
This suit of armor is reinforced with adamantine,\
one of the hardest substances in existence. While\
you’re wearing it, any Critical Hit against you be-\
comes a normal hit.\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].
