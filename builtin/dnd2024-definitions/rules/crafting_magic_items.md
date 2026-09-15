---
splat_schema: 1
splat_id: 9d5ca422-452e-5972-b406-18b8d78a911d
splat_kind: rule
splat_key: dnd2024:rules/crafting_magic_items
splat_name: Crafting Magic Items
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/crafting_magic_items
  parameters:
    special_crafting_rule_exceptions:
      potion_of_healing:
        key: dnd2024:rules/brewing_potions_of_healing
      spell_scroll:
        key: dnd2024:rules/scribing_spell_scrolls
    other_items_rule_scope: magic_item_crafting
    requirement_rules:
    - key: dnd2024:rules/crafting_magic_items_arcana_proficiency
    - key: dnd2024:rules/crafting_magic_items_tools
    - key: dnd2024:rules/crafting_magic_items_spells
    - key: dnd2024:rules/crafting_magic_items_time_and_cost
  implementation: dnd2024:rules_crafting_magic_items
splat_contracts:
- id: dnd2024:rules_crafting_magic_items
  inputs:
    item:
      type: definition
      required: true
  outputs:
  - crafting_rule_selection
  events: []
  capabilities:
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  engine_gaps: &id001
  - dnd2024:rules_crafting_magic_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 4e812b8a50534c59d320ecb6a14872a4847ddf61ea0c1101b4a1e1265ed6d03b
  parent_name: Crafting Magic Items
  line_span:
    first: 10658
    count: 6
splat_clauses:
- key: source_rules
  text: '# Crafting Magic Items “Equipment” contains rules on brewing Potions of Healing and scribing
    Spell Scrolls. To create other magic items, follow the rules below. In these rules, “you” refers to
    the character crafting the magic item.'
  sha256: 6579947fad6771f3f5461111c20cc3a249c3c1c2afdef8ddcfb7e2a5a3b176ba
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crafting Magic Items

<!-- SRD page 206 -->

“Equipment” contains rules on brewing Potions of\
Healing and scribing Spell Scrolls. To create other\
magic items, follow the rules below. In these rules,\
“you” refers to the character crafting the magic\
item.\
