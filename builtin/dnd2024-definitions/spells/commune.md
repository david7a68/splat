---
splat_schema: 1
splat_id: e417ecb8-8600-5893-a71a-34610f59657a
splat_kind: spell
splat_key: dnd2024:spells/commune
splat_name: Commune
splat_origin:
  key: dnd2024:reference/spell/commune
splat_sources:
- title: SRD 5.2.1
  locator: pp. 116
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: divination
  classes:
  - cleric
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 minute
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    mode: self
  material:
    description: incense
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/commune
  implementation: dnd2024:spell_commune
  parameters:
    targeting:
      kind: self
      contact: deity_or_divine_proxy
    questions:
      max: 3
      deadline: spell_end
      form: yes_or_no
      answers:
        correct: true
        beyond_entity_knowledge: unclear
        gm_short_phrase_allowed_when:
        - one_word_misleading
        - one_word_contrary_to_deity_interests
    repeated_casting:
      reset: long_rest_finished
      no_answer_chance_percent:
        base: 0
        additional_per_cast_after_first: 25
        maximum: 100
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_commune
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
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
  - information
  events:
  - spell.cast_completed
  - rest.completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - campaign_state
  - rest_lifecycle
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5b13916dfb59cfebe47f4c305f2b8dbdeecd2cf70e563c9f59b5c0bc8f145568
  source_path: builtin/srd-5.2.1/spell/commune.md
  pages:
  - 116
  engine_gaps: &id001
  - dnd2024:spell_commune
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 2c033477278a0fd79eced2c9cdd4609859332ffc9861a2a01a76799d93a461b4
splat_clauses:
- key: source_rules
  text: '# Commune Level 5 Divination (Cleric) Casting Time: 1 minute or Ritual Range: Self Components:
    V, S, M (incense) Duration: 1 minute You contact a deity or a divine proxy and ask up to three questions
    that can be answered with yes or no. You must ask your questions before the spell ends. You receive
    a correct answer for each question. Divine beings aren’t necessarily omniscient, so you might receive
    “unclear” as an answer if a question pertains to information that lies beyond the deity’s knowledge.
    In a case where a one-word answer could be misleading or contrary to the deity’s interests, the GM
    might offer a short phrase as an answer instead. If you cast the spell more than once before finishing
    a Long Rest, there is a cumulative 25 percent chance for each casting after the first that you get
    no answer.'
  sha256: 417a4b33a6a9b37ed894608ffe0d4070dff468ebb0628655754dba5ead375a2a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Commune

<!-- SRD page 116 -->

Level 5 Divination (Cleric)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V, S, M (incense)\
Duration: 1 minute\
You contact a deity or a divine proxy and ask up\
to three questions that can be answered with yes\
or no. You must ask your questions before the\
spell ends. You receive a correct answer for each\
question.\
Divine beings aren’t necessarily omniscient, so\
you might receive “unclear” as an answer if a ques-\
tion pertains to information that lies beyond the de-\
ity’s knowledge. In a case where a one-word answer\
could be misleading or contrary to the deity’s inter-\
ests, the GM might offer a short phrase as an answer\
instead.\
If you cast the spell more than once before finish-\
ing a Long Rest, there is a cumulative 25 percent\
chance for each casting after the first that you get\
no answer.\

Original layout: [[../../srd-5.2.1/pages/116|page 116]].
