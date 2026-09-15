---
splat_schema: 1
splat_id: 9c568993-cdec-5914-91ff-fc45f1dca1e9
splat_kind: rule
splat_key: dnd2024:rules/combat_ranged_attacks_in_close_combat
splat_name: Ranged Attacks in Close Combat
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_ranged_attacks_in_close_combat
  parameters:
    attack_kind: ranged
    disadvantage_when:
      enemy_distance_feet_maximum: 5
      enemy_can_see_attacker: true
      enemy_incapacitated: false
    attack_sources:
    - weapon
    - spell
    - other
  implementation: dnd2024:rule_combat_ranged_attacks_in_close_combat
splat_contracts:
- id: dnd2024:rule_combat_ranged_attacks_in_close_combat
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - attack_modifiers
  events: []
  capabilities:
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 15
  engine_gaps: &id001
  - dnd2024:rule_combat_ranged_attacks_in_close_combat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 2c75d589e46c5ccf2ab0ae4993ea0874c988c569a193bec8250e178f7f7e61f3
  parent_name: Combat
  line_span:
    first: 1023
    count: 7
splat_clauses:
- key: source_rules
  text: '# Ranged Attacks in Close Combat Aiming a ranged attack is more difficult when a foe is next
    to you. When you make a ranged attack roll with a weapon, a spell, or some other means, you have Disadvantage
    on the roll if you are within 5 feet of an enemy who can see you and doesn’t have the Incapacitated
    condition (see “Rules Glossary”).'
  sha256: 851374b1dc3bcfad2f9f9e402d191026caf8fb5473f6a63f515e96fdabcc8fd2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ranged Attacks in Close Combat

<!-- SRD page 15 -->

Aiming a ranged attack is more difficult when a foe\
is next to you. When you make a ranged attack roll\
with a weapon, a spell, or some other means, you\
have Disadvantage on the roll if you are within 5\
feet of an enemy who can see you and doesn’t have\
the Incapacitated condition (see “Rules Glossary”).\
