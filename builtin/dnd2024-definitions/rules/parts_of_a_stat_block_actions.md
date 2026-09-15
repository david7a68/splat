---
splat_schema: 1
splat_id: 03092efc-61c1-5adc-ac33-c78d75857b85
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_actions
splat_name: Actions
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_actions
  parameters:
    action_options:
      union:
      - stat_block_actions
      - universal_actions
  implementation: dnd2024:rules_parts_of_a_stat_block_actions
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_actions
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - action_options
  - effects
  - resource_updates
  events: []
  capabilities:
  - action_economy
  - movement
  - combat
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 256
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_actions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: b56d591b486bf7b54b4907f7f3a4ec01df48984f35e79081a840c32961be04e9
  parent_name: Parts of a Stat Block
  line_span:
    first: 11128
    count: 4
splat_clauses:
- key: source_rules
  text: '# Actions A monster can take the actions in this section or take one of the actions available
    to all creatures, as described in “Playing the Game.”'
  sha256: 5744579498093ca9092caf6fcf5dfa3b8dd6fc91d97e75a175c34026066cf2ba
  status: contract_specified
splat_requires_mechanics: *id001
---

# Actions

<!-- SRD page 256 -->

A monster can take the actions in this section or\
take one of the actions available to all creatures, as\
described in “Playing the Game.”\
