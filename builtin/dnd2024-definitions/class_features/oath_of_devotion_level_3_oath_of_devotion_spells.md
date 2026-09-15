---
splat_schema: 1
splat_id: b08a4a47-e37e-55ae-85de-18860c15fb64
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells
splat_name: 'Level 3: Oath of Devotion Spells'
splat_origin:
  key: dnd2024:reference/pages/056
splat_sources:
- title: SRD 5.2.1
  locator: pp. 56
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells
  parameters:
    class: paladin
    always_prepared_by_class_level:
    - level: 3
      spells:
      - key: dnd2024:spells/protection_from_evil_and_good
      - key: dnd2024:spells/shield_of_faith
    - level: 5
      spells:
      - key: dnd2024:spells/aid
      - key: dnd2024:spells/zone_of_truth
    - level: 9
      spells:
      - key: dnd2024:spells/beacon_of_hope
      - key: dnd2024:spells/dispel_magic
    - level: 13
      spells:
      - key: dnd2024:spells/freedom_of_movement
      - key: dnd2024:spells/guardian_of_faith
    - level: 17
      spells:
      - key: dnd2024:spells/commune
      - key: dnd2024:spells/flame_strike
    retain_at_higher_levels: true
    counts_toward_class_prepared_limit: false
  implementation: dnd2024:class_features_oath_of_devotion_level_3_oath_of_devotion_spells
splat_contracts:
- id: dnd2024:class_features_oath_of_devotion_level_3_oath_of_devotion_spells
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
  - 56
  engine_gaps: &id001
  - dnd2024:class_features_oath_of_devotion_level_3_oath_of_devotion_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 4dc9293b6e5f7c64002d7b5d4eddb45f10bec514f0b9bdd709535844c3e9f9a8
  parent_name: Oath of Devotion
  line_span:
    first: 4735
    count: 14
splat_clauses:
- key: source_rules
  text: '# Level 3: Oath of Devotion Spells The magic of your oath ensures you always have certain spells
    ready; when you reach a Paladin level specified in the Oath of Devotion Spells table, you thereafter
    always have the listed spells prepared. Oath of Devotion Spells Paladin Level Spells 3 Protection
    from Evil and Good, Shield of Faith 5 Aid, Zone of Truth 9 Beacon of Hope, Dispel Magic 13 Freedom
    of Movement, Guardian of Faith 17 Commune, Flame Strike'
  sha256: de54414db82ff4d07e923eef155bb516cd47c789aceecace044a67222ad63e32
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Oath of Devotion Spells

<!-- SRD page 56 -->

The magic of your oath ensures you always have\
certain spells ready; when you reach a Paladin level\
specified in the Oath of Devotion Spells table, you\
thereafter always have the listed spells prepared.\
Oath of Devotion Spells\
Paladin Level Spells\
3 Protection from Evil and Good,\
Shield of Faith\
5 Aid, Zone of Truth\
9 Beacon of Hope, Dispel Magic\
13 Freedom of Movement,\
Guardian of Faith\
17 Commune, Flame Strike\
