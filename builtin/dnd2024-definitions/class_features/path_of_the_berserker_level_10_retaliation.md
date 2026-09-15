---
splat_schema: 1
splat_id: 764f6967-da91-5443-adc4-a7824accc3ff
splat_kind: rule
splat_key: dnd2024:class_features/path_of_the_berserker_level_10_retaliation
splat_name: 'Level 10: Retaliation'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/path_of_the_berserker_level_10_retaliation
  parameters:
    trigger:
      event: damage.taken
      source_type: creature
      source_distance_max_feet: 5
    action: reaction
    attack:
      count: 1
      target: damage_source
      range: melee
      allowed:
      - weapon
      - unarmed_strike
  implementation: dnd2024:class_features_path_of_the_berserker_level_10_retaliation
splat_contracts:
- id: dnd2024:class_features_path_of_the_berserker_level_10_retaliation
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  events:
  - damage.taken
  capabilities:
  - attacks
  - reactions
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_path_of_the_berserker_level_10_retaliation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 65bbd4a9da0fbf5571d92670bef4a2c41cf4bbbdd970771b394a86619e72c616
  parent_name: Path of the Berserker
  line_span:
    first: 2472
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 10: Retaliation When you take damage from a creature that is within 5 feet of you, you
    can take a Reaction to make one melee attack against that creature, using a weapon or an Unarmed Strike.'
  sha256: 2f2f98bf3e94fd8129e2d6f53c8f4fcc24b2cd47554be3321afe87cf081e49cc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Retaliation

<!-- SRD page 30 -->

When you take damage from a creature that is\
within 5 feet of you, you can take a Reaction to\
make one melee attack against that creature, using\
a weapon or an Unarmed Strike.\
