---
splat_schema: 1
splat_id: 73cb3725-5cb6-53b0-bce4-ffb1751a0b35
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_attack_rolls
splat_name: Attack Rolls
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_attack_rolls
  parameters:
    spell_attack_modifier:
      op: add
      args:
      - var: caster.spellcasting_ability_modifier
      - var: caster.proficiency_bonus
  implementation: dnd2024:rules_casting_spells_attack_rolls
splat_contracts:
- id: dnd2024:rules_casting_spells_attack_rolls
  inputs:
    caster:
      type: participant
      required: true
    spell:
      type: definition
      required: true
  outputs:
  - attack_modifier
  events: []
  capabilities:
  - spell_attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 106
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_attack_rolls
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 80963432c199abff5647d70b65dcfeaa915319fdb8de6ae22f93b4759bb10389
  parent_name: Casting Spells
  line_span:
    first: 9299
    count: 7
splat_clauses:
- key: source_rules
  text: '# Attack Rolls Some spells require the caster to make an attack roll to determine whether the
    spell hits a target. Here’s how to calculate the attack modifier for your spells: Spell attack modifier
    = your spellcasting ability modifier + your Proficiency Bonus'
  sha256: 79acb1a8d69903ae684f57884125dc56dfb23362d549230b06df4f5ca221e40a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Attack Rolls

<!-- SRD page 106 -->

Some spells require the caster to make an attack\
roll to determine whether the spell hits a target.\
Here’s how to calculate the attack modifier for your\
spells:\
Spell attack modifier = your spellcasting ability\
modifier + your Proficiency Bonus\
