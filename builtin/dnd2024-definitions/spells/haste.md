---
splat_schema: 1
splat_id: 4402731c-08cc-5dd5-aed6-8f409703fdcc
splat_kind: spell
splat_key: dnd2024:spells/haste
splat_name: Haste
splat_origin:
  key: dnd2024:reference/spell/haste
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a shaving of licorice root
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/haste
  implementation: dnd2024:spell_haste
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      visible: true
      willing: true
    benefits:
      duration: spell_duration
      speed_multiplier: 2
      ac_bonus: 2
      saving_throw_advantage:
      - dexterity
      additional_action_each_target_turn:
        count: 1
        allowed:
        - attack
        - dash
        - disengage
        - hide
        - utilize
        attack_count_maximum: 1
    on_end:
      condition: incapacitated
      speed: 0
      until: target_next_turn_end
splat_contracts:
- id: dnd2024:spell_haste
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
    choices:
      required: false
      type: choice
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - spell.cast_completed
  - attack.roll
  - attack.hit
  - effect.ended
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - conditions
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: 9326b26e25ca456c2d83d64be70cc9e158fa3f72b633f1a83e25a42c42fd694e
  source_path: builtin/srd-5.2.1/spell/haste.md
  pages:
  - 139
  engine_gaps: &id001
  - dnd2024:spell_haste
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 45ffcd3e509e658ec1f8df75dd65385cf202763e171766f9c94bd2c69d7ac4e0
splat_clauses:
- key: source_rules
  text: '# Haste Level 3 Transmutation (Sorcerer, Wizard) Casting Time: Action Range: 30 feet Components:
    V, S, M (a shaving of licorice root) Duration: Concentration, up to 1 minute Choose a willing creature
    that you can see within range. Until the spell ends, the target’s Speed is doubled, it gains a +2
    bonus to Armor Class, it has Advantage on Dexterity saving throws, and it gains an additional action
    on each of its turns. That action can be used to take only the Attack (one attack only), Dash, Disengage,
    Hide, or Utilize action. When the spell ends, the target is Incapacitated and has a Speed of 0 until
    the end of its next turn, as a wave of lethargy washes over it.'
  sha256: 0e6b32c5ce68f77e2be6236c67f9afd20a6c9810f9c8c801c75d986f2a5c3d3b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Haste

<!-- SRD page 139 -->

Level 3 Transmutation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a shaving of licorice root)\
Duration: Concentration, up to 1 minute\
Choose a willing creature that you can see within\
range. Until the spell ends, the target’s Speed is\
doubled, it gains a +2 bonus to Armor Class, it has\
Advantage on Dexterity saving throws, and it gains\
an additional action on each of its turns. That ac-\
tion can be used to take only the Attack (one attack\
only), Dash, Disengage, Hide, or Utilize action.\
When the spell ends, the target is Incapacitated\
and has a Speed of 0 until the end of its next turn, as\
a wave of lethargy washes over it.\

Original layout: [[../../srd-5.2.1/pages/139|page 139]].
