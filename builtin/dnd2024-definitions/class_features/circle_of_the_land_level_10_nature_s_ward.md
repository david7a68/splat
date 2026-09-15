---
splat_schema: 1
splat_id: 2a7aed1d-4207-5101-aac3-ec846256c8cc
splat_kind: rule
splat_key: dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward
splat_name: 'Level 10: Nature’s Ward'
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward
  parameters:
    condition_immunities:
    - poisoned
    damage_resistance_by_current_circle_land:
      arid: fire
      polar: cold
      temperate: lightning
      tropical: poison
  implementation: dnd2024:class_features_circle_of_the_land_level_10_nature_s_ward
splat_contracts:
- id: dnd2024:class_features_circle_of_the_land_level_10_nature_s_ward
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.feature_gained
  capabilities:
  - immunities
  - resistances
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 46
  engine_gaps: &id001
  - dnd2024:class_features_circle_of_the_land_level_10_nature_s_ward
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 145c80b71f6cef6f4d62f76d0d2880e3fb869af1229521cfe101cce10ce27214
  parent_name: Circle of the Land
  line_span:
    first: 3854
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 10: Nature’s Ward You are immune to the Poisoned condition, and you have Resistance to
    a damage type associated with your current land choice in the Circle Spells feature, as shown in the
    Nature’s Ward table. Nature’s Ward Land Type Resistance Land Type Resistance Arid Fire Temperate Lightning
    Polar Cold Tropical Poison'
  sha256: f3d85d45f28285f447289a65056df40e349c713b7c90a96ca9228b1a10e09fc2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Nature’s Ward

<!-- SRD page 46 -->

You are immune to the Poisoned condition, and you\
have Resistance to a damage type associated with\
your current land choice in the Circle Spells feature,\
as shown in the Nature’s Ward table.\
Nature’s Ward\
Land Type Resistance Land Type Resistance\
Arid Fire Temperate Lightning\
Polar Cold Tropical Poison\
