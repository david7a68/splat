---
splat_schema: 1
splat_id: 36a3a2ae-343a-5897-8b0b-71147020d6aa
splat_kind: rule
splat_key: dnd2024:rules/weapons_sap
splat_name: Sap
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_sap
  parameters:
    trigger: weapon_hits_creature
    effect:
      next_attack_roll: disadvantage
      expires: start_of_attacker_next_turn
  implementation: dnd2024:rule_weapons_sap
splat_contracts:
- id: dnd2024:rule_weapons_sap
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - effects
  events:
  - attack.hit
  capabilities:
  - weapon_mastery
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rule_weapons_sap
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 0c2864b4be23bfac45083f352b90554eaf7b0938ab0cb2b0521d2f897dd6ea64
  parent_name: Weapons
  line_span:
    first: 7797
    count: 4
splat_clauses:
- key: source_rules
  text: '# Sap If you hit a creature with this weapon, that creature has Disadvantage on its next attack
    roll before the start of your next turn.'
  sha256: b40763cd8c9feb83b4c0ee858e89a0ff9ace623441a3396dfdb96da0c6072082
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sap

<!-- SRD page 90 -->

If you hit a creature with this weapon, that creature\
has Disadvantage on its next attack roll before the\
start of your next turn.\
