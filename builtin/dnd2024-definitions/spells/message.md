---
splat_schema: 1
splat_id: a118c863-9c91-5fb9-8fcb-36511d325bb3
splat_kind: spell
splat_key: dnd2024:spells/message
splat_name: Message
splat_origin:
  key: dnd2024:reference/spell/message
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148, 149
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: transmutation
  classes:
  - bard
  - druid
  - sorcerer
  - wizard
  components:
  - somatic
  - material
  concentration: false
  duration: 1 round
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: a copper wire
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/message
  implementation: dnd2024:spell_message
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
    communication:
      point_toward_target: true
      caster_whispers: true
      message_heard_only_by: target
      reply_optional: true
      reply_whispered: true
      reply_heard_only_by: caster
    through_solid_objects:
      requires:
      - caster_familiar_with_target
      - caster_knows_target_beyond_barrier
      blocked_by:
      - kind: magical_silence
      - materials:
        - stone
        - metal
        - wood
        minimum_thickness_feet: 1
      - material: lead
        any_thin_sheet: true
splat_contracts:
- id: dnd2024:spell_message
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
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    message:
      required: false
      type: text
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - knowledge
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - knowledge
splat_conversion:
  status: specified
  category: spell
  source_sha256: 7db17ae8a0c5a1974efebed70351cbfdf6e55c3f5e9f0d23af2926c53e8afdf9
  source_path: builtin/srd-5.2.1/spell/message.md
  pages:
  - 148
  - 149
  engine_gaps: &id001
  - dnd2024:spell_message
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 53ec6f002a3a25df69e49d1405f36ab0df209bf4767d8ee124caa57f099e3c18
splat_clauses:
- key: source_rules
  text: '# Message Transmutation Cantrip (Bard, Druid, Sorcerer, Wizard) Casting Time: Action Range: 120
    feet Components: S, M (a copper wire) Duration: 1 round You point toward a creature within range and
    whisper a message. The target (and only the target) hears the message and can reply in a whisper that
    only you can hear. You can cast this spell through solid objects if you are familiar with the target
    and know it is beyond the barrier. Magical silence; 1 foot of stone, metal, or wood; or a thin sheet
    of lead blocks the spell.'
  sha256: bd61636cbfca6ba9918632f8a5d4d4e86982db14c90a9741231e0abff86ad613
  status: contract_specified
splat_requires_mechanics: *id001
---

# Message

<!-- SRD page 148 -->

Transmutation Cantrip (Bard, Druid, Sorcerer,\
Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: S, M (a copper wire)\
Duration: 1 round\

<!-- SRD page 149 -->

You point toward a creature within range and\
whisper a message. The target (and only the target)\
hears the message and can reply in a whisper that\
only you can hear.\
You can cast this spell through solid objects if you\
are familiar with the target and know it is beyond\
the barrier. Magical silence; 1 foot of stone, metal,\
or wood; or a thin sheet of lead blocks the spell.\

Original layout: [[../../srd-5.2.1/pages/148|page 148]], [[../../srd-5.2.1/pages/149|page 149]].
