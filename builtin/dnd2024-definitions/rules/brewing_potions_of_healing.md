---
splat_schema: 1
splat_id: 34dbfb32-3426-56f7-aac8-6fad5adc572e
splat_kind: rule
splat_key: dnd2024:rules/brewing_potions_of_healing
splat_name: Brewing Potions of Healing
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/brewing_potions_of_healing
  parameters:
    tool:
      key: dnd2024:equipment/other_tools_herbalism_kit_5_gp
    tool_proficiency_required: true
    tool_use_required: true
    output:
      key: dnd2024:equipment/potion_of_healing_50_gp
    quantity: 1
    raw_material_gp: 25
    days: 1
    work_hours: 8
  implementation: dnd2024:rule_brewing_potions_of_healing
splat_contracts:
- id: dnd2024:rule_brewing_potions_of_healing
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - crafted_items
  events: []
  capabilities:
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rule_brewing_potions_of_healing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 46350c50e4d0ab4862bccd57eadd8d80216a89b91a48002378a4d803c0a518de
  parent_name: Brewing Potions of Healing
  line_span:
    first: 8991
    count: 5
splat_clauses:
- key: source_rules
  text: '# Brewing Potions of Healing A character who has proficiency with the Herbalism Kit can create
    a Potion of Healing. Doing so requires using that kit and 25 GP of raw material over the course of
    1 day (8 hours of work).'
  sha256: 4dc7c9e061abcdeee7445b47a86e9ef359d3d43b45888aeb24f5f44ecb01daa9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Brewing Potions of Healing

<!-- SRD page 103 -->

A character who has proficiency with the Herbalism\
Kit can create a Potion of Healing. Doing so requires\
using that kit and 25 GP of raw material over the\
course of 1 day (8 hours of work).\
