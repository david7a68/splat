---
splat_schema: 1
splat_id: 1bb954db-747e-57e9-9c96-10e6c8bceed5
splat_kind: rule
splat_key: dnd2024:rules/rogue_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/rogue_as_a_level_1_character
  parameters:
    class_definition:
      key: dnd2024:classes/rogue
    grant_core_traits: all
    grant_features:
      class_level: 1
      from: class_definition
  implementation: dnd2024:rules_rogue_as_a_level_1_character
splat_contracts:
- id: dnd2024:rules_rogue_as_a_level_1_character
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  engine_gaps: &id001
  - dnd2024:rules_rogue_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 0eaa42e153b9b95d97a2c265e0f777aac23a50a3ea7facb90676eb6e601b7777
  parent_name: Rogue
  line_span:
    first: 5149
    count: 4
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Rogue Traits table. • Gain the Rogue’s
    level 1 features, which are listed in the Rogue Features table.'
  sha256: 1e3bd2f5d24d23485a8656cc21119809b672de815cb8dbce7477a0355383bc62
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 61 -->

•  Gain all the traits in the Core Rogue Traits table.\
•  Gain the Rogue’s level 1 features, which are listed\
in the Rogue Features table.\
