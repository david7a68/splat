---
splat_schema: 1
splat_id: 90f2f9db-29eb-5ade-a417-c59abc81984c
splat_kind: rule_group
splat_key: dnd2024:species/human
splat_name: Human
splat_origin:
  key: dnd2024:reference/pages/086
splat_sources:
- title: SRD 5.2.1
  locator: pp. 86
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: species
splat_rule:
  mechanic: dnd2024:species/human
  implementation: dnd2024:species_human
  parameters:
    creature_type: humanoid
    size_choice:
      count: 1
      options:
      - size: medium
        height_feet_approximate:
          minimum: 4
          maximum: 7
      - size: small
        height_feet_approximate:
          minimum: 2
          maximum: 4
    speed_feet: 30
    resourceful:
      event: rest.long_rest_completed
      grant: heroic_inspiration
    skillful:
      proficiency_choice:
        count: 1
        category: any_skill
    versatile:
      feat_choice:
        count: 1
        category: origin
splat_contracts:
- id: dnd2024:species_human
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - proficiencies
  events:
  - character.species_selected
  - rest.long_rest_completed
  capabilities:
  - character_creation
  - inspiration
  - proficiencies
  - feats
splat_conversion:
  status: specified
  category: species
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 86
  engine_gaps: &id001
  - dnd2024:species_human
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 1a94eb8ec4eee0b4bc1f1c004e5b3db5f074d6aa4de3dcb3c32dd4fca27efc56
  line_span:
    first: 7382
    count: 20
splat_clauses:
- key: source_rules
  text: '# Human Creature Type: Humanoid Size: Medium (about 4–7 feet tall) or Small (about 2–4 feet tall),
    chosen when you select this species Speed: 30 feet As a Human, you have these special traits. Resourceful.
    You gain Heroic Inspiration whenever you finish a Long Rest. Skillful. You gain proficiency in one
    skill of your choice. Versatile. You gain an Origin feat of your choice (see “Feats”). Skilled is
    recommended. Fiendish Legacies Legacy Level 1 Abyssal You have Resistance to Poison damage. You also
    know the Poison Spray cantrip. Chthonic You have Resistance to Necrotic damage. You also know the
    Chill Touch cantrip. Infernal You have Resistance to Fire damage. You also know the Fire Bolt cantrip.'
  sha256: f6f913be40ecca6e6f462dc0da3424a8085bc7a7457a6b6bd8b20cc5dc49c385
  status: contract_specified
splat_requires_mechanics: *id001
---

# Human

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Medium (about 4–7 feet tall) or Small (about 2–4\
feet tall), chosen when you select this species\
Speed: 30 feet\
As a Human, you have these special traits.\
Resourceful. You gain Heroic Inspiration when-\
ever you finish a Long Rest.\
Skillful. You gain proficiency in one skill of your\
choice.\
Versatile. You gain an Origin feat of your choice\
(see “Feats”). Skilled is recommended.\
Fiendish Legacies\
Legacy Level 1\
Abyssal You have Resistance to Poison damage.\
You also know the Poison Spray cantrip.\
Chthonic You have Resistance to Necrotic damage.\
You also know the Chill Touch cantrip.\
Infernal You have Resistance to Fire damage.\
You also know the Fire Bolt cantrip.\
