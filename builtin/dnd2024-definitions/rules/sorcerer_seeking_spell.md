---
splat_schema: 1
splat_id: 0b70dc3a-8e2a-534b-a77a-522d1e5e7fe9
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_seeking_spell
splat_name: Seeking Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_seeking_spell
  parameters:
    trigger: spell_attack_miss
    cost_sorcery_points: 1
    reroll_d20: 1
    must_use_new_roll: true
    compatible_with_other_metamagic: true
  implementation: dnd2024:rule_sorcerer_seeking_spell
splat_contracts:
- id: dnd2024:rule_sorcerer_seeking_spell
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_modifications
  events:
  - attack.missed
  capabilities:
  - metamagic
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 67
  engine_gaps: &id001
  - dnd2024:rule_sorcerer_seeking_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 7d68e77ca0eb587131e3da3692b908b270e610c07b872eeeb1a759dbb6e532f4
  parent_name: Sorcerer
  line_span:
    first: 5682
    count: 8
splat_clauses:
- key: source_rules
  text: '# Seeking Spell Cost: 1 Sorcery Point If you make an attack roll for a spell and miss, you can
    spend 1 Sorcery Point to reroll the d20, and you must use the new roll. You can use Seeking Spell
    even if you’ve already used a different Metamagic option during the casting of the spell.'
  sha256: eb2ff88f0cc865d68633f7e28b26745b61eb58f6d0e0358dc4c2d39c9e4d3d78
  status: contract_specified
splat_requires_mechanics: *id001
---

# Seeking Spell

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
If you make an attack roll for a spell and miss, you\
can spend 1 Sorcery Point to reroll the d20, and you\
must use the new roll.\
You can use Seeking Spell even if you’ve already\
used a different Metamagic option during the cast-\
ing of the spell.\
