---
splat_schema: 1
splat_id: cc79a993-f37a-5d1f-8fa0-80815f9f7787
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_strong_wind
splat_name: Strong Wind
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/environmental_effects_strong_wind
  parameters:
    ranged_weapon_attacks: disadvantage
    extinguish_open_flames: true
    disperse_fog: true
    flying_creature_end_turn:
      must: land
      otherwise: fall
    desert_sandstorm:
      gm_option: true
      wisdom_perception: disadvantage
  implementation: dnd2024:rule_environmental_effects_strong_wind
splat_contracts:
- id: dnd2024:rule_environmental_effects_strong_wind
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - turn.ended
  capabilities:
  - environment
  - movement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 195
  engine_gaps: &id001
  - dnd2024:rule_environmental_effects_strong_wind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: d693d47d66f17b3f65f5aca3634ced73431b28dbe329bf4889375824d9f8d44e
  parent_name: Environmental Effects
  line_span:
    first: 9668
    count: 8
splat_clauses:
- key: source_rules
  text: '# Strong Wind Strong wind imposes Disadvantage on ranged attack rolls with weapons. It also extinguishes
    open flames and disperses fog. A flying creature in a strong wind must land at the end of its turn
    or fall. A strong wind in a desert can create a sandstorm that imposes Disadvantage on Wisdom (Perception)
    checks.'
  sha256: 3e4d1e9f4bfdc257bd5c23690752d5374b80b7a01a484b56893f4d5794a3c608
  status: contract_specified
splat_requires_mechanics: *id001
---

# Strong Wind

<!-- SRD page 195 -->

Strong wind imposes Disadvantage on ranged at-\
tack rolls with weapons. It also extinguishes open\
flames and disperses fog. A flying creature in a\
strong wind must land at the end of its turn or fall.\
A strong wind in a desert can create a sandstorm\
that imposes Disadvantage on Wisdom (Perception)\
checks.\
