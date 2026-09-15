---
splat_schema: 1
splat_id: eea6db6d-0573-53f9-8b0f-50a625d5f69c
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_3_bard_subclass
splat_name: 'Level 3: Bard Subclass'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/bard_level_3_bard_subclass
  parameters:
    subclass_choice:
      count: 1
      class: bard
    feature_grant_policy:
      class_level_at_least_feature_level: true
      events:
      - subclass.selected
      - character.level_gained
      persistent: true
    class_scope:
      class: bard
      minimum_level: 3
  implementation: dnd2024:class_features_bard_level_3_bard_subclass
splat_contracts:
- id: dnd2024:class_features_bard_level_3_bard_subclass
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
  - 32
  engine_gaps: &id001
  - dnd2024:class_features_bard_level_3_bard_subclass
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: e392f0732c77b1315d5f511e7149e08883fa9009695de957f90d7837460bbac4
  parent_name: Bard
  line_span:
    first: 2662
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 3: Bard Subclass You gain a Bard subclass of your choice. The College of Lore subclass
    is detailed after this class’s description. A subclass is a specialization that grants you features
    at certain Bard levels. For the rest of your career, you gain each of your subclass’s features that
    are of your Bard level or lower.'
  sha256: fc16cefb803d77b7060af56fa744742b0ea0a0471a40b520baec306275c37d63
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Bard Subclass

<!-- SRD page 32 -->

You gain a Bard subclass of your choice. The College\
of Lore subclass is detailed after this class’s descrip-\
tion. A subclass is a specialization that grants you\
features at certain Bard levels. For the rest of your\
career, you gain each of your subclass’s features\
that are of your Bard level or lower.\
