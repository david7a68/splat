---
splat_schema: 1
splat_id: f9838231-2dc8-5eed-9fb3-4a4345633755
splat_kind: rule
splat_key: dnd2024:rules/warlock_whispers_of_the_grave
splat_name: Whispers of the Grave
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_whispers_of_the_grave
  parameters:
    warlock_level_minimum: 7
    cast:
      spell:
        key: dnd2024:spells/speak_with_dead
      spell_slot_cost: 0
      target: spell_definition
      uses: unlimited
  implementation: dnd2024:rule_warlock_whispers_of_the_grave
splat_contracts:
- id: dnd2024:rule_warlock_whispers_of_the_grave
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - feature_effects
  events: []
  capabilities:
  - invocations
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 74
  engine_gaps: &id001
  - dnd2024:rule_warlock_whispers_of_the_grave
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 8e3dae594fcfbd3cc9e9cd7e15741a73cb6d28fca4cfc35fb167eeb090840053
  parent_name: Warlock
  line_span:
    first: 6364
    count: 4
splat_clauses:
- key: source_rules
  text: '# Whispers of the Grave Prerequisite: Level 7+ Warlock You can cast Speak with Dead without expending
    a spell slot.'
  sha256: 480d229202475453e5d254f8cb050cb32f17db86ebe394a48a258390e5f6dc05
  status: contract_specified
splat_requires_mechanics: *id001
---

# Whispers of the Grave

<!-- SRD page 74 -->

Prerequisite: Level 7+ Warlock\
You can cast Speak with Dead without expending a\
spell slot.\
