---
splat_schema: 1
splat_id: aa5e05a3-e3ac-5ecd-8433-7f5cc3f02807
splat_kind: rule
splat_key: dnd2024:rules/weapons_topple
splat_name: Topple
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_topple
  parameters:
    trigger: weapon_hit
    optional: true
    save:
      ability: constitution
      dc:
        op: add
        args:
        - 8
        - var: attack.ability_modifier
        - var: subject.proficiency_bonus
      failure:
        condition: prone
  implementation: dnd2024:rules_weapons_topple
splat_contracts:
- id: dnd2024:rules_weapons_topple
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - attack.hit
  capabilities:
  - weapon_mastery
  - saving_throws
  - conditions
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rules_weapons_topple
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 53a30b260a72047892f54a8532b88e06b440624c632d559dbb66c29b3c675ce3
  parent_name: Weapons
  line_span:
    first: 7807
    count: 6
splat_clauses:
- key: source_rules
  text: '# Topple If you hit a creature with this weapon, you can force the creature to make a Constitution
    saving throw (DC 8 plus the ability modifier used to make the attack roll and your Proficiency Bonus).
    On a failed save, the creature has the Prone condition.'
  sha256: 8e5a093038c0425e036bb9902ce898e44491a7b3b71b333705112fdd56353e26
  status: contract_specified
splat_requires_mechanics: *id001
---

# Topple

<!-- SRD page 90 -->

If you hit a creature with this weapon, you can force\
the creature to make a Constitution saving throw\
(DC 8 plus the ability modifier used to make the\
attack roll and your Proficiency Bonus). On a failed\
save, the creature has the Prone condition.\
