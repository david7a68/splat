---
splat_schema: 1
splat_id: 9a897644-e7f1-55d2-b419-de6018343271
splat_kind: rule
splat_key: dnd2024:glossary/spell_attack
splat_name: Spell Attack
splat_origin:
  key: dnd2024:reference/glossary/spell_attack
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/spell_attack
  parameters:
    test_kind: attack_roll
    source_any:
    - spell
    - other_magical_effect
  implementation: dnd2024:glossary_spell_attack
splat_contracts:
- id: dnd2024:glossary_spell_attack
  inputs:
    attack:
      type: resolution
      required: true
  outputs:
  - is_spell_attack
  events: []
  capabilities:
  - attacks
  - spellcasting
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 16e6ff1a01734e81f65a85001b1a84edd6b9eb7580283514cce18bbef3a2d41c
  source_path: builtin/srd-5.2.1/glossary/spell_attack.md
  pages:
  - 188
  engine_gaps: &id001
  - dnd2024:glossary_spell_attack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 1ef9c87cd04f4b3752144689213a0e4176770867f75c68f251c2550d00cf3697
splat_clauses:
- key: source_rules
  text: '# Spell Attack A spell attack is an attack roll made as part of a spell or another magical effect.
    See also “Spells” (“Casting Spells”).'
  sha256: da046f3c0137990166d19602ce4758b793db69e666a714420e87f25f754a68e6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spell Attack

<!-- SRD page 188 -->

A spell attack is an attack roll made as part of a spell\
or another magical effect. See also “Spells” (“Casting\
Spells”).\

Original layout: [[../../srd-5.2.1/pages/188|page 188]].
