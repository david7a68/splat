---
splat_schema: 1
splat_id: 1a205b4b-a700-5650-92f9-e6363618c8e7
splat_kind: rule
splat_key: dnd2024:rules/warlock_repelling_blast
splat_name: Repelling Blast
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_repelling_blast
  parameters:
    prerequisite:
      warlock_level_minimum: 2
      known_warlock_cantrip_deals_damage_via_attack_roll: true
    cantrip_choice:
      known: true
      class: warlock
      requires_attack_roll: true
    hit_effect:
      target_maximum_size: large
      optional_push:
        feet_maximum: 10
        direction: straight_away_from_caster
    repeatable: true
    repeat_choice_must_differ: true
  implementation: dnd2024:rule_warlock_repelling_blast
splat_contracts:
- id: dnd2024:rule_warlock_repelling_blast
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - feature_effects
  events:
  - attack.hit
  capabilities:
  - invocations
  - movement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 74
  engine_gaps: &id001
  - dnd2024:rule_warlock_repelling_blast
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 586d20450abb5fdce2856e541f715fa480dba1c94418609042737079f3a8d3d6
  parent_name: Warlock
  line_span:
    first: 6343
    count: 10
splat_clauses:
- key: source_rules
  text: '# Repelling Blast Prerequisite: Level 2+ Warlock, a Warlock Cantrip That Deals Damage via an
    Attack Roll Choose one of your known Warlock cantrips that requires an attack roll. When you hit a
    Large or smaller creature with that cantrip, you can push the creature up to 10 feet straight away
    from you. Repeatable. You can gain this invocation more than once. Each time you do so, choose a different
    eligible cantrip.'
  sha256: 2156b4dd29bf89f938aa0622bfa0b1fe175be9773b40b03c82bf3351effd494e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Repelling Blast

<!-- SRD page 74 -->

Prerequisite: Level 2+ Warlock, a Warlock Cantrip\
That Deals Damage via an Attack Roll\
Choose one of your known Warlock cantrips that\
requires an attack roll. When you hit a Large or\
smaller creature with that cantrip, you can push the\
creature up to 10 feet straight away from you.\
Repeatable. You can gain this invocation more\
than once. Each time you do so, choose a different\
eligible cantrip.\
