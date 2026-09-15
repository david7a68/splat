---
splat_schema: 1
splat_id: b2b774d9-ecd2-5cd8-aaa3-e53ec6194472
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_3_life_domain_spells
splat_name: 'Level 3: Life Domain Spells'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/life_domain_level_3_life_domain_spells
  parameters:
    class: cleric
    always_prepared_by_class_level:
    - level: 3
      spells:
      - key: dnd2024:spells/aid
      - key: dnd2024:spells/bless
      - key: dnd2024:spells/cure_wounds
      - key: dnd2024:spells/lesser_restoration
    - level: 5
      spells:
      - key: dnd2024:spells/mass_healing_word
      - key: dnd2024:spells/revivify
    - level: 7
      spells:
      - key: dnd2024:spells/aura_of_life
      - key: dnd2024:spells/death_ward
    - level: 9
      spells:
      - key: dnd2024:spells/greater_restoration
      - key: dnd2024:spells/mass_cure_wounds
    retain_at_higher_levels: true
    counts_toward_class_prepared_limit: false
  implementation: dnd2024:class_features_life_domain_level_3_life_domain_spells
splat_contracts:
- id: dnd2024:class_features_life_domain_level_3_life_domain_spells
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.class_level_gained
  capabilities:
  - spell_preparation
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 40
  engine_gaps: &id001
  - dnd2024:class_features_life_domain_level_3_life_domain_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 5caca7063e4f6d9be092c6e6ef899a33869b6ebeab3316a0cfa27d86ffb97f13
  parent_name: Life Domain
  line_span:
    first: 3304
    count: 14
splat_clauses:
- key: source_rules
  text: '# Level 3: Life Domain Spells Your connection to this divine domain ensures you always have certain
    spells ready. When you reach a Cleric level specified in the Life Domain Spells table, you thereafter
    always have the listed spells prepared. Life Domain Spells Cleric Prepared Spells Level 3 Aid, Bless,
    Cure Wounds, Lesser Restoration 5 Mass Healing Word, Revivify 7 Aura of Life, Death Ward 9 Greater
    Restoration, Mass Cure Wounds'
  sha256: b78ecabd57cec758bfe7ff68a8a360cd47a1a7c57a1f2b80a77091478fc92adc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Life Domain Spells

<!-- SRD page 40 -->

Your connection to this divine domain ensures you\
always have certain spells ready. When you reach\
a Cleric level specified in the Life Domain Spells\
table, you thereafter always have the listed spells\
prepared.\
Life Domain Spells\
Cleric Prepared Spells\
Level\
3 Aid, Bless, Cure Wounds,\
Lesser Restoration\
5 Mass Healing Word, Revivify\
7 Aura of Life, Death Ward\
9 Greater Restoration, Mass Cure Wounds\
