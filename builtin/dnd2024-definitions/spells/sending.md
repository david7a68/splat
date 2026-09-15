---
splat_schema: 1
splat_id: ce7e7bb7-5224-5d68-a9ff-1501a5266ca0
splat_kind: spell
splat_key: dnd2024:spells/sending
splat_name: Sending
splat_origin:
  key: dnd2024:reference/spell/sending
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: divination
  classes:
  - bard
  - cleric
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: unlimited
  material:
    description: a copper wire
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/sending
  implementation: dnd2024:spell_sending
  parameters:
    targets:
      kind: creature
      eligibility_any:
      - caster_has_met
      - described_by_someone_who_has_met
      distance: unlimited
      same_plane_required: false
    message:
      word_limit: 25
      channel: mental
      recognize_sender_if_target_knows_caster: true
      understands_meaning: true
      reply:
        optional: true
        timing: immediate
        same_manner: true
        word_limit: 25
    cross_plane:
      delivery_failure_chance_percent: 5
      caster_knows_failure: true
    block:
      choice: recipient_after_receiving_message
      duration_hours: 8
      scope: same_caster_sending
      attempt_while_blocked:
        spell_fails: true
        caster_learns_blocked: true
splat_contracts:
- id: dnd2024:spell_sending
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
  - time.elapsed
  capabilities:
  - spellcasting
  - effects
  - knowledge
splat_conversion:
  status: specified
  category: spell
  source_sha256: 6bf1e92f8d1055771ddcf08e617e3a558a299f19f28bc169f18b49fb49b3ef05
  source_path: builtin/srd-5.2.1/spell/sending.md
  pages:
  - 161
  engine_gaps: &id001
  - dnd2024:spell_sending
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 0df9c65903daeb5d457610f9ff83f4ce0b3f78fccf77af5483aea08278819f76
splat_clauses:
- key: source_rules
  text: '# Sending Level 3 Divination (Bard, Cleric, Wizard) Casting Time: Action Range: Unlimited Components:
    V, S, M (a copper wire) Duration: Instantaneous You send a short message of 25 words or fewer to a
    creature you have met or a creature described to you by someone who has met it. The target hears the
    message in its mind, recognizes you as the sender if it knows you, and can answer in a like manner
    immediately. The spell enables targets to understand the meaning of your message. You can send the
    message across any distance and even to other planes of existence, but if the target is on a different
    plane than you, there is a 5 percent chance that the message doesn’t arrive. You know if the delivery
    fails. Upon receiving your message, a creature can block your ability to reach it again with this
    spell for 8 hours. If you try to send another message during that time, you learn that you are blocked,
    and the spell fails.'
  sha256: 433025992f28a62a70651c04ff6eb258f4b307fa402434facf961db3395569bc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sending

<!-- SRD page 161 -->

Level 3 Divination (Bard, Cleric, Wizard)\
Casting Time: Action\
Range: Unlimited\
Components: V, S, M (a copper wire)\
Duration: Instantaneous\
You send a short message of 25 words or fewer to\
a creature you have met or a creature described to\
you by someone who has met it. The target hears\
the message in its mind, recognizes you as the\
sender if it knows you, and can answer in a like\
manner immediately. The spell enables targets to\
understand the meaning of your message.\
You can send the message across any distance and\
even to other planes of existence, but if the target\
is on a different plane than you, there is a 5 percent\
chance that the message doesn’t arrive. You know if\
the delivery fails.\
Upon receiving your message, a creature can\
block your ability to reach it again with this spell for\
8 hours. If you try to send another message during\
that time, you learn that you are blocked, and the\
spell fails.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]].
