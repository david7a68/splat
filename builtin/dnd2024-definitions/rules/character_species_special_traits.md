---
splat_schema: 1
splat_id: 82876aba-1989-5725-996a-87a4669768f3
splat_kind: rule
splat_key: dnd2024:rules/character_species_special_traits
splat_name: Special Traits
splat_origin:
  key: dnd2024:reference/pages/084
splat_sources:
- title: SRD 5.2.1
  locator: pp. 84
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/character_species_special_traits
  parameters:
    species_trait_grants:
      all: true
      from: chosen_species
      resolve_trait_choices: true
  implementation: dnd2024:rules_character_species_special_traits
splat_contracts:
- id: dnd2024:rules_character_species_special_traits
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
  - 84
  engine_gaps: &id001
  - dnd2024:rules_character_species_special_traits
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 23abf227455b44c7105b325c59e3fed9bdf60942889906e197238005af1d5a4e
  parent_name: Character Species
  line_span:
    first: 7159
    count: 7
splat_clauses:
- key: source_rules
  text: '# Special Traits Each species gives a character special traits— unique characteristics based
    on the species’ physiology or magical nature. When you choose a species, your character gets all the
    special traits listed for it. Some traits involve making a choice from a handful of options.'
  sha256: cd152b845486eb6b923924add724d8fc4c7611d3f8c4d4b8f27695f5a98613d3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Special Traits

<!-- SRD page 84 -->

Each species gives a character special traits—\
unique characteristics based on the species’ physi-\
ology or magical nature. When you choose a species,\
your character gets all the special traits listed for it.\
Some traits involve making a choice from a handful\
of options.\
