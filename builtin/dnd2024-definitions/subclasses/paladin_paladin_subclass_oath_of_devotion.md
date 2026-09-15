---
splat_schema: 1
splat_id: d4f23084-9bb3-5bee-a9a5-3af40aae9d48
splat_kind: rule_group
splat_key: dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion
splat_name: 'Paladin Subclass: Oath of Devotion'
splat_origin:
  key: dnd2024:reference/pages/056
splat_sources:
- title: SRD 5.2.1
  locator: pp. 56
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
splat_rule:
  mechanic: dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion
  implementation: dnd2024:subclasses_paladin_paladin_subclass_oath_of_devotion
  parameters:
    class: paladin
    minimum_class_level: 3
    feature_grants:
    - class_level: 15
      definition:
        key: dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection
    - class_level: 20
      definition:
        key: dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus
    - class_level: 3
      definition:
        key: dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells
    - class_level: 3
      definition:
        key: dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon
    - class_level: 7
      definition:
        key: dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion
    grant_eligible_features:
      'on':
      - subclass.selected
      - character.level_gained
      comparison: class_level_at_least_threshold
      persistent: true
splat_contracts:
- id: dnd2024:subclasses_paladin_paladin_subclass_oath_of_devotion
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - feature_grants
  events:
  - subclass.selected
  - character.level_gained
  capabilities:
  - character_features
  - spellcasting
  - auras
splat_conversion:
  status: specified
  category: subclass
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 56
  engine_gaps: &id001
  - dnd2024:subclasses_paladin_paladin_subclass_oath_of_devotion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 3eeb6282cc3608546253b7aaa308c8051729ebdcca6bed80e4c3acdd3c75855c
  parent_name: Paladin
  line_span:
    first: 4718
    count: 17
splat_clauses:
- key: source_rules
  text: '# Paladin Subclass: Oath of Devotion Uphold the Ideals of Justice and Order The Oath of Devotion
    binds Paladins to the ideals of justice and order. These Paladins meet the archetype of the knight
    in shining armor. They hold themselves to the highest standards of conduct, and some—for better or
    worse—hold the rest of the world to the same standards. Many who swear this oath are devoted to gods
    of law and good and use their gods’ tenets as the measure of personal devotion. Others hold angels
    as their ideals and incorporate images of angelic wings into their helmets or coats of arms. These
    paladins share the following tenets: • Let your word be your promise. • Protect the weak and never
    fear to act. • Let your honorable deeds be an example.'
  sha256: 338ec45dda6469b36eadd5e580f5b1c734cce57cc0fca35e0108c60b10e33504
  status: contract_specified
splat_requires_mechanics: *id001
---

# Paladin Subclass: Oath of Devotion

<!-- SRD page 56 -->

Uphold the Ideals of Justice and Order\
The Oath of Devotion binds Paladins to the ideals\
of justice and order. These Paladins meet the ar-\
chetype of the knight in shining armor. They hold\
themselves to the highest standards of conduct, and\
some—for better or worse—hold the rest of the\
world to the same standards.\
Many who swear this oath are devoted to gods\
of law and good and use their gods’ tenets as the\
measure of personal devotion. Others hold angels as\
their ideals and incorporate images of angelic wings\
into their helmets or coats of arms.\
These paladins share the following tenets:\
•    Let your word be your promise.\
•    Protect the weak and never fear to act.\
•    Let your honorable deeds be an example.\
