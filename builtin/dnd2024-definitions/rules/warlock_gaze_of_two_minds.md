---
splat_schema: 1
splat_id: 681c6d78-669a-52f2-9a64-72e6f5ce979c
splat_kind: rule
splat_key: dnd2024:rules/warlock_gaze_of_two_minds
splat_name: Gaze of Two Minds
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_gaze_of_two_minds
  parameters:
    prerequisites:
      warlock_level_minimum: 5
    activation:
      economy: bonus_action
      range: touch
      target_willing: true
    duration:
      until: actor_next_turn_end
    maintenance:
      economy: bonus_action
      'on': subsequent_turn
      same_plane_required: true
      extend_until: actor_next_turn_end
      failure_to_maintain: end_connection
    perception:
      use_target_senses: true
      benefit_special_senses: true
    spell_origin_choice:
      options:
      - actor_space
      - target_space
      maximum_separation_feet: 60
  implementation: dnd2024:rules_warlock_gaze_of_two_minds
splat_contracts:
- id: dnd2024:rules_warlock_gaze_of_two_minds
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - effects
  - resource_updates
  events: []
  capabilities:
  - class_features
  - effects
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 73
  engine_gaps: &id001
  - dnd2024:rules_warlock_gaze_of_two_minds
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 6d75995c520ff313c2fbdbcb78f341aac3c31a9719b3969248717f8a700d80ff
  parent_name: Warlock
  line_span:
    first: 6199
    count: 15
splat_clauses:
- key: source_rules
  text: '# Gaze of Two Minds Prerequisite: Level 5+ Warlock You can use a Bonus Action to touch a willing
    creature and perceive through its senses until the end of your next turn. As long as the creature
    is on the same plane of existence as you, you can take a Bonus Action on subsequent turns to maintain
    this connection, extending the duration until the end of your next turn. The connection ends if you
    don’t maintain it in this way. While perceiving through the other creature’s senses, you benefit from
    any special senses possessed by that creature, and you can cast spells as if you were in your space
    or the other creature’s space if the two of you are within 60 feet of each other.'
  sha256: cd95239a80cd8b8b8ff3cab1309238ffc080688efd19a3d192707395c1423ce7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gaze of Two Minds

<!-- SRD page 73 -->

Prerequisite: Level 5+ Warlock\
You can use a Bonus Action to touch a willing crea-\
ture and perceive through its senses until the end\
of your next turn. As long as the creature is on the\
same plane of existence as you, you can take a Bo-\
nus Action on subsequent turns to maintain this\
connection, extending the duration until the end\
of your next turn. The connection ends if you don’t\
maintain it in this way.\
While perceiving through the other creature’s\
senses, you benefit from any special senses pos-\
sessed by that creature, and you can cast spells as if\
you were in your space or the other creature’s space\
if the two of you are within 60 feet of each other.\
