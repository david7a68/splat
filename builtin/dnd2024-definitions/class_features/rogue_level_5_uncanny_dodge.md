---
splat_schema: 1
splat_id: dd38f4d0-ae83-5739-92d4-6507a1d0e480
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_5_uncanny_dodge
splat_name: 'Level 5: Uncanny Dodge'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_5_uncanny_dodge
  parameters:
    action: reaction
    trigger: visible_attacker.attack_hits_actor
    damage_multiplier: 0.5
    rounding: down
    scope: trigger_attack.damage_against_actor
    class_scope:
      class: rogue
      minimum_level: 5
  implementation: dnd2024:class_features_rogue_level_5_uncanny_dodge
splat_contracts:
- id: dnd2024:class_features_rogue_level_5_uncanny_dodge
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - attack.hit
  - damage.pending
  capabilities:
  - character_features
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_5_uncanny_dodge
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 027aa7de2a037bc96275d4655da1942a033897071027837b528a9ce506fdaaaa
  parent_name: Rogue
  line_span:
    first: 5295
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 5: Uncanny Dodge When an attacker that you can see hits you with an attack roll, you
    can take a Reaction to halve the attack’s damage against you (round down).'
  sha256: b2add0f52bb9ecfbd1a129462ee17153112363b3e7071939ec2c9fed58617e41
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Uncanny Dodge

<!-- SRD page 63 -->

When an attacker that you can see hits you with an\
attack roll, you can take a Reaction to halve the at-\
tack’s damage against you (round down).\
