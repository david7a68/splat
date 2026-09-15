---
splat_schema: 1
splat_id: 200e5608-4d85-5036-b8e6-c0b00009f9ce
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_14_overchannel
splat_name: 'Level 14: Overchannel'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/evoker_level_14_overchannel
  parameters:
    trigger: actor.casts_damaging_wizard_spell
    requires_slot_level:
    - 1
    - 2
    - 3
    - 4
    - 5
    optional: true
    spell_damage: maximum
    spell_damage_scope: cast_turn
    usage_counter:
      id: wizard.overchannel
      increment_each_use: 1
      reset: rest.long_completed
    backlash:
      when_use_number_minimum: 2
      timing: immediately_after_cast
      damage:
        dice:
          count:
            op: multiply
            args:
            - var: slot.level
            - var: overchannel.use_number
          sides: 12
        type: necrotic
      ignores:
      - resistance
      - immunity
    class_scope:
      class: wizard
      minimum_level: 14
  implementation: dnd2024:class_features_evoker_level_14_overchannel
splat_contracts:
- id: dnd2024:class_features_evoker_level_14_overchannel
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
  - spell.cast
  - spell.damage
  - rest.long_completed
  capabilities:
  - character_features
  - spellcasting
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 82
  engine_gaps: &id001
  - dnd2024:class_features_evoker_level_14_overchannel
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 8b612750dcdfd22b09b34a31c7b83eb72d0256fe7c5685f082aa572c50efdef7
  parent_name: Evoker
  line_span:
    first: 7046
    count: 13
splat_clauses:
- key: source_rules
  text: '# Level 14: Overchannel You can increase the power of your spells. When you cast a Wizard spell
    with a spell slot of levels 1–5 that deals damage, you can deal maximum damage with that spell on
    the turn you cast it. The first time you do so, you suffer no adverse effect. If you use this feature
    again before you finish a Long Rest, you take 2d12 Necrotic damage for each level of the spell slot
    immediately after you cast it. This damage ignores Resistance and Immunity. Each time you use this
    feature again before finishing a Long Rest, the Necrotic damage per spell level increases by 1d12.'
  sha256: dbef09a0f27f00665528d4d306e2f36e5fac6a35296909b10d8a62c5c86df34a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Overchannel

<!-- SRD page 82 -->

You can increase the power of your spells. When\
you cast a Wizard spell with a spell slot of levels 1–5\
that deals damage, you can deal maximum damage\
with that spell on the turn you cast it.\
The first time you do so, you suffer no adverse ef-\
fect. If you use this feature again before you finish a\
Long Rest, you take 2d12 Necrotic damage for each\
level of the spell slot immediately after you cast it.\
This damage ignores Resistance and Immunity.\
Each time you use this feature again before finish-\
ing a Long Rest, the Necrotic damage per spell level\
increases by 1d12.\
