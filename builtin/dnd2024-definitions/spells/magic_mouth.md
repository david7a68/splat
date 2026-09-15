---
splat_schema: 1
splat_id: e15d7476-d650-5b00-a02a-e6254044cf76
splat_kind: spell
splat_key: dnd2024:spells/magic_mouth
splat_name: Magic Mouth
splat_origin:
  key: dnd2024:reference/spell/magic_mouth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 146
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: illusion
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    feet: 30
  material:
    description: jade dust worth 10+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/magic_mouth
  implementation: dnd2024:spell_magic_mouth
  parameters:
    spell_lifetime:
      concentration: false
      until: dispelled_or_source_specific_end
    target:
      kind: object
      visible: true
      range_feet: 30
      worn_or_carried_by_another: false
    message:
      spoken_at_cast: true
      maximum_words: 25
      maximum_delivery_seconds: 600
      playback_voice: caster_voice_at_cast
      playback_volume: spoken_volume_at_cast
    trigger:
      chooser: caster
      timing: cast
      allowed_modalities:
      - visual
      - audible
      maximum_distance_from_object_feet: 30
      condition_resolution: supplied_trigger_predicate
    delivery:
      event: declared_trigger.occurred
      mouth_appears: true
      mouth_location: existing_mouth_or_mouthlike_feature_if_present
    recurrence_choice:
      chooser: caster
      timing: cast
      options:
      - end_after_delivery
      - repeat_on_each_trigger
splat_contracts:
- id: dnd2024:spell_magic_mouth
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target:
      type: object
      required: true
    message:
      type: text
      required: true
    trigger:
      type: choice
      required: true
    recurrence:
      type: choice
      required: true
  outputs:
  - effects
  events:
  - declared_trigger.occurred
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - illusions
splat_conversion:
  status: specified
  category: spell
  source_sha256: 911fc9a944050fdc8a1e9df06cf4184f67fd332de8b3921bbb32a7e938263b80
  source_path: builtin/srd-5.2.1/spell/magic_mouth.md
  pages:
  - 146
  engine_gaps: &id001
  - dnd2024:spell_magic_mouth
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 4e08705f1610eab72593ba1015dc1a91b615a2f019e0f269ce2cb39894eff2d0
splat_clauses:
- key: source_rules
  text: '# Magic Mouth Level 2 Illusion (Bard, Wizard) Casting Time: 1 minute or Ritual Range: 30 feet
    Components: V, S, M (jade dust worth 10+ GP, which the spell consumes) Duration: Until dispelled You
    implant a message within an object in range—a message that is uttered when a trigger condition is
    met. Choose an object that you can see and that isn’t being worn or carried by another creature. Then
    speak the message, which must be 25 words or fewer, though it can be delivered over as long as 10
    minutes. Finally, determine the circumstance that will trigger the spell to deliver your message.
    When that trigger occurs, a magical mouth appears on the object and recites the message in your voice
    and at the same volume you spoke. If the object you chose has a mouth or something that looks like
    a mouth (for example, the mouth of a statue), the magical mouth appears there, so the words appear
    to come from the object’s mouth. When you cast this spell, you can have the spell end after it delivers
    its message, or it can remain and repeat its message whenever the trigger occurs. The trigger can
    be as general or as detailed as you like, though it must be based on visual or audible conditions
    that occur within 30 feet of the object. For example, you could instruct the mouth to speak when any
    creature moves within 30 feet of the object or when a silver bell rings within 30 feet of it.'
  sha256: 3537b2bd94fb92dddf0e2433a434f574109b02fb1266dc0539718970c2465c33
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magic Mouth

<!-- SRD page 146 -->

Level 2 Illusion (Bard, Wizard)\
Casting Time: 1 minute or Ritual\
Range: 30 feet\
Components: V, S, M (jade dust worth 10+ GP, which\
the spell consumes)\
Duration: Until dispelled\
You implant a message within an object in range—a\
message that is uttered when a trigger condition is\
met. Choose an object that you can see and that isn’t\
being worn or carried by another creature. Then\
speak the message, which must be 25 words or\
fewer, though it can be delivered over as long as 10\
minutes. Finally, determine the circumstance that\
will trigger the spell to deliver your message.\
When that trigger occurs, a magical mouth ap-\
pears on the object and recites the message in your\
voice and at the same volume you spoke. If the ob-\
ject you chose has a mouth or something that looks\
like a mouth (for example, the mouth of a statue),\
the magical mouth appears there, so the words\
appear to come from the object’s mouth. When you\
cast this spell, you can have the spell end after it\
delivers its message, or it can remain and repeat its\
message whenever the trigger occurs.\
The trigger can be as general or as detailed as you\
like, though it must be based on visual or audible\
conditions that occur within 30 feet of the object.\
For example, you could instruct the mouth to speak\
when any creature moves within 30 feet of the ob-\
ject or when a silver bell rings within 30 feet of it.\

Original layout: [[../../srd-5.2.1/pages/146|page 146]].
