---
splat_schema: 1
splat_id: 8ab8a47f-5879-5a45-9bdd-e22e6bf6585f
splat_kind: rule
splat_key: dnd2024:glossary/difficulty_class
splat_name: Difficulty Class
splat_origin:
  key: dnd2024:reference/glossary/difficulty_class
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/difficulty_class
  parameters:
    target_number_for:
    - ability_check
    - saving_throw
  implementation: dnd2024:glossary_difficulty_class
splat_contracts:
- id: dnd2024:glossary_difficulty_class
  inputs:
    dc:
      type: integer
      required: true
    test_kind:
      type: choice
      required: true
  outputs:
  - target_number
  events: []
  capabilities:
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 78061dd246ee9ed493f1ba7e0040f32f3527682468b016ce07134883bc3acd9b
  source_path: builtin/srd-5.2.1/glossary/difficulty_class.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_difficulty_class
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: ba040057248f737e63a7e124b6108b1121e6641602bc4f25ec55af5b12909bae
splat_clauses:
- key: source_rules
  text: '# Difficulty Class A Difficulty Class (DC) is the target number for an ability check or a saving
    throw. See also “Playing the Game” (“D20 Tests”).'
  sha256: da12898053e32e751d35fbb6cd338d99d689c6ce745108bb54fdc66c679e2487
  status: contract_specified
splat_requires_mechanics: *id001
---

# Difficulty Class

<!-- SRD page 181 -->

A Difficulty Class (DC) is the target number for an\
ability check or a saving throw. See also “Playing the\
Game” (“D20 Tests”).\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].
