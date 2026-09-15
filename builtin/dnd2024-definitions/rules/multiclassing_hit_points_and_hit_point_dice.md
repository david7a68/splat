---
splat_schema: 1
splat_id: 7582bdb4-11fc-5bbb-b962-a27a64a4cea2
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_hit_points_and_hit_point_dice
splat_name: Hit Points and Hit Point Dice
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing_hit_points_and_hit_point_dice
  parameters:
    class_level_1_maximum_hit_die_hp_only_if:
      total_character_level: 1
    new_class_at_later_character_level: normal_level_gain_hp
    hit_dice:
      sum_all_classes: true
      pool_by_die_size: true
      track_different_die_sizes_separately: true
  implementation: dnd2024:rules_multiclassing_hit_points_and_hit_point_dice
splat_contracts:
- id: dnd2024:rules_multiclassing_hit_points_and_hit_point_dice
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - hit_points
  - hit_dice
  events: []
  capabilities:
  - multiclassing
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 25
  engine_gaps: &id001
  - dnd2024:rules_multiclassing_hit_points_and_hit_point_dice
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 5940806d567250aae313852fcc252d76fd57c9cd555c0d848cc7be34c8b7f22d
  parent_name: Multiclassing
  line_span:
    first: 1924
    count: 15
splat_clauses:
- key: source_rules
  text: '# Hit Points and Hit Point Dice You gain the Hit Points from your new class as described for
    levels after 1. You gain the level 1 Hit Points for a class only when your total character level is
    1. Add together the Hit Dice granted by all your classes to form your pool of Hit Dice. If these dice
    are the same die type, you can pool them together. For example, both the Fighter and the Paladin have
    a d10 Hit Die, so if you are a level 5 Fighter / level 5 Paladin, you have ten d10 Hit Dice. If your
    classes give you Hit Dice of different types, track them separately. If you are a level 5 Cleric /
    level 5 Paladin, for example, you have five d8 Hit Dice and five d10 Hit Dice.'
  sha256: 2257dadd3ba683ddbd0374535c9abb73bc6cbc53f035de3e9ee976f481957ef5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hit Points and Hit Point Dice

<!-- SRD page 25 -->

You gain the Hit Points from your new class as de-\
scribed for levels after 1. You gain the level 1 Hit\
Points for a class only when your total character\
level is 1.\
Add together the Hit Dice granted by all your\
classes to form your pool of Hit Dice. If these dice\
are the same die type, you can pool them together.\
For example, both the Fighter and the Paladin have\
a d10 Hit Die, so if you are a level 5 Fighter / level 5\
Paladin, you have ten d10 Hit Dice. If your classes\
give you Hit Dice of different types, track them sep-\
arately. If you are a level 5 Cleric / level 5 Paladin,\
for example, you have five d8 Hit Dice and five d10\
Hit Dice.\
