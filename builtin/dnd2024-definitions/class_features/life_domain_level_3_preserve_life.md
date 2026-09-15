---
splat_schema: 1
splat_id: b32a75fd-5ca0-5dbc-b4a7-29e38ed82236
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_3_preserve_life
splat_name: 'Level 3: Preserve Life'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/life_domain_level_3_preserve_life
  parameters:
    action: magic
    requires_item: holy_symbol
    resource_cost:
      id: cleric.channel_divinity
      amount: 1
    healing_pool:
      op: multiply
      args:
      - 5
      - var: actor.class_levels.cleric
    targets:
      bloodied: true
      range_feet: 30
      self_allowed: true
      selection: actor_choice
    distribution:
      chooser: actor
      type: integer_allocation
      sum_maximum: healing_pool
      per_target_hp_cap:
        op: multiply
        args:
        - var: target.hit_point_maximum
        - 0.5
    class_scope:
      class: cleric
      minimum_level: 3
  implementation: dnd2024:class_features_life_domain_level_3_preserve_life
splat_contracts:
- id: dnd2024:class_features_life_domain_level_3_preserve_life
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
  - healing
  - resource_expenditure
  events:
  - action.magic
  capabilities:
  - character_features
  - resources
  - healing
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 40
  engine_gaps: &id001
  - dnd2024:class_features_life_domain_level_3_preserve_life
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 75987bdd24c8eac21f5516569a8c0957133897e4a35f346b513829e956b71669
  parent_name: Life Domain
  line_span:
    first: 3318
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 3: Preserve Life As a Magic action, you present your Holy Symbol and expend a use of
    your Channel Divinity to evoke healing energy that can restore a number of Hit Points equal to five
    times your Cleric level. Choose Bloodied creatures within 30 feet of yourself (which can include you),
    and divide those Hit Points among them. This feature can restore a creature to no more than half its
    Hit Point maximum.'
  sha256: 8e44a2a08f628ffa6d25ac2afc801da38f30cd96bf3a13e5dcfdf45ea69bede1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Preserve Life

<!-- SRD page 40 -->

As a Magic action, you present your Holy Symbol\
and expend a use of your Channel Divinity to evoke\
healing energy that can restore a number of Hit\
Points equal to five times your Cleric level. Choose\
Bloodied creatures within 30 feet of yourself (which\
can include you), and divide those Hit Points among\
them. This feature can restore a creature to no\
more than half its Hit Point maximum.\
