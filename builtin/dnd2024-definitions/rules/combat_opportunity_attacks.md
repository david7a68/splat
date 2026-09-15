---
splat_schema: 1
splat_id: 2f3e8ffa-77e7-5fbc-8a94-bf06f8ee3523
splat_kind: rule
splat_key: dnd2024:rules/combat_opportunity_attacks
splat_name: Opportunity Attacks
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_opportunity_attacks
  parameters:
    trigger:
      event: creature.leaves_reach
      target_visible: true
    cost: reaction
    attack_count: 1
    attack_options:
    - melee_weapon
    - unarmed_strike
    attack_timing: immediately_before_leaves_reach
    prevented_by:
    - disengage
    - teleportation
    - movement_without_target_movement_action_bonus_action_or_reaction
  implementation: dnd2024:rule_combat_opportunity_attacks
splat_contracts:
- id: dnd2024:rule_combat_opportunity_attacks
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - attack_modifiers
  events:
  - creature.leaves_reach
  capabilities:
  - attacks
  - reactions
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 15
  engine_gaps: &id001
  - dnd2024:rule_combat_opportunity_attacks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 57d719e911a7d4b3d6f128f20409d965a3c94a557cbe185647b67423ab1f1bb5
  parent_name: Combat
  line_span:
    first: 1041
    count: 18
splat_clauses:
- key: source_rules
  text: '# Opportunity Attacks Combatants watch for enemies to drop their guard. If you move heedlessly
    past your foes, you put yourself in danger by provoking an Opportunity Attack. Avoiding Opportunity
    Attacks. You can avoid provoking an Opportunity Attack by taking the Disengage action. You also don’t
    provoke an Opportunity Attack when you Teleport or when you are moved without using your movement,
    action, Bonus Action, or Reaction. For example, you don’t provoke an Opportunity Attack if an explosion
    hurls you out of a foe’s reach or if you fall past an enemy. Making an Opportunity Attack. You can
    make an Opportunity Attack when a creature that you can see leaves your reach. To make the attack,
    take a Reaction to make one melee attack with a weapon or an Unarmed Strike against that creature.
    The attack occurs right before it leaves your reach.'
  sha256: 336aa6952244840f2a1ea479321510ee3485c6d63438aba881d8ebd98fa3031d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Opportunity Attacks

<!-- SRD page 15 -->

Combatants watch for enemies to drop their guard.\
If you move heedlessly past your foes, you put your-\
self in danger by provoking an Opportunity Attack.\
Avoiding Opportunity Attacks. You can avoid\
provoking an Opportunity Attack by taking the\
Disengage action. You also don’t provoke an Oppor-\
tunity Attack when you Teleport or when you are\
moved without using your movement, action, Bonus\
Action, or Reaction. For example, you don’t provoke\
an Opportunity Attack if an explosion hurls you out\
of a foe’s reach or if you fall past an enemy.\
Making an Opportunity Attack. You can make an\
Opportunity Attack when a creature that you can\
see leaves your reach. To make the attack, take a Re-\
action to make one melee attack with a weapon or\
an Unarmed Strike against that creature. The attack\
occurs right before it leaves your reach.\
