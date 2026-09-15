---
splat_schema: 1
splat_id: 4aa574cc-7fbf-59db-8e63-eac5762dd36d
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_3_warlock_subclass
splat_name: 'Level 3: Warlock Subclass'
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warlock_level_3_warlock_subclass
  parameters:
    subclass_choice:
      count: 1
      class: warlock
    feature_grant_policy:
      class_level_at_least_feature_level: true
      events:
      - subclass.selected
      - character.level_gained
      persistent: true
    class_scope:
      class: warlock
      minimum_level: 3
  implementation: dnd2024:class_features_warlock_level_3_warlock_subclass
splat_contracts:
- id: dnd2024:class_features_warlock_level_3_warlock_subclass
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
  - 72
  engine_gaps: &id001
  - dnd2024:class_features_warlock_level_3_warlock_subclass
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: f758f3052b264f1e88faf19da93985aa38722d3d8852b2a8f035f3e766f13ac2
  parent_name: Warlock
  line_span:
    first: 6095
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 3: Warlock Subclass You gain a Warlock subclass of your choice. The Fiend Patron subclass
    is detailed after this class’s description. A subclass is a specialization that grants you features
    at certain Warlock levels. For the rest of your career, you gain each of your subclass’s features
    that are of your Warlock level or lower.'
  sha256: 979ae81cc308d8a194c5b6ef5006a66c6ce48ecda395cfee26a52708dfc1a868
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Warlock Subclass

<!-- SRD page 72 -->

You gain a Warlock subclass of your choice. The\
Fiend Patron subclass is detailed after this class’s\
description. A subclass is a specialization that\
grants you features at certain Warlock levels. For\
the rest of your career, you gain each of your sub-\
class’s features that are of your Warlock level or\
lower.\
