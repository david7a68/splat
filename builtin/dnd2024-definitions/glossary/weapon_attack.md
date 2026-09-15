---
splat_schema: 1
splat_id: 9e8c157d-4984-5099-beda-deccd883499f
splat_kind: rule
splat_key: dnd2024:glossary/weapon_attack
splat_name: Weapon Attack
splat_origin:
  key: dnd2024:reference/glossary/weapon_attack
splat_sources:
- title: SRD 5.2.1
  locator: pp. 191
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/weapon_attack
  parameters:
    test_kind: attack_roll
    uses_weapon: true
  implementation: dnd2024:glossary_weapon_attack
splat_contracts:
- id: dnd2024:glossary_weapon_attack
  inputs:
    attack:
      type: resolution
      required: true
  outputs:
  - is_weapon_attack
  events: []
  capabilities:
  - attacks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: a086b679f70cde236946a8447c3ecbbbd198f2350d28398677b5411818a7945f
  source_path: builtin/srd-5.2.1/glossary/weapon_attack.md
  pages:
  - 191
  engine_gaps: &id001
  - dnd2024:glossary_weapon_attack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 4d99883fa9b38279310c1b5835066a8a47ea25b79b598544f3e8f40ce4f53517
splat_clauses:
- key: source_rules
  text: '# Weapon Attack A weapon attack is an attack roll made with a weapon. See also “Weapon.”'
  sha256: 0a1132778fd4c84f3be0d26b1a77c41ee730f289f66971a9e37bc482fb91e05e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weapon Attack

<!-- SRD page 191 -->

A weapon attack is an attack roll made with a\
weapon. See also “Weapon.”\

Original layout: [[../../srd-5.2.1/pages/191|page 191]].
