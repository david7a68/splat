---
splat_schema: 1
splat_id: a8bb6f61-52b7-55df-bae5-c4943468298a
splat_kind: rule
splat_key: dnd2024:glossary/creature_type
splat_name: Creature Type
splat_origin:
  key: dnd2024:reference/glossary/creature_type
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/creature_type
  parameters:
    types:
    - aberration
    - beast
    - celestial
    - construct
    - dragon
    - elemental
    - fey
    - fiend
    - giant
    - humanoid
    - monstrosity
    - ooze
    - plant
    - undead
    intrinsic_type_rules: false
    all_creatures_have_type: true
    type_specific_effects: effect_defined
  implementation: dnd2024:glossary_creature_type
splat_contracts:
- id: dnd2024:glossary_creature_type
  inputs:
    subject:
      type: participant
      required: true
    type:
      type: choice
      required: true
  outputs:
  - creature_type
  events: []
  capabilities:
  - statistics
  - targeting
splat_conversion:
  status: specified
  category: glossary
  source_sha256: c6f8fa71f4a1c11cbe0180f955e418d16f6e4a3e405aea45687f92d2e9e4d6a3
  source_path: builtin/srd-5.2.1/glossary/creature_type.md
  pages:
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_creature_type
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 6bf466a529326ea3f3563c325c6e3e74abb239bbefbe77f1454f0c7b64a4ddb4
splat_clauses:
- key: source_rules
  text: '# Creature Type Every creature, including every player character, has a tag in the rules that
    identifies the type of creature it is. Most player characters are of the Humanoid type. These are
    the game’s creature types: Aberration Elemental Monstrosity Beast Fey Ooze Celestial Fiend Plant Construct
    Giant Undead Dragon Humanoid The types don’t have rules themselves, but some rules in the game affect
    creatures of certain types in different ways.'
  sha256: e6ad85c45ca66aafeae6db05780dddcbfb839132a319dc48e0861ef8ebd5bc3b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Creature Type

<!-- SRD page 179 -->

Every creature, including every player character,\
has a tag in the rules that identifies the type of crea-\
ture it is. Most player characters are of the Human-\
oid type. These are the game’s creature types:\
Aberration Elemental Monstrosity\
Beast Fey Ooze\
Celestial Fiend Plant\
Construct Giant Undead\
Dragon Humanoid\
The types don’t have rules themselves, but some\
rules in the game affect creatures of certain types\
in different ways.\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].
