---
splat_schema: 1
splat_id: 80d03a57-0912-5ec7-b90b-e132a8876e05
splat_kind: rule
splat_key: dnd2024:glossary/opportunity_attacks
splat_name: Opportunity Attacks
splat_origin:
  key: dnd2024:reference/glossary/opportunity_attacks
splat_sources:
- title: SRD 5.2.1
  locator: pp. 185
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/opportunity_attacks
  parameters:
    trigger:
      creature_leaves_reach: true
      subject_can_see_creature: true
      movement_uses_any:
      - action
      - bonus_action
      - reaction
      - speed
    cost: reaction
    attack:
      count: 1
      type: melee
      options:
      - weapon
      - unarmed_strike
      target: provoking_creature
      timing: immediately_before_leaving_reach
  implementation: dnd2024:glossary_opportunity_attacks
splat_contracts:
- id: dnd2024:glossary_opportunity_attacks
  inputs:
    subject:
      type: participant
      required: true
    provoker:
      type: participant
      required: true
    movement:
      type: event
      required: true
    attack_option:
      type: choice
      required: true
  outputs:
  - reaction_attack
  events:
  - reach.exit_pending
  capabilities:
  - attacks
  - action_economy
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 19a458fa80cce79bf37fc3f26cf5379f6835dc2f8019433517b2842d43393b5f
  source_path: builtin/srd-5.2.1/glossary/opportunity_attacks.md
  pages:
  - 185
  engine_gaps: &id001
  - dnd2024:glossary_opportunity_attacks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 89a9f6c36b8a9bac19e502ab73207d683834220896f7184cd5dd5f909954100f
splat_clauses:
- key: source_rules
  text: '# Opportunity Attacks You can make an Opportunity Attack when a creature that you can see leaves
    your reach using its action, its Bonus Action, its Reaction, or one of its speeds. To make the Opportunity
    Attack, take a Reaction to make one melee attack with a weapon or an Unarmed Strike against the provoking
    creature. The attack occurs right before the creature leaves your reach. See also “Playing the Game”
    (“Combat”).'
  sha256: 3c2f89a7cc10cf7ff5f8ed2ff1f9f4d99279bb3b87baac0bf95778ac3a210eb5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Opportunity Attacks

<!-- SRD page 185 -->

You can make an Opportunity Attack when a crea-\
ture that you can see leaves your reach using its\
action, its Bonus Action, its Reaction, or one of its\
speeds. To make the Opportunity Attack, take a Re-\
action to make one melee attack with a weapon or\
an Unarmed Strike against the provoking creature.\
The attack occurs right before the creature leaves\
your reach. See also “Playing the Game” (“Combat”).\

Original layout: [[../../srd-5.2.1/pages/185|page 185]].
