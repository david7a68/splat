---
splat_schema: 1
splat_id: 38032089-6d47-58ee-827b-02e1b3bdbf00
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_18_elusive
splat_name: 'Level 18: Elusive'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_18_elusive
  parameters:
    unless:
      actor_condition: incapacitated
    attacks_against_actor_can_have_advantage: false
    class_scope:
      class: rogue
      minimum_level: 18
  implementation: dnd2024:class_features_rogue_level_18_elusive
splat_contracts:
- id: dnd2024:class_features_rogue_level_18_elusive
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
  - attack.roll
  capabilities:
  - character_features
  - conditions
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_18_elusive
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: bbe9fa321933b5e53b45000c63c83a05ce3de86cb993a7a7fea99fc8b67bf6b6
  parent_name: Rogue
  line_span:
    first: 5336
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 18: Elusive You’re so evasive that attackers rarely gain the upper hand against you.
    No attack roll can have Advantage against you unless you have the Incapacitated condition.'
  sha256: 2a3689604f856b62bc30d206459a0996c47f299683fb268121cca4cd67075e63
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Elusive

<!-- SRD page 63 -->

You’re so evasive that attackers rarely gain the\
upper hand against you. No attack roll can have\
Advantage against you unless you have the Incapac-\
itated condition.\
