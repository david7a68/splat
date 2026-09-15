---
splat_schema: 1
splat_id: 5ca55667-0040-5455-a5b2-3421baaf054a
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing
splat_name: 'Level 3: Dark One’s Blessing'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing
  parameters:
    triggers:
    - enemy_reduced_to_zero_by: self
    - enemy_reduced_to_zero_by: other
      enemy_distance_max_feet: 10
    temporary_hit_points:
      op: max
      args:
      - 1
      - op: add
        args:
        - stat: abilities.charisma.modifier
        - stat: classes.warlock.level
  implementation: dnd2024:class_features_fiend_patron_level_3_dark_one_s_blessing
splat_contracts:
- id: dnd2024:class_features_fiend_patron_level_3_dark_one_s_blessing
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - creature.hit_points_became_zero
  capabilities:
  - temporary_hit_points
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 76
  engine_gaps: &id001
  - dnd2024:class_features_fiend_patron_level_3_dark_one_s_blessing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 3eb0c86a0e3c3b9ce741e18169bb8ceb5eadb9a149f8018622adbb80d5e60408
  parent_name: Fiend Patron
  line_span:
    first: 6480
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 3: Dark One’s Blessing When you reduce an enemy to 0 Hit Points, you gain Temporary Hit
    Points equal to your Charisma modifier plus your Warlock level (minimum of 1 Temporary Hit Point).
    You also gain this benefit if someone else reduces an enemy within 10 feet of you to 0 Hit Points.'
  sha256: c5c1bdf9dbc3a6d5f8f247bb5d811226360dd87b1b7799cc29413f740d40115b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Dark One’s Blessing

<!-- SRD page 76 -->

When you reduce an enemy to 0 Hit Points, you gain\
Temporary Hit Points equal to your Charisma mod-\
ifier plus your Warlock level (minimum of 1 Tempo-\
rary Hit Point). You also gain this benefit if someone\
else reduces an enemy within 10 feet of you to 0 Hit\
Points.\
