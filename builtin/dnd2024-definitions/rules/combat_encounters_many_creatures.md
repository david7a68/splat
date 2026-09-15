---
splat_schema: 1
splat_id: 0b6c115a-41ea-526f-85af-8df2e8181de1
splat_kind: rule
splat_key: dnd2024:rules/combat_encounters_many_creatures
splat_name: Many Creatures
splat_origin:
  key: dnd2024:reference/pages/203
splat_sources:
- title: SRD 5.2.1
  locator: pp. 203
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_encounters_many_creatures
  parameters:
    guideline: true
    if_creatures_per_character_greater_than: 2
    recommendation: include_quickly_defeated_fragile_creatures
    priority_character_levels:
    - 1
    - 2
    mandatory_combat_modifier: false
  implementation: dnd2024:rules_combat_encounters_many_creatures
splat_contracts:
- id: dnd2024:rules_combat_encounters_many_creatures
  inputs:
    party:
      type: participant_list
      required: true
    enemies:
      type: participant_list
      required: true
  outputs:
  - encounter_composition_guidance
  events: []
  capabilities:
  - encounter_building
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 203
  engine_gaps: &id001
  - dnd2024:rules_combat_encounters_many_creatures
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 2045bd64edf6c9c8680bb272294c01bd59bd8c7186d9c50fc196b8405d03c20e
  parent_name: Combat Encounters
  line_span:
    first: 10348
    count: 8
splat_clauses:
- key: source_rules
  text: '# Many Creatures The more creatures in an encounter, the higher the risk that a lucky streak
    on their part could deal more damage to the characters than you expect. If your encounter includes
    more than two creatures per character, include fragile creatures that can be defeated quickly. This
    guideline is especially important for characters of level 1 or 2.'
  sha256: 1699cff380ab4c53c46f0c4e5c61bd33d2792afb7ad8ae614544e8a5afb1bed3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Many Creatures

<!-- SRD page 203 -->

The more creatures in an encounter, the higher\
the risk that a lucky streak on their part could deal\
more damage to the characters than you expect. If\
your encounter includes more than two creatures\
per character, include fragile creatures that can be\
defeated quickly. This guideline is especially im-\
portant for characters of level 1 or 2.\
