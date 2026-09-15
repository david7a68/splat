---
splat_schema: 1
splat_id: 28a9289f-6427-5a51-81b0-e6bab514e6cd
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_18_dragon_companion
splat_name: 'Level 18: Dragon Companion'
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/draconic_sorcery_level_18_dragon_companion
  parameters:
    spell:
      key: dnd2024:spells/summon_dragon
    material_component_required: false
    free_cast:
      spell_slot_cost: 0
      uses:
        maximum: 1
        recover:
          event: rest.long_rest_completed
          amount: all
    optional_cast_modification:
      timing: casting_started
      requires_concentration: false
      duration:
        minutes: 1
  implementation: dnd2024:class_features_draconic_sorcery_level_18_dragon_companion
splat_contracts:
- id: dnd2024:class_features_draconic_sorcery_level_18_dragon_companion
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  - spell_grants
  events:
  - spell.cast_started
  capabilities:
  - spellcasting
  - concentration
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 70
  engine_gaps: &id001
  - dnd2024:class_features_draconic_sorcery_level_18_dragon_companion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: ba4381f4f00b4caa5ef44523c55f38a7c8e67fb3991629caa712d9fb11e5396f
  parent_name: Draconic Sorcery
  line_span:
    first: 5924
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 18: Dragon Companion You can cast Summon Dragon without a Material component. You can
    also cast it once without a spell slot, and you regain the ability to cast it in this way when you
    finish a Long Rest. Whenever you start casting the spell, you can modify it so that it doesn’t require
    Concentration. If you do so, the spell’s duration becomes 1 minute for that casting.'
  sha256: c91371aa1beff3467b1909bce8e33a19bb5f01628c6ebd390c9d61d9a6ee9c60
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Dragon Companion

<!-- SRD page 70 -->

You can cast Summon Dragon without a Material\
component. You can also cast it once without a spell\
slot, and you regain the ability to cast it in this way\
when you finish a Long Rest.\
Whenever you start casting the spell, you can\
modify it so that it doesn’t require Concentration. If\
you do so, the spell’s duration becomes 1 minute for\
that casting.\
