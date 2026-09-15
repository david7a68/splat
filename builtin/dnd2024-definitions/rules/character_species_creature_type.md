---
splat_schema: 1
splat_id: db6641be-bec7-5f9e-bb5b-529bab8bd26a
splat_kind: rule
splat_key: dnd2024:rules/character_species_creature_type
splat_name: Creature Type
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/character_species_creature_type
  parameters:
    creature_type_source: species
    srd_species_creature_type: humanoid
  implementation: dnd2024:rules_character_species_creature_type
splat_contracts:
- id: dnd2024:rules_character_species_creature_type
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
  - 83
  engine_gaps: &id001
  - dnd2024:rules_character_species_creature_type
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: e415d9dd8aaadd2e8f454cdd59716bbf91ad734f2226aa7bdf4c4d5a827dfd01
  parent_name: Character Species
  line_span:
    first: 7144
    count: 6
splat_clauses:
- key: source_rules
  text: '# Creature Type A character’s species determines the character’s creature type, which is described
    in “Rules Glossary.” Every species in “Character Origins” is Humanoid; playable non-Humanoid species
    appear in other books.'
  sha256: 68068fe05981e285864a9d5fa18a4914b88ebc968cba8c7c7ec3e627bbc1f005
  status: contract_specified
splat_requires_mechanics: *id001
---

# Creature Type

<!-- SRD page 83 -->

A character’s species determines the character’s\
creature type, which is described in “Rules Glos-\
sary.” Every species in “Character Origins” is Hu-\
manoid; playable non-Humanoid species appear in\
other books.\
