---
splat_schema: 1
splat_id: e511a026-8cb1-521b-b740-dc415548ebf3
splat_kind: rule
splat_key: dnd2024:glossary/dodge_action
splat_name: Dodge [Action]
splat_origin:
  key: dnd2024:reference/glossary/dodge_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/dodge_action
  parameters:
    cost: action
    ends: own_next_turn_start
    incoming_attacks:
      disadvantage: true
      requires_subject_sees_attacker: true
    dexterity_saves: advantage
    lose_benefits_when_any:
    - incapacitated
    - speed_zero
  implementation: dnd2024:glossary_dodge_action
splat_contracts:
- id: dnd2024:glossary_dodge_action
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - dodge_effect
  events:
  - action.requested
  - attack.pending
  - saving_throw.pending
  - condition.changed
  - speed.changed
  - turn.started
  capabilities:
  - action_economy
  - effects
  - checks
  - attacks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 59afaf9f0f1d0fee6330b2571ae40e5a941a6d945038048b2e3b13d7bb743276
  source_path: builtin/srd-5.2.1/glossary/dodge_action.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_dodge_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 7995c06e1cd3dcf8e8a61af0b1a1258a80186534ea5617b4b178f528375c4de7
splat_clauses:
- key: source_rules
  text: '# Dodge [Action] If you take the Dodge action, you gain the following benefits: until the start
    of your next turn, any attack roll made against you has Disadvantage if you can see the attacker,
    and you make Dexterity saving throws with Advantage. You lose these benefits if you have the Incapacitated
    condition or if your Speed is 0.'
  sha256: 03823c80061ecf4c50caff1c9f7f96b0103c266b5f19497f0d0304a81d272319
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dodge [Action]

<!-- SRD page 181 -->

If you take the Dodge action, you gain the following\
benefits: until the start of your next turn, any attack\
roll made against you has Disadvantage if you can\
see the attacker, and you make Dexterity saving\
throws with Advantage.\
You lose these benefits if you have the Incapaci-\
tated condition or if your Speed is 0.\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].
