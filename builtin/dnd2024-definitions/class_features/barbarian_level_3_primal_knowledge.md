---
splat_schema: 1
splat_id: e3f958a2-c2be-5f89-8deb-0939ae637dd2
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_3_primal_knowledge
splat_name: 'Level 3: Primal Knowledge'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_3_primal_knowledge
  parameters:
    skill_proficiency:
      count: 1
      choices:
      - animal_handling
      - athletics
      - intimidation
      - nature
      - perception
      - survival
    ability_substitution:
      requires:
        rage_active: true
      skills:
      - acrobatics
      - intimidation
      - perception
      - stealth
      - survival
      ability: strength
      optional: true
    class_scope:
      class: barbarian
      minimum_level: 3
  implementation: dnd2024:class_features_barbarian_level_3_primal_knowledge
splat_contracts:
- id: dnd2024:class_features_barbarian_level_3_primal_knowledge
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
  - character.feature_acquired
  - ability.check_pending
  capabilities:
  - character_features
  - proficiencies
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_3_primal_knowledge
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 4aa5f28d0d262a2213c98f027592b34ad288e29eececdae7cfb55f59140cc5b5
  parent_name: Barbarian
  line_span:
    first: 2357
    count: 12
splat_clauses:
- key: source_rules
  text: '# Level 3: Primal Knowledge You gain proficiency in another skill of your choice from the skill
    list available to Barbarians at level 1. In addition, while your Rage is active, you can channel primal
    power when you attempt certain tasks; whenever you make an ability check using one of the following
    skills, you can make it as a Strength check even if it normally uses a different ability: Acrobatics,
    Intimidation, Perception, Stealth, or Survival. When you use this ability, your Strength represents
    primal power coursing through you, honing your agility, bearing, and senses.'
  sha256: 406ad5c3cf8537f189b9e92c1e95972243e321197dc25ed9102c943ec362dc3f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Primal Knowledge

<!-- SRD page 29 -->

You gain proficiency in another skill of your choice\
from the skill list available to Barbarians at level 1.\
In addition, while your Rage is active, you can\
channel primal power when you attempt certain\
tasks; whenever you make an ability check using\
one of the following skills, you can make it as a\
Strength check even if it normally uses a differ-\
ent ability: Acrobatics, Intimidation, Perception,\
Stealth, or Survival. When you use this ability, your\
Strength represents primal power coursing through\
you, honing your agility, bearing, and senses.\
