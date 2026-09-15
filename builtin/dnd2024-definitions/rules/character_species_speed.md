---
splat_schema: 1
splat_id: d83b2d63-7f22-5c99-9cef-a4fb729ca7ab
splat_kind: rule
splat_key: dnd2024:rules/character_species_speed
splat_name: Speed
splat_origin:
  key: dnd2024:reference/pages/084
splat_sources:
- title: SRD 5.2.1
  locator: pp. 84
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/character_species_speed
  parameters:
    speed_source: selected_species
  implementation: dnd2024:rule_character_species_speed
splat_contracts:
- id: dnd2024:rule_character_species_speed
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
  - movement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 84
  engine_gaps: &id001
  - dnd2024:rule_character_species_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 4564542aae0aae260a307a9955f4d6db4c3b12771b36a868c9145be7dd0219d5
  parent_name: Character Species
  line_span:
    first: 7156
    count: 3
splat_clauses:
- key: source_rules
  text: '# Speed Your character’s species determines the character’s Speed.'
  sha256: e8510ce1430477679773320aa152c6a09bb84758843bdba661c67812c87e1b34
  status: contract_specified
splat_requires_mechanics: *id001
---

# Speed

<!-- SRD page 84 -->

Your character’s species determines the character’s\
Speed.\
