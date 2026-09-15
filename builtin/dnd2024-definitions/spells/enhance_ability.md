---
splat_schema: 1
splat_id: fdda293d-3ffe-5a92-8eb6-9e7c15474c40
splat_kind: spell
splat_key: dnd2024:spells/enhance_ability
splat_name: Enhance Ability
splat_origin:
  key: dnd2024:reference/spell/enhance_ability
splat_sources:
- title: SRD 5.2.1
  locator: pp. 127
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - bard
  - cleric
  - druid
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: fur or a feather
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/enhance_ability
  implementation: dnd2024:spell_enhance_ability
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 3600
    targets:
      kind: creature
      touch: true
      count:
        base: 1
        increment_per_slot_above: 1
        base_slot: 2
    ability:
      chooser: caster
      per_target: true
      options:
      - strength
      - dexterity
      - intelligence
      - wisdom
      - charisma
    modifier:
      selector: ability_checks_using_chosen_ability
      advantage: true
splat_contracts:
- id: dnd2024:spell_enhance_ability
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    ability_choices:
      type: choice
      required: true
  outputs:
  - effects
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 25373f5f446f9e2560425eab8c09672c41922160b3102a9a6e03e2432a67dfca
  source_path: builtin/srd-5.2.1/spell/enhance_ability.md
  pages:
  - 127
  engine_gaps: &id001
  - dnd2024:spell_enhance_ability
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 335448fa92904066cc5e8126ae4d919fef96799a74d7a0e246a2683cd886a1a8
splat_clauses:
- key: source_rules
  text: '# Enhance Ability Level 2 Transmutation (Bard, Cleric, Druid, Ranger, Sorcerer, Wizard) Casting
    Time: Action Range: Touch Components: V, S, M (fur or a feather) Duration: Concentration, up to 1
    hour You touch a creature and choose Strength, Dexterity, Intelligence, Wisdom, or Charisma. For the
    duration, the target has Advantage on ability checks using the chosen ability. Using a Higher-Level
    Spell Slot. You can target one additional creature for each spell slot level above 2. You can choose
    a different ability for each target.'
  sha256: 2d7ce003626962a00eabb4de3a79d96e28fcc5af5798a0551fe41502627a8d49
  status: contract_specified
splat_requires_mechanics: *id001
---

# Enhance Ability

<!-- SRD page 127 -->

Level 2 Transmutation (Bard, Cleric, Druid, Ranger,\
Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (fur or a feather)\
Duration: Concentration, up to 1 hour\
You touch a creature and choose Strength, Dex-\
terity, Intelligence, Wisdom, or Charisma. For the\
duration, the target has Advantage on ability checks\
using the chosen ability.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 2. You can choose a different ability for each\
target.\

Original layout: [[../../srd-5.2.1/pages/127|page 127]].
