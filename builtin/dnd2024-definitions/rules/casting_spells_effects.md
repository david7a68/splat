---
splat_schema: 1
splat_id: 161f29c9-5247-520f-a254-db941a2d13b8
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_effects
splat_name: Effects
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_effects
  parameters:
    effect_definition_authoritative: true
    mundane_physical_laws_constrain_effects: false
    unlisted_outcomes:
      authority: gm
      input_type: resolution
  implementation: dnd2024:rules_casting_spells_effects
splat_contracts:
- id: dnd2024:rules_casting_spells_effects
  inputs:
    spell:
      type: definition
      required: true
    unlisted_outcome:
      required: false
      type: resolution
  outputs:
  - spell_effects
  events: []
  capabilities:
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 106
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_effects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 8e6b1a618f9d42344c9800aeab0e7c0e5d83af52cacf4c0c09d6688a9f86b560
  parent_name: Casting Spells
  line_span:
    first: 9248
    count: 8
splat_clauses:
- key: source_rules
  text: '# Effects The effects of a spell are detailed after its duration entry. Those details present
    exactly what the spell does, which ignores mundane physical laws; any outcomes beyond those effects
    are under the GM’s purview. Whatever the effects, they typically deal with targets, saving throws,
    attack rolls, or all three, each of which is detailed below.'
  sha256: c201ade083e4eca355e18c82a374e5aad3348e0943123c7a6b53a92499f7ca25
  status: contract_specified
splat_requires_mechanics: *id001
---

# Effects

<!-- SRD page 106 -->

The effects of a spell are detailed after its dura-\
tion entry. Those details present exactly what the\
spell does, which ignores mundane physical laws;\
any outcomes beyond those effects are under the\
GM’s purview. Whatever the effects, they typically\
deal with targets, saving throws, attack rolls, or all\
three, each of which is detailed below.\
