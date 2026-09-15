---
splat_schema: 1
splat_id: 1459f9af-b62b-502a-a285-1112785c55ab
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_18_superior_defense
splat_name: 'Level 18: Superior Defense'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_18_superior_defense
  parameters:
    trigger: actor.turn.start
    optional: true
    cost:
      resource: monk.focus_points
      amount: 3
    duration:
      minutes: 1
    ends_on:
    - actor.incapacitated
    resistance:
      all_damage_types: true
      except:
      - force
    class_scope:
      class: monk
      minimum_level: 18
  implementation: dnd2024:class_features_monk_level_18_superior_defense
splat_contracts:
- id: dnd2024:class_features_monk_level_18_superior_defense
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
  - turn.start
  - condition.applied
  - damage.pending
  capabilities:
  - character_features
  - resources
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_18_superior_defense
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: c76e45c9fd1370d9e0a6b3e37de3ead2ad32dcbf58f6eadb81ebbd56ef383428
  parent_name: Monk
  line_span:
    first: 4336
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 18: Superior Defense At the start of your turn, you can expend 3 Focus Points to bolster
    yourself against harm for 1 minute or until you have the Incapacitated condition. During that time,
    you have Resistance to all damage except Force damage.'
  sha256: 87f42cae1f8025e3f3985e1667575ce321af3d489fc6d4677e4cdcba0592ef35
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Superior Defense

<!-- SRD page 52 -->

At the start of your turn, you can expend 3 Focus\
Points to bolster yourself against harm for 1 min-\
ute or until you have the Incapacitated condition.\
During that time, you have Resistance to all damage\
except Force damage.\
