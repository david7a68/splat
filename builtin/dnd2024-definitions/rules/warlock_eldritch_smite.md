---
splat_schema: 1
splat_id: 000658d6-364c-5983-a6e0-70543064f0f9
splat_kind: rule
splat_key: dnd2024:rules/warlock_eldritch_smite
splat_name: Eldritch Smite
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72, 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_eldritch_smite
  parameters:
    prerequisites:
      warlock_level_minimum: 5
      invocation: pact_of_the_blade
    trigger: pact_weapon.hit_creature
    uses_per_turn: 1
    cost:
      resource: pact_magic_slot
      amount: 1
    extra_damage:
      type: force
      dice:
        count:
          op: add
          left: 1
          right: expended_slot_level
        sides: 8
    optional_prone:
      target_size_maximum: huge
  implementation: dnd2024:rules_warlock_eldritch_smite
splat_contracts:
- id: dnd2024:rules_warlock_eldritch_smite
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - effects
  - resource_updates
  events:
  - attack.hit
  capabilities:
  - class_features
  - effects
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 72
  - 73
  engine_gaps: &id001
  - dnd2024:rules_warlock_eldritch_smite
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 30a42070ca45fe9331a14300a89c6258d1e63f17bac8307387580628b69d9d1f
  parent_name: Warlock
  line_span:
    first: 6173
    count: 9
splat_clauses:
- key: source_rules
  text: '# Eldritch Smite Prerequisite: Level 5+ Warlock, Pact of the Blade Invocation Once per turn when
    you hit a creature with your pact weapon, you can expend a Pact Magic spell slot to deal an extra
    1d8 Force damage to the target, plus another 1d8 per level of the spell slot, and you can give the
    target the Prone condition if it is Huge or smaller.'
  sha256: 2ea2c9cd10e0ac7e70259a85eeee5b7a9e1dbda63dd44d87920fbedc10fb6f66
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eldritch Smite

<!-- SRD page 72 -->

Prerequisite: Level 5+ Warlock, Pact of the Blade\
Invocation\
Once per turn when you hit a creature with your\
pact weapon, you can expend a Pact Magic spell slot\
to deal an extra 1d8 Force damage to the target,\

<!-- SRD page 73 -->

plus another 1d8 per level of the spell slot, and you\
can give the target the Prone condition if it is Huge\
or smaller.\
