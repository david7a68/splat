---
splat_schema: 1
splat_id: 72467b86-44e5-57b5-9aba-c403dfeab9b7
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_distant_spell
splat_name: Distant Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_distant_spell
  parameters:
    cost:
      resource: sorcery_points
      amount: 1
    trigger: spell.cast_pending
    range_branches:
    - range_feet_minimum: 5
      range_multiplier: 2
    - range_kind: touch
      replacement_range_feet: 30
  implementation: dnd2024:rules_sorcerer_distant_spell
splat_contracts:
- id: dnd2024:rules_sorcerer_distant_spell
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
  - dnd2024:rules_sorcerer_distant_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 513afb6c3e1295081010dcd624329a7700296a12e27bd1245e7a67a57a5beefa
  parent_name: Sorcerer
  line_span:
    first: 5643
    count: 7
splat_clauses:
- key: source_rules
  text: '# Distant Spell Cost: 1 Sorcery Point When you cast a spell that has a range of at least 5 feet,
    you can spend 1 Sorcery Point to double the spell’s range. Or when you cast a spell that has a range
    of Touch, you can spend 1 Sorcery Point to make the spell’s range 30 feet.'
  sha256: 6f01a655459bbdaebd5869935f0bbd77c2e65e5c21da2f4a9156a1022d668062
  status: contract_specified
splat_requires_mechanics: *id001
---

# Distant Spell

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
When you cast a spell that has a range of at least 5\
feet, you can spend 1 Sorcery Point to double the\
spell’s range. Or when you cast a spell that has a\
range of Touch, you can spend 1 Sorcery Point to\
make the spell’s range 30 feet.\
