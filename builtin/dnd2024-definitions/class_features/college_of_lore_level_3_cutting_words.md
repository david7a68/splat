---
splat_schema: 1
splat_id: 68549fb5-f176-5a00-b8e4-286e0a53312b
splat_kind: rule
splat_key: dnd2024:class_features/college_of_lore_level_3_cutting_words
splat_name: 'Level 3: Cutting Words'
splat_origin:
  key: dnd2024:reference/pages/035
splat_sources:
- title: SRD 5.2.1
  locator: pp. 35
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/college_of_lore_level_3_cutting_words
  parameters:
    action: reaction
    trigger_any:
    - visible_creature.damage_rolled
    - visible_creature.ability_check_succeeded
    - visible_creature.attack_roll_succeeded
    range_feet: 60
    resource_cost:
      id: bard.bardic_inspiration
      amount: 1
    roll:
      var: actor.bardic_inspiration_die
    modify: subtract_from_trigger_roll
    recompute_success: true
    class_scope:
      class: bard
      minimum_level: 3
  implementation: dnd2024:class_features_college_of_lore_level_3_cutting_words
splat_contracts:
- id: dnd2024:class_features_college_of_lore_level_3_cutting_words
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
  - damage.rolled
  - ability.check_succeeded
  - attack.roll_succeeded
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
  - 35
  engine_gaps: &id001
  - dnd2024:class_features_college_of_lore_level_3_cutting_words
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 0691751d05c577a311ad4764b5663ef155f290ae1ed1b53303b3c508149953e1
  parent_name: College of Lore
  line_span:
    first: 2884
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 3: Cutting Words You learn to use your wit to supernaturally distract, confuse, and otherwise
    sap the confidence and competence of others. When a creature that you can see within 60 feet of yourself
    makes a damage roll or succeeds on an ability check or attack roll, you can take a Reaction to expend
    one use of your Bardic Inspiration; roll your Bardic Inspiration die, and subtract the number rolled
    from the creature’s roll, reducing the damage or potentially turning the success into a failure.'
  sha256: b21df2895c46d0a804d9153a0544740bd02687839853235a2ebf90ff47106f90
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Cutting Words

<!-- SRD page 35 -->

You learn to use your wit to supernaturally distract,\
confuse, and otherwise sap the confidence and com-\
petence of others. When a creature that you can see\
within 60 feet of yourself makes a damage roll or\
succeeds on an ability check or attack roll, you can\
take a Reaction to expend one use of your Bardic\
Inspiration; roll your Bardic Inspiration die, and\
subtract the number rolled from the creature’s roll,\
reducing the damage or potentially turning the suc-\
cess into a failure.\
