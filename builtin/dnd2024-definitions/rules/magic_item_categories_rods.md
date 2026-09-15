---
splat_schema: 1
splat_id: 13727e86-eb5f-51e5-8cb5-68e0d06f0bbf
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_rods
splat_name: Rods
splat_origin:
  key: dnd2024:reference/pages/204
splat_sources:
- title: SRD 5.2.1
  locator: pp. 204
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_categories_rods
  parameters:
    rod:
      typical_weight_lb:
        minimum: 2
        maximum: 5
      arcane_focus: true
      description_can_override: true
  implementation: dnd2024:rules_magic_item_categories_rods
splat_contracts:
- id: dnd2024:rules_magic_item_categories_rods
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
  - 204
  engine_gaps: &id001
  - dnd2024:rules_magic_item_categories_rods
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 8f6d196a01728805640f4e8009cc7c596c985a378ca893e780075c321dfd0ab1
  parent_name: Magic Item Categories
  line_span:
    first: 10477
    count: 6
splat_clauses:
- key: source_rules
  text: '# Rods An item in the Rod category is a scepter usually made of metal, wood, or bone. A typical
    rod weighs 2 to 5 pounds. Unless its description notes otherwise, a rod can be used as an Arcane Focus.'
  sha256: 5accd980cefe64547e8d91517169513423689cc86bcd00360fa1e83541bd6815
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rods

<!-- SRD page 204 -->

An item in the Rod category is a scepter usually\
made of metal, wood, or bone. A typical rod weighs\
2 to 5 pounds.\
Unless its description notes otherwise, a rod can\
be used as an Arcane Focus.\
