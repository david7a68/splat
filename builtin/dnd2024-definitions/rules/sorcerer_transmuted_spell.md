---
splat_schema: 1
splat_id: 083c3735-c3bb-58b1-af7b-d6c1f0018f31
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_transmuted_spell
splat_name: Transmuted Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_transmuted_spell
  parameters:
    cost:
      sorcery_points: 1
    damage_type_choice:
    - acid
    - cold
    - fire
    - lightning
    - poison
    - thunder
    requires_original_type_in_choice: true
    new_type_must_differ: true
    scope: current_spell_cast
  implementation: dnd2024:rules_sorcerer_transmuted_spell
splat_contracts:
- id: dnd2024:rules_sorcerer_transmuted_spell
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - spell.cast_pending
  capabilities:
  - metamagic
  - damage
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 67
  engine_gaps: &id001
  - dnd2024:rules_sorcerer_transmuted_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 149680373a3251ca8034e800eb2281079abafc5d93296f0efed693c40276f32b
  parent_name: Sorcerer
  line_span:
    first: 5697
    count: 7
splat_clauses:
- key: source_rules
  text: '# Transmuted Spell Cost: 1 Sorcery Point When you cast a spell that deals a type of damage from
    the following list, you can spend 1 Sorcery Point to change that damage type to one of the other listed
    types: Acid, Cold, Fire, Lightning, Poison, Thunder.'
  sha256: 7d181f20eb5775dd55bc5bf1f80a6d4d6106c41e8b98b41bb3003039b87d7ed9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Transmuted Spell

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
When you cast a spell that deals a type of damage\
from the following list, you can spend 1 Sorcery\
Point to change that damage type to one of the\
other listed types: Acid, Cold, Fire, Lightning, Poi-\
son, Thunder.\
