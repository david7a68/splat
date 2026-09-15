---
splat_schema: 1
splat_id: b5e882b6-d88d-5cbb-9e36-b76184304cf5
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_step_2_character_origin
splat_name: 'Step 2: Character Origin'
splat_origin:
  key: dnd2024:reference/pages/019
splat_sources:
- title: SRD 5.2.1
  locator: pp. 19
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_step_2_character_origin
  parameters:
    choices:
      background: 1
      species: 1
      languages: 2
  implementation: dnd2024:rule_create_your_character_step_2_character_origin
splat_contracts:
- id: dnd2024:rule_create_your_character_step_2_character_origin
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
  - character_changes
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 19
  engine_gaps: &id001
  - dnd2024:rule_create_your_character_step_2_character_origin
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 80d692240248f27f1a1fb02d39a64b9404ee0a05aaad055885111cc66466e388
  parent_name: Create Your Character
  line_span:
    first: 1390
    count: 8
splat_clauses:
- key: source_rules
  text: '# Step 2: Character Origin Determining your character’s origin involves choosing a background,
    a species, and two languages. A character’s background represents the place and occupation that were
    most formative for the character. The combination of background, species, and languages provides fertile
    soil for your imagination as you ponder your character’s earliest days.'
  sha256: 02d633b67264ce5e358b288a41c73bbaa9b093ff0d11fb46b1bd290eae195abc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Step 2: Character Origin

<!-- SRD page 19 -->

Determining your character’s origin involves choos-\
ing a background, a species, and two languages.\
A character’s background represents the place\
and occupation that were most formative for the\
character. The combination of background, species,\
and languages provides fertile soil for your imagi-\
nation as you ponder your character’s earliest days.\
