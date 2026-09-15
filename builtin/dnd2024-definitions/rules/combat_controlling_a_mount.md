---
splat_schema: 1
splat_id: 7b6124b3-cd72-59c6-a5f0-323f4dd69247
splat_kind: rule
splat_key: dnd2024:rules/combat_controlling_a_mount
splat_name: Controlling a Mount
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_controlling_a_mount
  parameters:
    controlled:
      requires: trained_to_accept_rider
      initiative: rider_initiative
      movement_turn: rider_turn
      controller: rider
      actions:
      - dash
      - disengage
      - dodge
      may_act_on_mounting_turn: true
    independent:
      initiative: unchanged
      controller: mount
  implementation: dnd2024:rule_combat_controlling_a_mount
splat_contracts:
- id: dnd2024:rule_combat_controlling_a_mount
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - resolution
  events: []
  capabilities:
  - mounts
  - turns
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  engine_gaps: &id001
  - dnd2024:rule_combat_controlling_a_mount
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: b40ec1ba2a469fe8a898fdd2e95fbc6183c61adf229474ece183480fb7552069
  parent_name: Combat
  line_span:
    first: 1069
    count: 13
splat_clauses:
- key: source_rules
  text: '# Controlling a Mount You can control a mount only if it has been trained to accept a rider.
    Domesticated horses, mules, and similar creatures have such training. The Initiative of a controlled
    mount changes to match yours when you mount it. It moves on your turn as you direct it, and it has
    only three action options during that turn: Dash, Disengage, and Dodge. A controlled mount can move
    and act even on the turn that you mount it. In contrast, an independent mount—one that lets you ride
    but ignores your control—retains its place in the Initiative order and moves and acts as it likes.'
  sha256: 8437a91834ddfcaee3c3bb8c7f8dbcfa7091232f9afd83d7866c93a6ebb9f68c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Controlling a Mount

<!-- SRD page 16 -->

You can control a mount only if it has been trained\
to accept a rider. Domesticated horses, mules, and\
similar creatures have such training.\
The Initiative of a controlled mount changes to\
match yours when you mount it. It moves on your\
turn as you direct it, and it has only three action op-\
tions during that turn: Dash, Disengage, and Dodge.\
A controlled mount can move and act even on the\
turn that you mount it.\
In contrast, an independent mount—one that lets\
you ride but ignores your control—retains its place\
in the Initiative order and moves and acts as it likes.\
