---
splat_schema: 1
splat_id: 9743460d-03fc-5d9f-8e27-0bb09f864f0c
splat_kind: rule
splat_key: dnd2024:rules/magic_item_rarity
splat_name: Magic Item Rarity
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_rarity
  parameters:
    rarity_values_gp:
      common: 100
      uncommon: 400
      rare: 4000
      very_rare: 40000
      legendary: 200000
      artifact: null
    artifact_value: priceless
    consumable_non_scroll_multiplier: 0.5
    spell_scroll_value:
      op: multiply
      left: scribe_cost
      right: 2
  implementation: dnd2024:rules_magic_item_rarity
splat_contracts:
- id: dnd2024:rules_magic_item_rarity
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - item_attributes
  - actor_modifiers
  events: []
  capabilities:
  - equipment
  - armor_class
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 205
  engine_gaps: &id001
  - dnd2024:rules_magic_item_rarity
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 6981f4133bf7cc420d9ae2abc9b3443ffb7ce95626339eacd1d1628f652d0e25
  parent_name: Magic Item Rarity
  line_span:
    first: 10557
    count: 8
splat_clauses:
- key: source_rules
  text: '# Magic Item Rarity Every magic item has a rarity, which provides a rough measure of an item’s
    power relative to other magic items. The rarities are shown in the Magic Item Rarities and Values
    table. Common magic items, such as a Potion of Healing, are the most plentiful. Artifacts, such as
    the Dragon Orb, are priceless, unique, and difficult to acquire.'
  sha256: b5004aa5aab076dedeff9ac168dc77edeb5a589f24e396ba8ef5ed6e715e6dfc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magic Item Rarity

<!-- SRD page 205 -->

Every magic item has a rarity, which provides a\
rough measure of an item’s power relative to other\
magic items. The rarities are shown in the Magic\
Item Rarities and Values table.\
Common magic items, such as a Potion of Healing,\
are the most plentiful. Artifacts, such as the Dragon\
Orb, are priceless, unique, and difficult to acquire.\
