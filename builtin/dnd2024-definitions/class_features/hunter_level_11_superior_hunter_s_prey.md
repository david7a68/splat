---
splat_schema: 1
splat_id: 96a6cef6-b93e-5400-99a8-9947e4688b71
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_11_superior_hunter_s_prey
splat_name: 'Level 11: Superior Hunter’s Prey'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/hunter_level_11_superior_hunter_s_prey
  parameters:
    trigger: actor.damages_own_hunters_mark_target
    limit:
      count: 1
      period: turn
    optional: true
    additional_target:
      type: creature
      different_from_original: true
      visible_to_actor: true
      within_original_target_feet: 30
    damage: triggering_hunters_mark.extra_damage
    class_scope:
      class: ranger
      minimum_level: 11
  implementation: dnd2024:class_features_hunter_level_11_superior_hunter_s_prey
splat_contracts:
- id: dnd2024:class_features_hunter_level_11_superior_hunter_s_prey
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
  - damage.dealt
  capabilities:
  - character_features
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  engine_gaps: &id001
  - dnd2024:class_features_hunter_level_11_superior_hunter_s_prey
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: fd094553c0312133a71df46ae4d776ea439d7cc1bbc8a8e1850aed3d1c39abdc
  parent_name: Hunter
  line_span:
    first: 5117
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 11: Superior Hunter’s Prey Once per turn when you deal damage to a creature marked by
    your Hunter’s Mark, you can also deal that spell’s extra damage to a different creature that you can
    see within 30 feet of the first creature.'
  sha256: 75a63eb3f07496c76f8255fe8846947804b82b83ffb52b5ff4810bda7009c34c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 11: Superior Hunter’s Prey

<!-- SRD page 61 -->

Once per turn when you deal damage to a creature\
marked by your Hunter’s Mark, you can also deal\
that spell’s extra damage to a different creature that\
you can see within 30 feet of the first creature.\
