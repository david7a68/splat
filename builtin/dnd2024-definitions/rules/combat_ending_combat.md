---
splat_schema: 1
splat_id: 985f5870-52a3-5c24-beca-6d3d6c8111d2
splat_kind: rule
splat_key: dnd2024:rules/combat_ending_combat
splat_name: Ending Combat
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_ending_combat
  parameters:
    end_conditions:
      any:
      - side_killed
      - side_unconscious
      - side_surrendered
      - side_fled
      - mutual_agreement
  implementation: dnd2024:rules_combat_ending_combat
splat_contracts:
- id: dnd2024:rules_combat_ending_combat
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
  - 14
  engine_gaps: &id001
  - dnd2024:rules_combat_ending_combat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 85537868173ef7d681b9cd179800e77d18808ec8ab5435f183a5f1384dc0d1c9
  parent_name: Combat
  line_span:
    first: 878
    count: 5
splat_clauses:
- key: source_rules
  text: '# Ending Combat Combat ends when one side or the other is defeated, which can mean the creatures
    are killed or knocked out or have surrendered or fled. Combat can also end when both sides agree to
    end it.'
  sha256: 20aff0823da784b29b44a996d33fff5d01f88961acd3d897a465bd4929b98ec3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ending Combat

<!-- SRD page 14 -->

Combat ends when one side or the other is defeated,\
which can mean the creatures are killed or knocked\
out or have surrendered or fled. Combat can also\
end when both sides agree to end it.\
