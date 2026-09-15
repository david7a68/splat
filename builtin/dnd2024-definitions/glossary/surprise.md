---
splat_schema: 1
splat_id: a59eca66-9256-5433-90af-8fd98647cfe5
splat_kind: rule
splat_key: dnd2024:glossary/surprise
splat_name: Surprise
splat_origin:
  key: dnd2024:reference/glossary/surprise
splat_sources:
- title: SRD 5.2.1
  locator: pp. 189
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/surprise
  parameters:
    predicate: caught_unawares_by_combat_start
    initiative_roll: disadvantage
  implementation: dnd2024:glossary_surprise
splat_contracts:
- id: dnd2024:glossary_surprise
  inputs:
    subject:
      type: participant
      required: true
    caught_unawares:
      type: boolean
      required: true
  outputs:
  - roll_mode
  events:
  - initiative.rolled
  capabilities:
  - initiative
splat_conversion:
  status: specified
  category: glossary
  source_sha256: d3a09343bf43c23fa488298b701ab21dcfff5d90776e56fd8bb003aaaa39a312
  source_path: builtin/srd-5.2.1/glossary/surprise.md
  pages:
  - 189
  engine_gaps: &id001
  - dnd2024:glossary_surprise
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 406fcee106977fbad9892e324697b8a62a038ce6d40d0dbcefb08eeef36c711f
splat_clauses:
- key: source_rules
  text: '# Surprise If a creature is caught unawares by the start of combat, that creature is surprised,
    which causes it to have Disadvantage on its Initiative roll. See also “Playing the Game” (“Combat”).'
  sha256: 4898ed6ec18dd36e67565fd0416180dcaec132cef6f88abf3cf74d6737fed67f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Surprise

<!-- SRD page 189 -->

If a creature is caught unawares by the start of\
combat, that creature is surprised, which causes it\
to have Disadvantage on its Initiative roll. See also\
“Playing the Game” (“Combat”).\

Original layout: [[../../srd-5.2.1/pages/189|page 189]].
