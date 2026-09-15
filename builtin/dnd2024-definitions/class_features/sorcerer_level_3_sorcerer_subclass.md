---
splat_schema: 1
splat_id: 7ef240a0-6697-5584-9b17-21cb393fa82f
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
splat_name: 'Level 3: Sorcerer Subclass'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
  parameters:
    subclass_choice:
      count: 1
      class: sorcerer
    feature_grant_policy:
      class_level_at_least_feature_level: true
      events:
      - subclass.selected
      - character.level_gained
      persistent: true
    class_scope:
      class: sorcerer
      minimum_level: 3
  implementation: dnd2024:class_features_sorcerer_level_3_sorcerer_subclass
splat_contracts:
- id: dnd2024:class_features_sorcerer_level_3_sorcerer_subclass
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
  - 66
  engine_gaps: &id001
  - dnd2024:class_features_sorcerer_level_3_sorcerer_subclass
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 5a7fbe0341a99d78d88c248858c0c8c1a7d93d9f6f421b37727d572dc7f1c879
  parent_name: Sorcerer
  line_span:
    first: 5594
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 3: Sorcerer Subclass You gain a Sorcerer subclass of your choice. The Draconic Sorcery
    subclass is detailed after this class’s description. A subclass is a specialization that grants you
    features at certain Sorcerer levels. For the rest of your career, you gain each of your subclass’s
    features that are of your Sorcerer level or lower.'
  sha256: b65a8d2fb2be2a441004b0c58d5e3067a1cdb9abdfe690917d3849146ea206bc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Sorcerer Subclass

<!-- SRD page 66 -->

You gain a Sorcerer subclass of your choice. The\
Draconic Sorcery subclass is detailed after this\
class’s description. A subclass is a specialization\
that grants you features at certain Sorcerer levels.\
For the rest of your career, you gain each of your\
subclass’s features that are of your Sorcerer level or\
lower.\
