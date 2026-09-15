---
splat_schema: 1
splat_id: 6205a774-967b-5a3c-84ea-b1e5c1407d6b
splat_kind: rule
splat_key: dnd2024:rules/combat_mounting_and_dismounting
splat_name: Mounting and Dismounting
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_mounting_and_dismounting
  parameters:
    timing: during_movement
    mount_distance_max_ft: 5
    choices:
    - mount
    - dismount
    movement_cost:
      op: floor
      args:
      - op: divide
        args:
        - var: subject.speed
        - 2
  implementation: dnd2024:rules_combat_mounting_and_dismounting
splat_contracts:
- id: dnd2024:rules_combat_mounting_and_dismounting
  inputs:
    subject:
      type: participant
      required: true
    mount:
      type: participant
      required: true
    mode:
      type: choice
      required: true
  outputs:
  - mounted_state
  - movement_remaining
  events: []
  capabilities:
  - mounts
  - movement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 15
  engine_gaps: &id001
  - dnd2024:rules_combat_mounting_and_dismounting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 1010ddcf2a47a37d8b50a65b9f47c022588c29f8a29659b251411ac2e84ef795
  parent_name: Combat
  line_span:
    first: 1063
    count: 6
splat_clauses:
- key: source_rules
  text: '# Mounting and Dismounting During your move, you can mount a creature that is within 5 feet of
    you or dismount. Doing so costs an amount of movement equal to half your Speed (round down). For example,
    if your Speed is 30 feet, you spend 15 feet of movement to mount a horse.'
  sha256: 7d45f68879134d0fcdd0ffd4e4ab743ce33422a23378469d535b87555464c15e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mounting and Dismounting

<!-- SRD page 15 -->

During your move, you can mount a creature that\
is within 5 feet of you or dismount. Doing so costs\
an amount of movement equal to half your Speed\
(round down). For example, if your Speed is 30 feet,\
you spend 15 feet of movement to mount a horse.\
