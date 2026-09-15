---
splat_schema: 1
splat_id: e5333c93-9a9d-59e5-978c-651643bcd39c
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
splat_name: 'Level 20: Arcane Apotheosis'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
  parameters:
    requires_innate_sorcery_active: true
    free_metamagic:
      maximum_options: 1
      period: each_own_turn
      sorcery_point_cost: 0
  implementation: dnd2024:class_features_sorcerer_level_20_arcane_apotheosis
splat_contracts:
- id: dnd2024:class_features_sorcerer_level_20_arcane_apotheosis
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  events:
  - spell.metamagic_applied
  capabilities:
  - metamagic
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 66
  engine_gaps: &id001
  - dnd2024:class_features_sorcerer_level_20_arcane_apotheosis
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 35a4471f5f30f40d657a01e4133c1946e759c710cb4ef0ee31b497f34def0744
  parent_name: Sorcerer
  line_span:
    first: 5624
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 20: Arcane Apotheosis While your Innate Sorcery feature is active, you can use one Metamagic
    option on each of your turns without spending Sorcery Points on it.'
  sha256: 16990c589af37bd135f9a4ea62f6b04e75b4ede4935c1b3909a067e2bb6ca668
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Arcane Apotheosis

<!-- SRD page 66 -->

While your Innate Sorcery feature is active, you can\
use one Metamagic option on each of your turns\
without spending Sorcery Points on it.\
