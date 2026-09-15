---
splat_schema: 1
splat_id: be9423d1-2ca8-544d-b016-3c882f52a1b1
splat_kind: spell
splat_key: dnd2024:spells/speak_with_dead
splat_name: Speak with Dead
splat_origin:
  key: dnd2024:reference/spell/speak_with_dead
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: necromancy
  classes:
  - bard
  - cleric
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 10
  material:
    description: burning incense
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/speak_with_dead
  implementation: dnd2024:spell_speak_with_dead
  parameters:
    target:
      kind: corpse
      count: 1
      visible_to_caster: false
      mouth_required: true
      undead_when_died: false
      no_same_spell_target_within_days: 10
    questions:
      maximum: 5
      knowledge: in_life_only
      languages: in_life_only
      answer_style:
      - brief
      - cryptic
      - repetitive
      truth_compulsion_if_caster_antagonistic_or_known_enemy: false
      learn_new_information: false
      comprehend_events_after_death: false
      speculate_future: false
    animation:
      returns_soul: false
      animating_spirit_only: true
splat_contracts:
- id: dnd2024:spell_speak_with_dead
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - animated_corpse
  - questions_remaining
  - answers
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - forms
  - summoning
  - time
  - knowledge
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2c70cbe35be1e5bef34ba27840b9a47e5ff4047be7ba7bc9df3eff1d7891b1cd
  source_path: builtin/srd-5.2.1/spell/speak_with_dead.md
  pages:
  - 164
  engine_gaps: &id001
  - dnd2024:spell_speak_with_dead
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 7d2357f780e78a47036ba397e7c389d83cfb672ed13e88bb5086835787560e9d
splat_clauses:
- key: source_rules
  text: '# Speak with Dead Level 3 Necromancy (Bard, Cleric, Wizard) Casting Time: Action Range: 10 feet
    Components: V, S, M (burning incense) Duration: 10 minutes You grant the semblance of life to a corpse
    of your choice within range, allowing it to answer questions you pose. The corpse must have a mouth,
    and this spell fails if the deceased creature was Undead when it died. The spell also fails if the
    corpse was the target of this spell within the past 10 days. Until the spell ends, you can ask the
    corpse up to five questions. The corpse knows only what it knew in life, including the languages it
    knew. Answers are usually brief, cryptic, or repetitive, and the corpse is under no compulsion to
    offer a truthful answer if you are antagonistic toward it or it recognizes you as an enemy. This spell
    doesn’t return the creature’s soul to its body, only its animating spirit. Thus, the corpse can’t
    learn new information, doesn’t comprehend anything that has happened since it died, and can’t speculate
    about future events.'
  sha256: f70bf4c2f90d0714ab7f8a23aaebf96b1dde3061a46401a7e3dd8ba3075fe54a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Speak with Dead

<!-- SRD page 164 -->

Level 3 Necromancy (Bard, Cleric, Wizard)\
Casting Time: Action\
Range: 10 feet\
Components: V, S, M (burning incense)\
Duration: 10 minutes\
You grant the semblance of life to a corpse of your\
choice within range, allowing it to answer ques-\
tions you pose. The corpse must have a mouth, and\
this spell fails if the deceased creature was Undead\
when it died. The spell also fails if the corpse was\
the target of this spell within the past 10 days.\
Until the spell ends, you can ask the corpse up to\
five questions. The corpse knows only what it knew\
in life, including the languages it knew. Answers are\
usually brief, cryptic, or repetitive, and the corpse\
is under no compulsion to offer a truthful answer if\
you are antagonistic toward it or it recognizes you\
as an enemy. This spell doesn’t return the creature’s\
soul to its body, only its animating spirit. Thus, the\
corpse can’t learn new information, doesn’t compre-\
hend anything that has happened since it died, and\
can’t speculate about future events.\

Original layout: [[../../srd-5.2.1/pages/164|page 164]].
