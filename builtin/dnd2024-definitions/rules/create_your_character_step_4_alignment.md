---
splat_schema: 1
splat_id: cf637505-115a-5d75-885e-759441477936
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_step_4_alignment
splat_name: 'Step 4: Alignment'
splat_origin:
  key: dnd2024:reference/pages/021
splat_sources:
- title: SRD 5.2.1
  locator: pp. 21
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_step_4_alignment
  parameters:
    choose_count: 1
    choices:
    - lawful_good
    - neutral_good
    - chaotic_good
    - lawful_neutral
    - neutral
    - chaotic_neutral
    - lawful_evil
    - neutral_evil
    - chaotic_evil
    default_evil_allowed: false
    evil_exception_authority: gm
  implementation: dnd2024:rules_create_your_character_step_4_alignment
splat_contracts:
- id: dnd2024:rules_create_your_character_step_4_alignment
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - alignment
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 21
  engine_gaps: &id001
  - dnd2024:rules_create_your_character_step_4_alignment
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 084b5609488d8b9a98c1d6cb3472bbe65d898946c53d5158ab8f043c289e5c66
  parent_name: Create Your Character
  line_span:
    first: 1577
    count: 6
splat_clauses:
- key: source_rules
  text: '# Step 4: Alignment Choose your character’s alignment from the options below, and note it on
    your character sheet. The game assumes that player characters aren’t of an evil alignment. Check with
    your GM before making an evil character.'
  sha256: 2d6e8770d95d16ee77acdc86b9efd0b267d9795547b48dd83961ef1e721ad440
  status: contract_specified
splat_requires_mechanics: *id001
---

# Step 4: Alignment

<!-- SRD page 21 -->

Choose your character’s alignment from the options\
below, and note it on your character sheet.\
The game assumes that player characters aren’t of\
an evil alignment. Check with your GM before mak-\
ing an evil character.\
