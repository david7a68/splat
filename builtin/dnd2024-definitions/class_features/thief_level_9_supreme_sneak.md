---
splat_schema: 1
splat_id: b936ddb1-08bf-5e5c-bfda-8ca0af67f130
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_9_supreme_sneak
splat_name: 'Level 9: Supreme Sneak'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/thief_level_9_supreme_sneak
  parameters:
    extends: rogue.cunning_strike
    option: stealth_attack
    cost:
      resource: sneak_attack.d6_dice
      amount: 1
    requires:
      invisible_source: hide_action
    attack_ends_invisible: false
    conditional_retention:
      check: actor.turn.end
      requires_cover_any:
      - three_quarters
      - total
      on_requirement_failure: end_hide_invisible
    class_scope:
      class: rogue
      minimum_level: 9
  implementation: dnd2024:class_features_thief_level_9_supreme_sneak
splat_contracts:
- id: dnd2024:class_features_thief_level_9_supreme_sneak
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
  - sneak_attack.damage_pending
  - attack.roll
  - turn.end
  capabilities:
  - character_features
  - resources
  - conditions
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 64
  engine_gaps: &id001
  - dnd2024:class_features_thief_level_9_supreme_sneak
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 7d775ae729c6f13e433732d8e9fdd7a5f8ab6e075151620e3e4ed0d7fbe468df
  parent_name: Thief
  line_span:
    first: 5373
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 9: Supreme Sneak You gain the following Cunning Strike option. Stealth Attack (Cost:
    1d6). If you have the Hide action’s Invisible condition, this attack doesn’t end that condition on
    you if you end the turn behind Three-Quarters Cover or Total Cover.'
  sha256: af8f21217bcc865b3db3999066fa007d8a1ee1218eee609cb2e024c390e2c194
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 9: Supreme Sneak

<!-- SRD page 64 -->

You gain the following Cunning Strike option.\
Stealth Attack (Cost: 1d6). If you have the Hide\
action’s Invisible condition, this attack doesn’t end\
that condition on you if you end the turn behind\
Three-Quarters Cover or Total Cover.\
