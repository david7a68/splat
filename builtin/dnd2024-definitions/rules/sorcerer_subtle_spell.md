---
splat_schema: 1
splat_id: 97069f88-f383-51ec-9e25-eb4943a439fe
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_subtle_spell
splat_name: Subtle Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_subtle_spell
  parameters:
    cost:
      resource: sorcery_points
      amount: 1
    trigger: spell.cast_pending
    omit_components:
    - verbal
    - somatic
    - material_without_cost_or_consumption
    retain_material_if:
      any:
      - consumed
      - specified_cost
  implementation: dnd2024:rules_sorcerer_subtle_spell
splat_contracts:
- id: dnd2024:rules_sorcerer_subtle_spell
  inputs:
    caster:
      type: participant
      required: true
    spell:
      type: definition
      required: true
    targets:
      type: participant_list
      required: true
    cast_level:
      type: integer
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - spell_modifications
  events:
  - spell.cast_pending
  capabilities:
  - spellcasting
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 67
  engine_gaps: &id001
  - dnd2024:rules_sorcerer_subtle_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 5ba8e19e229efd826dfb6d6eb195962b6b8718d65762a2ff8b7ec4c2c42caf33
  parent_name: Sorcerer
  line_span:
    first: 5690
    count: 7
splat_clauses:
- key: source_rules
  text: '# Subtle Spell Cost: 1 Sorcery Point When you cast a spell, you can spend 1 Sorcery Point to
    cast it without any Verbal, Somatic, or Material components, except Material components that are consumed
    by the spell or that have a cost specified in the spell.'
  sha256: 8b087b21933b85444a70443d87d05603683aea57aa83173c21d0ef0781262e4b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Subtle Spell

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
When you cast a spell, you can spend 1 Sorcery\
Point to cast it without any Verbal, Somatic, or Ma-\
terial components, except Material components that\
are consumed by the spell or that have a cost speci-\
fied in the spell.\
