---
splat_schema: 1
splat_id: 92d39316-b0ec-5655-a7c1-c5df64ea7786
splat_kind: rule
splat_key: dnd2024:rules/combat_cover
splat_name: Cover
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_cover
  parameters:
    requires: effect_origin_opposite_side_of_cover
    stacking: highest_degree_only
    degrees:
    - degree: half
      minimum_covered_fraction: 0.5
      ac_bonus: 2
      dexterity_save_bonus: 2
      creature_can_supply: true
    - degree: three_quarters
      minimum_covered_fraction: 0.75
      ac_bonus: 5
      dexterity_save_bonus: 5
    - degree: total
      minimum_covered_fraction: 1
      direct_targeting: false
  implementation: dnd2024:rule_combat_cover
splat_contracts:
- id: dnd2024:rule_combat_cover
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - cover_modifiers
  events: []
  capabilities:
  - cover
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 15
  engine_gaps: &id001
  - dnd2024:rule_combat_cover
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 92e1064dabb15c93ab7f3c36e2557599bb57a2cce1292f1cf6aa28b253d5672e
  parent_name: Combat
  line_span:
    first: 983
    count: 24
splat_clauses:
- key: source_rules
  text: '# Cover Walls, trees, creatures, and other obstacles can provide cover, making a target more
    difficult to harm. As detailed in the Cover table, there are three degrees of cover, each of which
    gives a different benefit to a target. A target can benefit from cover only when an attack or other
    effect originates on the opposite side of the cover. If a target is behind multiple sources of cover,
    only the most protective degree of cover applies; the degrees aren’t added together. For example,
    if a target is behind a creature that gives Half Cover and a tree trunk that gives Three-Quarters
    Cover, the target has Three-Quarters Cover. Cover Degree Benefit to Target Offered By … Half +2 bonus
    to AC Another creature or and Dexterity an object that covers at saving throws least half of the target
    Three- +5 bonus to AC An object that covers at Quarters and Dexterity least three-quarters of saving
    throws the target Total Can’t be targeted An object that covers directly the whole target'
  sha256: 8eed097a3f87203640993b6345d1976c0b6686266b9f832c26ba44a1103448f2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cover

<!-- SRD page 15 -->

Walls, trees, creatures, and other obstacles can pro-\
vide cover, making a target more difficult to harm.\
As detailed in the Cover table, there are three de-\
grees of cover, each of which gives a different bene-\
fit to a target.\
A target can benefit from cover only when an at-\
tack or other effect originates on the opposite side\
of the cover. If a target is behind multiple sources\
of cover, only the most protective degree of cover\
applies; the degrees aren’t added together. For ex-\
ample, if a target is behind a creature that gives Half\
Cover and a tree trunk that gives Three-Quarters\
Cover, the target has Three-Quarters Cover.\
Cover\
Degree Benefit to Target Offered By …\
Half +2 bonus to AC Another creature or\
and Dexterity an object that covers at\
saving throws least half of the target\
Three- +5 bonus to AC An object that covers at\
Quarters and Dexterity least three-quarters of\
saving throws the target\
Total Can’t be targeted An object that covers\
directly the whole target\
