---
splat_schema: 1
splat_id: 65cbc408-7f94-5095-9c76-06b2ec1df96e
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_18_aura_expansion
splat_name: 'Level 18: Aura Expansion'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_18_aura_expansion
  parameters:
    modifies_aura: paladin.aura_of_protection
    shape: emanation
    radius_feet: 30
    class_scope:
      class: paladin
      minimum_level: 18
  implementation: dnd2024:class_features_paladin_level_18_aura_expansion
splat_contracts:
- id: dnd2024:class_features_paladin_level_18_aura_expansion
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - aura.calculate
  capabilities:
  - character_features
  - auras
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_18_aura_expansion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 9f7ed7d4929aca708593a9eb3d0dc495f790a1c3cdea92f5d6964d96b8106bfe
  parent_name: Paladin
  line_span:
    first: 4656
    count: 2
splat_clauses:
- key: source_rules
  text: '# Level 18: Aura Expansion Your Aura of Protection is now a 30-foot Emanation.'
  sha256: dea5d5a7b2bb6a663c996342a5a3b24df8dd49383d9a7ac556cdb2253a4a405c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Aura Expansion

<!-- SRD page 55 -->

Your Aura of Protection is now a 30-foot Emanation.\
