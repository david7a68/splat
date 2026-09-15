---
splat_schema: 1
splat_id: ab4d6e06-0f81-5659-80d3-47c3b45a54ef
splat_kind: spell
splat_key: dnd2024:spells/contact_other_plane
splat_name: Contact Other Plane
splat_origin:
  key: dnd2024:reference/spell/contact_other_plane
splat_sources:
- title: SRD 5.2.1
  locator: pp. 119
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: divination
  classes:
  - warlock
  - wizard
  components:
  - verbal
  concentration: false
  duration: 1 minute
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/contact_other_plane
  implementation: dnd2024:spell_contact_other_plane
  parameters:
    targeting:
      kind: self
      contact: knowledgeable_extraplanar_entity
    save:
      ability: intelligence
      dc: 15
      success:
        questions_max: 5
        deadline: spell_end
        responder: gm
        answer_words: 1
        unknown_answer: unclear
        misleading_one_word: short_phrase_allowed
      failure:
        damage:
          dice: 6d6
          type: psychic
        condition: incapacitated
        ends_on:
        - long_rest_finished
        - greater_restoration_applied
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_contact_other_plane
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
    campaign:
      type: campaign
      required: true
    questions:
      type: text
      required: true
  outputs:
  - effects
  - damage_results
  - information
  events:
  - spell.cast_completed
  - rest.completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
  - campaign_state
  - rest_lifecycle
splat_conversion:
  status: specified
  category: spell
  source_sha256: 8774bc9cedfb8d4c1b2b802fc697575d3b92b7fa3c94103b67a528dc6365ea0b
  source_path: builtin/srd-5.2.1/spell/contact_other_plane.md
  pages:
  - 119
  engine_gaps: &id001
  - dnd2024:spell_contact_other_plane
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 90e381c908e2fdc5f8faff10bb98cf6d276639d41ab509c4825f23f2158e5a22
splat_clauses:
- key: source_rules
  text: '# Contact Other Plane Level 5 Divination (Warlock, Wizard) Casting Time: 1 minute or Ritual Range:
    Self Components: V Duration: 1 minute You mentally contact a demigod, the spirit of a longdead sage,
    or some other knowledgeable entity from another plane. Contacting this otherworldly intelligence can
    break your mind. When you cast this spell, make a DC 15 Intelligence saving throw. On a successful
    save, you can ask the entity up to five questions. You must ask your questions before the spell ends.
    The GM answers each question with one word, such as “yes,” “no,” “maybe,” “never,” “irrelevant,” or
    “unclear” (if the entity doesn’t know the answer to the question). If a one-word answer would be misleading,
    the GM might instead offer a short phrase as an answer. On a failed save, you take 6d6 Psychic damage
    and have the Incapacitated condition until you finish a Long Rest. A Greater Restoration spell cast
    on you ends this effect.'
  sha256: 50002e91585acb526958264057ec44d27c8077add107b0225cc49115049c21cf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Contact Other Plane

<!-- SRD page 119 -->

Level 5 Divination (Warlock, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V\
Duration: 1 minute\
You mentally contact a demigod, the spirit of a long-\
dead sage, or some other knowledgeable entity\
from another plane. Contacting this otherworldly\
intelligence can break your mind. When you cast\
this spell, make a DC 15 Intelligence saving throw.\
On a successful save, you can ask the entity up to\
five questions. You must ask your questions before\
the spell ends. The GM answers each question with\
one word, such as “yes,” “no,” “maybe,” “never,” “ir-\
relevant,” or “unclear” (if the entity doesn’t know\
the answer to the question). If a one-word answer\
would be misleading, the GM might instead offer a\
short phrase as an answer.\
On a failed save, you take 6d6 Psychic damage and\
have the Incapacitated condition until you finish a\
Long Rest. A Greater Restoration spell cast on you\
ends this effect.\

Original layout: [[../../srd-5.2.1/pages/119|page 119]].
