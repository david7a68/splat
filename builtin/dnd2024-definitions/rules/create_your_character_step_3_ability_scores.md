---
splat_schema: 1
splat_id: 4e28e57f-bfd5-5688-afe1-58d7a4414d03
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_step_3_ability_scores
splat_name: 'Step 3: Ability Scores'
splat_origin:
  key: dnd2024:reference/pages/020
splat_sources:
- title: SRD 5.2.1
  locator: pp. 20
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_step_3_ability_scores
  parameters:
    generate_count: 6
    assign:
      abilities:
      - strength
      - dexterity
      - constitution
      - intelligence
      - wisdom
      - charisma
      each_number_once: true
  implementation: dnd2024:rule_create_your_character_step_3_ability_scores
splat_contracts:
- id: dnd2024:rule_create_your_character_step_3_ability_scores
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 20
  engine_gaps: &id001
  - dnd2024:rule_create_your_character_step_3_ability_scores
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 90b065cb3ace6563f87c615f806afc30ffd5e2a23b0647cb15d16e192a621a76
  parent_name: Create Your Character
  line_span:
    first: 1507
    count: 6
splat_clauses:
- key: source_rules
  text: '# Step 3: Ability Scores To determine your character’s ability scores, you first generate a set
    of six numbers using the instructions below and then assign them to your six abilities. “Playing the
    Game” explains what each ability means.'
  sha256: c4bb5082cd44b54ab0e8a297e6d6b61b02624195be0e13731944d48016799a14
  status: contract_specified
splat_requires_mechanics: *id001
---

# Step 3: Ability Scores

<!-- SRD page 20 -->

To determine your character’s ability scores, you\
first generate a set of six numbers using the instruc-\
tions below and then assign them to your six abili-\
ties. “Playing the Game” explains what each ability\
means.\
