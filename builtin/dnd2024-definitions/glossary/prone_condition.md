---
splat_schema: 1
splat_id: 4a852418-1cfd-599d-8511-812b3997433c
splat_kind: rule
splat_key: dnd2024:glossary/prone_condition
splat_name: Prone [Condition]
splat_origin:
  key: dnd2024:reference/glossary/prone_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/prone_condition
  parameters:
    condition: prone
    movement_options:
    - crawl
    - right_self
    right_self:
      movement_cost:
        op: floor
        args:
        - op: divide
          args:
          - var: speed
          - 2
      requires_speed_greater_than: 0
      ends_condition: true
    own_attack_rolls: disadvantage
    incoming_attacks:
      attacker_distance_feet_max_5: advantage
      attacker_distance_feet_greater_5: disadvantage
  implementation: dnd2024:glossary_prone_condition
splat_contracts:
- id: dnd2024:glossary_prone_condition
  inputs:
    subject:
      type: participant
      required: true
    attacker:
      required: false
      type: participant
    movement_option:
      required: false
      type: choice
  outputs:
  - condition_effects
  - stand_cost
  events:
  - movement.pending
  - attack.pending
  capabilities:
  - conditions
  - movement
  - attacks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 9fb11b014d55b145fec6504693c05041df45e944f4c9cc4d200bcdd450c806a0
  source_path: builtin/srd-5.2.1/glossary/prone_condition.md
  pages:
  - 186
  engine_gaps: &id001
  - dnd2024:glossary_prone_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 659252eb55ce2ec6316be5646aa171fed27dad7c891e0378e40f59dfcfbb6d20
splat_clauses:
- key: source_rules
  text: '# Prone [Condition] While you have the Prone condition, you experience the following effects.
    Restricted Movement. Your only movement options are to crawl or to spend an amount of movement equal
    to half your Speed (round down) to right yourself and thereby end the condition. If your Speed is
    0, you can’t right yourself. Attacks Affected. You have Disadvantage on attack rolls. An attack roll
    against you has Advantage if the attacker is within 5 feet of you. Otherwise, that attack roll has
    Disadvantage.'
  sha256: db69862824f83fd473ca47fb564fbcd33cf9ed0208674d4134b1856b4dcaedb3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Prone [Condition]

<!-- SRD page 186 -->

While you have the Prone condition, you experience\
the following effects.\
Restricted Movement. Your only movement op-\
tions are to crawl or to spend an amount of move-\
ment equal to half your Speed (round down) to\
right yourself and thereby end the condition. If your\
Speed is 0, you can’t right yourself.\
Attacks Affected. You have Disadvantage on at-\
tack rolls. An attack roll against you has Advantage\
if the attacker is within 5 feet of you. Otherwise,\
that attack roll has Disadvantage.\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].
