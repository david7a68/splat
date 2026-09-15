---
splat_schema: 1
splat_id: c3617baa-f4ba-5cd0-949f-5afb9c1525d8
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_3_rogue_subclass
splat_name: 'Level 3: Rogue Subclass'
splat_origin:
  key: dnd2024:reference/pages/062
splat_sources:
- title: SRD 5.2.1
  locator: pp. 62
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_3_rogue_subclass
  parameters:
    subclass_choice:
      count: 1
      class: rogue
    feature_grant_policy:
      class_level_at_least_feature_level: true
      events:
      - subclass.selected
      - character.level_gained
      persistent: true
    class_scope:
      class: rogue
      minimum_level: 3
  implementation: dnd2024:class_features_rogue_level_3_rogue_subclass
splat_contracts:
- id: dnd2024:class_features_rogue_level_3_rogue_subclass
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - subclass
  - feature_grants
  events:
  - subclass.selected
  - character.level_gained
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 62
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_3_rogue_subclass
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: a870a062db0a9817606ea480b8786ee37deb4107dae0c0a675c6764e05f001be
  parent_name: Rogue
  line_span:
    first: 5249
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 3: Rogue Subclass You gain a Rogue subclass of your choice. The Thief subclass is detailed
    after this class’s description. A subclass is a specialization that grants you features at certain
    Rogue levels. For the rest of your career, you gain each of your subclass’s features that are of your
    Rogue level or lower.'
  sha256: 5efc0af01d0e7640a3f7d914160d6a02d3f5a813a8ee6787f9b71f243ee27f05
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Rogue Subclass

<!-- SRD page 62 -->

You gain a Rogue subclass of your choice. The Thief\
subclass is detailed after this class’s description. A\
subclass is a specialization that grants you features\
at certain Rogue levels. For the rest of your career,\
you gain each of your subclass’s features that are of\
your Rogue level or lower.\
