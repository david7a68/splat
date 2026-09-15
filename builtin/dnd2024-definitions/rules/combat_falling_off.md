---
splat_schema: 1
splat_id: bddaacc1-13c0-5d9c-9fbc-38adfae6d250
splat_kind: rule
splat_key: dnd2024:rules/combat_falling_off
splat_name: Falling Off
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_falling_off
  parameters:
    triggers:
    - mount_forced_movement_pending
    - rider_knocked_prone
    - mount_knocked_prone
    save:
      ability: dexterity
      dc: 10
    failure:
      dismount: true
      condition: prone
      destination:
        unoccupied: true
        within_mount_feet: 5
  implementation: dnd2024:rules_combat_falling_off
splat_contracts:
- id: dnd2024:rules_combat_falling_off
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - action_options
  - effects
  - resource_updates
  events:
  - movement.forced_pending
  - condition.applied
  capabilities:
  - action_economy
  - movement
  - combat
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  engine_gaps: &id001
  - dnd2024:rules_combat_falling_off
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 628c1bffb2522e7746a3e26a2ff16ab218a6f0262f82ad72c2905a57420f712f
  parent_name: Combat
  line_span:
    first: 1082
    count: 8
splat_clauses:
- key: source_rules
  text: '# Falling Off If an effect is about to move your mount against its will while you’re on it, you
    must succeed on a DC 10 Dexterity saving throw or fall off, landing with the Prone condition (see
    “Rules Glossary”) in an unoccupied space within 5 feet of the mount. While mounted, you must make
    the same save if you’re knocked Prone or the mount is.'
  sha256: 04b93a0fe358cde4038c5399df8e0d41f78fb84b8f0a072ef9e3805b35707ae8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Falling Off

<!-- SRD page 16 -->

If an effect is about to move your mount against its\
will while you’re on it, you must succeed on a DC 10\
Dexterity saving throw or fall off, landing with the\
Prone condition (see “Rules Glossary”) in an unoc-\
cupied space within 5 feet of the mount.\
While mounted, you must make the same save if\
you’re knocked Prone or the mount is.\
