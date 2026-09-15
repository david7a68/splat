---
splat_schema: 1
splat_id: 1794c9f1-dec1-5bf7-a926-96724d9510fd
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_slippery_ice
splat_name: Slippery Ice
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/environmental_effects_slippery_ice
  parameters:
    terrain: difficult
    triggers:
    - first_entry_on_turn
    - start_turn_inside
    save:
      ability: dexterity
      dc: 10
    failure:
      condition: prone
  implementation: dnd2024:rules_environmental_effects_slippery_ice
splat_contracts:
- id: dnd2024:rules_environmental_effects_slippery_ice
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - movement.entered
  - turn.started
  capabilities:
  - environment
  - saving_throws
  - conditions
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 195
  engine_gaps: &id001
  - dnd2024:rules_environmental_effects_slippery_ice
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: dae6f0684f62fca9ec57669864a22de5e7f4375986ff56e195c1adefcb362617
  parent_name: Environmental Effects
  line_span:
    first: 9663
    count: 5
splat_clauses:
- key: source_rules
  text: '# Slippery Ice Slippery ice is Difficult Terrain. A creature that moves onto slippery ice for
    the first time on a turn or starts its turn there must succeed on a DC 10 Dexterity saving throw or
    have the Prone condition.'
  sha256: 6c64458b885684677113071aefa4ffd9fa1cade2784e6c11cd5b158e893f2052
  status: contract_specified
splat_requires_mechanics: *id001
---

# Slippery Ice

<!-- SRD page 195 -->

Slippery ice is Difficult Terrain. A creature that\
moves onto slippery ice for the first time on a turn\
or starts its turn there must succeed on a DC 10\
Dexterity saving throw or have the Prone condition.\
