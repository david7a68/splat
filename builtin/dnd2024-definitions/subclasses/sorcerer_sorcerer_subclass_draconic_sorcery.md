---
splat_schema: 1
splat_id: 850bcc82-f5d2-5fd2-bb97-a39bc776c3c4
splat_kind: rule_group
splat_key: dnd2024:subclasses/sorcerer_sorcerer_subclass_draconic_sorcery
splat_name: 'Sorcerer Subclass: Draconic Sorcery'
splat_origin:
  key: dnd2024:reference/pages/069
splat_sources:
- title: SRD 5.2.1
  locator: pp. 69
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: subclass
splat_rule:
  mechanic: dnd2024:subclasses/sorcerer_sorcerer_subclass_draconic_sorcery
  implementation: dnd2024:subclasses_sorcerer_sorcerer_subclass_draconic_sorcery
  parameters:
    class: sorcerer
    minimum_class_level: 3
    feature_grants:
    - class_level: 14
      definition:
        key: dnd2024:class_features/draconic_sorcery_level_14_dragon_wings
    - class_level: 18
      definition:
        key: dnd2024:class_features/draconic_sorcery_level_18_dragon_companion
    - class_level: 3
      definition:
        key: dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience
    - class_level: 3
      definition:
        key: dnd2024:class_features/draconic_sorcery_level_3_draconic_spells
    - class_level: 6
      definition:
        key: dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity
    grant_eligible_features:
      'on':
      - subclass.selected
      - character.level_gained
      comparison: class_level_at_least_threshold
      persistent: true
splat_contracts:
- id: dnd2024:subclasses_sorcerer_sorcerer_subclass_draconic_sorcery
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
splat_conversion:
  status: specified
  category: subclass
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 69
  engine_gaps: &id001
  - dnd2024:subclasses_sorcerer_sorcerer_subclass_draconic_sorcery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 389857e8417d1d968ea8e41a51979b912b15a73731bdea0140d1b9fba9da6a3c
  parent_name: Sorcerer
  line_span:
    first: 5876
    count: 10
splat_clauses:
- key: source_rules
  text: '# Sorcerer Subclass: Draconic Sorcery Breathe the Magic of Dragons Your innate magic comes from
    the gift of a dragon. Perhaps an ancient dragon facing death bequeathed some of its magical power
    to you or your ancestor. You might have absorbed magic from a site infused with dragons’ power. Or
    perhaps you handled a treasure taken from a dragon’s hoard that was steeped in draconic power. Or
    you might have a dragon for an ancestor.'
  sha256: 1a58ad8dcf60375870817e7ada56fd543dcd529d9ad470f7ac65cec6bd5ceb42
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sorcerer Subclass: Draconic Sorcery

<!-- SRD page 69 -->

Breathe the Magic of Dragons\
Your innate magic comes from the gift of a dragon.\
Perhaps an ancient dragon facing death bequeathed\
some of its magical power to you or your ancestor.\
You might have absorbed magic from a site infused\
with dragons’ power. Or perhaps you handled a\
treasure taken from a dragon’s hoard that was\
steeped in draconic power. Or you might have a\
dragon for an ancestor.\
