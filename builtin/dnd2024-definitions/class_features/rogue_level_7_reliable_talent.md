---
splat_schema: 1
splat_id: 6c1c08de-c6cc-5b75-a110-5cd4f15ea2e2
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_7_reliable_talent
splat_name: 'Level 7: Reliable Talent'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_7_reliable_talent
  parameters:
    requires:
      test: ability_check
      uses_skill_or_tool_proficiency: true
    optional_d20_floor: 10
    class_scope:
      class: rogue
      minimum_level: 7
  implementation: dnd2024:class_features_rogue_level_7_reliable_talent
splat_contracts:
- id: dnd2024:class_features_rogue_level_7_reliable_talent
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - ability.check.rolled
  capabilities:
  - character_features
  - proficiencies
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_7_reliable_talent
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 3d63ba6a79d7af423100a724318f05c331398f50fa01b940533a3e7e0b2c2a3c
  parent_name: Rogue
  line_span:
    first: 5307
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 7: Reliable Talent Whenever you make an ability check that uses one of your skill or
    tool proficiencies, you can treat a d20 roll of 9 or lower as a 10.'
  sha256: 7284a69e787749393957291c0aaa89436f964756617ee44dc6d3a7dade7e93bc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Reliable Talent

<!-- SRD page 63 -->

Whenever you make an ability check that uses one\
of your skill or tool proficiencies, you can treat a\
d20 roll of 9 or lower as a 10.\
