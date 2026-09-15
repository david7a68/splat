---
splat_schema: 1
splat_id: 11ffd42c-6b9b-59d6-8c80-dc1af3c23326
splat_kind: rule
splat_key: dnd2024:rules/warlock_pact_of_the_tome
splat_name: Pact of the Tome
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_pact_of_the_tome
  parameters:
    conjure:
      timing:
      - short_rest_finished
      - long_rest_finished
      location: hand
      appearance: owner_choice
      exclusive_magic_access: owner
    book_disappears:
    - replacement_book_conjured
    - owner_dies
    on_conjure_choices:
    - count: 3
      spell_level: 0
      source: any_class_spell_list
      exclude_already_prepared: true
    - count: 2
      spell_level: 1
      ritual: true
      source: any_class_spell_list
      exclude_already_prepared: true
    while_book_on_person:
      chosen_spells_prepared: true
      chosen_spells_class: warlock
    spellcasting_focus: true
  implementation: dnd2024:rules_warlock_pact_of_the_tome
splat_contracts:
- id: dnd2024:rules_warlock_pact_of_the_tome
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - book
  - prepared_spells
  - spellcasting_focus
  events:
  - rest.short_finished
  - rest.long_finished
  - participant.died
  capabilities:
  - character_options
  - inventory
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 74
  engine_gaps: &id001
  - dnd2024:rules_warlock_pact_of_the_tome
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 5aad5c3459591d3a06037bf8718057d080c01f1c2ad061619c1b10fa0eb2c559
  parent_name: Warlock
  line_span:
    first: 6326
    count: 17
splat_clauses:
- key: source_rules
  text: '# Pact of the Tome Stitching together strands of shadow, you conjure forth a book in your hand
    at the end of a Short or Long Rest. This Book of Shadows (you determine its appearance) contains eldritch
    magic that only you can access, granting you the benefits below. The book disappears if you conjure
    another book with this feature or if you die. Cantrips and Rituals. When the book appears, choose
    three cantrips, and choose two level 1 spells that have the Ritual tag. The spells can be from any
    class’s spell list, and they must be spells you don’t already have prepared. While the book is on
    your person, you have the chosen spells prepared, and they function as Warlock spells for you. Spellcasting
    Focus. You can use the book as a Spellcasting Focus.'
  sha256: c81324b69ce066112e839e91441581281cbebbae78f524fb186ae390a1863772
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pact of the Tome

<!-- SRD page 74 -->

Stitching together strands of shadow, you conjure\
forth a book in your hand at the end of a Short or\
Long Rest. This Book of Shadows (you determine its\
appearance) contains eldritch magic that only you\
can access, granting you the benefits below. The\
book disappears if you conjure another book with\
this feature or if you die.\
Cantrips and Rituals. When the book appears,\
choose three cantrips, and choose two level 1 spells\
that have the Ritual tag. The spells can be from any\
class’s spell list, and they must be spells you don’t\
already have prepared. While the book is on your\
person, you have the chosen spells prepared, and\
they function as Warlock spells for you.\
Spellcasting Focus. You can use the book as a\
Spellcasting Focus.\
