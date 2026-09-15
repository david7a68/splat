---
splat_schema: 1
splat_id: 56b40ce8-a49c-51f7-b098-238f909165d5
splat_kind: rule
splat_key: dnd2024:rules/exploration_carrying_objects
splat_name: Carrying Objects
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/exploration_carrying_objects
  parameters:
    ordinary_gear_weight_tracking: optional
    exceptional_load:
      triggers:
      - unusually_heavy_object
      - massive_object_count
      authority: gm
      enforce:
        key: dnd2024:glossary/carrying_capacity
  implementation: dnd2024:rules_exploration_carrying_objects
splat_contracts:
- id: dnd2024:rules_exploration_carrying_objects
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events: []
  capabilities:
  - inventory
  - carrying_capacity
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 12
  engine_gaps: &id001
  - dnd2024:rules_exploration_carrying_objects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 80fe9e53977b897f040ec77c52350c804a9d1d0c9276e92ca16bb05407555227
  parent_name: Exploration
  line_span:
    first: 689
    count: 7
splat_clauses:
- key: source_rules
  text: '# Carrying Objects You can usually carry your gear and treasure without worrying about the weight
    of those objects. If you try to haul an unusually heavy object or a massive number of lighter objects,
    the GM might require you to abide by the rules for carrying capacity in “Rules Glossary.”'
  sha256: 38c9d1fc5eda680aef2ac62cce9216daf8d86d71f82a0c3b91db397de9f3e621
  status: contract_specified
splat_requires_mechanics: *id001
---

# Carrying Objects

<!-- SRD page 12 -->

You can usually carry your gear and treasure with-\
out worrying about the weight of those objects. If\
you try to haul an unusually heavy object or a mas-\
sive number of lighter objects, the GM might require\
you to abide by the rules for carrying capacity in\
“Rules Glossary.”\
