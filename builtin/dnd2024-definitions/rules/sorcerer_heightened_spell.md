---
splat_schema: 1
splat_id: 7d996fa1-2b9f-5ff0-8820-348733229a3c
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_heightened_spell
splat_name: Heightened Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_heightened_spell
  parameters:
    cost:
      resource: sorcery_points
      amount: 2
    trigger: spell.cast_pending
    spell_requires_saving_throw: true
    target_choice:
      count: 1
      from: spell_targets
    save_disadvantage:
      scope: all_saves_against_this_casting
  implementation: dnd2024:rules_sorcerer_heightened_spell
splat_contracts:
- id: dnd2024:rules_sorcerer_heightened_spell
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
  - dnd2024:rules_sorcerer_heightened_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: c0850e214577c659e98eac7774219225d08861382d5adc82350f92e9cde6ce62
  parent_name: Sorcerer
  line_span:
    first: 5667
    count: 6
splat_clauses:
- key: source_rules
  text: '# Heightened Spell Cost: 2 Sorcery Points When you cast a spell that forces a creature to make
    a saving throw, you can spend 2 Sorcery Points to give one target of the spell Disadvantage on saves
    against the spell.'
  sha256: 7213bb1b3ce621f2cc91a8cd3be9f8eaa0704ebbadcbe0333f561b99c8eb2f34
  status: contract_specified
splat_requires_mechanics: *id001
---

# Heightened Spell

<!-- SRD page 67 -->

Cost: 2 Sorcery Points\
When you cast a spell that forces a creature to make\
a saving throw, you can spend 2 Sorcery Points to\
give one target of the spell Disadvantage on saves\
against the spell.\
