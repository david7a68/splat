---
splat_schema: 1
splat_id: ef52665e-5eb6-575b-a612-dcd08c65f661
splat_kind: rule
splat_key: dnd2024:rules/character_species
splat_name: Character Species
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/character_species
  parameters:
    species_selection:
      count: 1
      grants: species_game_traits
    normal_lifespan_years_default: 80
    species_lifespan_overrides_default: true
    character_age:
      max: species_normal_lifespan
    physical_maturity_age_basis: shared_across_species
  implementation: dnd2024:rules_character_species
splat_contracts:
- id: dnd2024:rules_character_species
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - species_traits
  - age_constraints
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 83
  engine_gaps: &id001
  - dnd2024:rules_character_species
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: d81231217b28b9a9471f0e267e385460a462862ba6c07d562c6116201a2cd6ba
  parent_name: Character Species
  line_span:
    first: 7120
    count: 22
splat_clauses:
- key: source_rules
  text: '# Character Species When you choose your character’s species, you determine whether your character
    is a human or a member of a fantastical species, such as dragonborn or gnome. The peoples of the multiverse
    hail from different worlds and include many kinds of sapient life forms. A player character’s species
    is the set of game traits that an adventurer gains from being one of those life forms. Some species
    can trace their origin to a single world, plane of existence, or god, while other species first appeared
    in multiple realms at once. Whatever a species’ genesis, its members have spread across the multiverse
    and contribute to many different cultures. Members of most species live for about 80 years, with exceptions
    noted in the text about the species in “Character Origins.” Regardless of life span, members of all
    species reach physical maturity at about the same age. Your character can be any age that isn’t beyond
    their species’ normal life span.'
  sha256: c72c6fa80433de3fd9f3d61a51ddb43b17cbf47dc5137b9e3467c9fe659f2599
  status: contract_specified
splat_requires_mechanics: *id001
---

# Character Species

<!-- SRD page 83 -->

When you choose your character’s species, you\
determine whether your character is a human or a\
member of a fantastical species, such as dragonborn\
or gnome.\
The peoples of the multiverse hail from different\
worlds and include many kinds of sapient life forms.\
A player character’s species is the set of game traits\
that an adventurer gains from being one of those\
life forms.\
Some species can trace their origin to a single\
world, plane of existence, or god, while other spe-\
cies first appeared in multiple realms at once.\
Whatever a species’ genesis, its members have\
spread across the multiverse and contribute to\
many different cultures.\
Members of most species live for about 80 years,\
with exceptions noted in the text about the spe-\
cies in “Character Origins.” Regardless of life span,\
members of all species reach physical maturity at\
about the same age. Your character can be any age\
that isn’t beyond their species’ normal life span.\
