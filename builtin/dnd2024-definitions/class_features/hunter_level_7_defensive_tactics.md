---
splat_schema: 1
splat_id: 4ce22736-a27c-56e4-a2f5-8a49d15fa34b
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_7_defensive_tactics
splat_name: 'Level 7: Defensive Tactics'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/hunter_level_7_defensive_tactics
  parameters:
    choice:
      count: 1
      options:
      - id: escape_the_horde
        opportunity_attacks_against_self: disadvantage
      - id: multiattack_defense
        trigger: creature.attack_roll_hits_self
        source_further_attacks_against_self: disadvantage
        until: current_turn_end
    replace_choice:
      events:
      - rest.short_rest_completed
      - rest.long_rest_completed
      with: other_option
  implementation: dnd2024:class_features_hunter_level_7_defensive_tactics
splat_contracts:
- id: dnd2024:class_features_hunter_level_7_defensive_tactics
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - choices
  events:
  - attack.pending
  - attack.hit
  - rest.short_rest_completed
  - rest.long_rest_completed
  capabilities:
  - character_choices
  - attacks
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  engine_gaps: &id001
  - dnd2024:class_features_hunter_level_7_defensive_tactics
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 03583430950daf707abb3e854a5d3f7e9aabf892618941588a2c3f192689bdec
  parent_name: Hunter
  line_span:
    first: 5107
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 7: Defensive Tactics You gain one of the following feature options of your choice. Whenever
    you finish a Short or Long Rest, you can replace the chosen option with the other one. Escape the
    Horde. Opportunity Attacks have Disadvantage against you. Multiattack Defense. When a creature hits
    you with an attack roll, that creature has Disadvantage on all other attack rolls against you this
    turn.'
  sha256: a830c39f538f58d2b85baf5e8975f936f60535ad3b9e0110ef7e3f0aa9ac8403
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Defensive Tactics

<!-- SRD page 61 -->

You gain one of the following feature options of your\
choice. Whenever you finish a Short or Long Rest,\
you can replace the chosen option with the other\
one.\
Escape the Horde. Opportunity Attacks have Dis-\
advantage against you.\
Multiattack Defense. When a creature hits you\
with an attack roll, that creature has Disadvantage\
on all other attack rolls against you this turn.\
