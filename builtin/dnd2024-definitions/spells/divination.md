---
splat_schema: 1
splat_id: 14c9bb02-561e-58c7-9632-ca9ced09ac95
splat_kind: spell
splat_key: dnd2024:spells/divination
splat_name: Divination
splat_origin:
  key: dnd2024:reference/spell/divination
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: divination
  classes:
  - cleric
  - druid
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action or Ritual
  ritual: true
  range:
    mode: self
  material:
    description: incense worth 25+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/divination
  implementation: dnd2024:spell_divination
  parameters:
    question:
      maximum: 1
      subjects:
      - specific_goal
      - event
      - activity
      future_limit_days: 7
    reply:
      provider: gm
      truthful: true
      form_choices:
      - short_phrase
      - cryptic_rhyme
      accounts_for_later_changed_circumstances: false
    repeat_cast:
      reset: caster_long_rest_completed
      no_answer_chance_percent_per_cast_after_first: 25
      cumulative: true
splat_contracts:
- id: dnd2024:spell_divination
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
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
  - rest.completed
  capabilities:
  - spellcasting
  - effects
  - knowledge
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5b6d49a14e9b4583c44b58688fe24439db84016f43582f30a5bde0b1ad97ddac
  source_path: builtin/srd-5.2.1/spell/divination.md
  pages:
  - 125
  engine_gaps: &id001
  - dnd2024:spell_divination
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: fc8ad0188f090072e262c4051c80d0ee3c1e830af7412ce4c0389745d474dd44
splat_clauses:
- key: source_rules
  text: '# Divination Level 4 Divination (Cleric, Druid, Wizard) Casting Time: Action or Ritual Range:
    Self Components: V, S, M (incense worth 25+ GP, which the spell consumes) Duration: Instantaneous
    This spell puts you in contact with a god or a god’s servants. You ask one question about a specific
    goal, event, or activity to occur within 7 days. The GM offers a truthful reply, which might be a
    short phrase or cryptic rhyme. The spell doesn’t account for circumstances that might change the answer,
    such as the casting of other spells. If you cast the spell more than once before finishing a Long
    Rest, there is a cumulative 25 percent chance for each casting after the first that you get no answer.'
  sha256: 265868ab8e3c98cca1de74c08f5f55dbe759f90a7c25d96a53eb327062f58412
  status: contract_specified
splat_requires_mechanics: *id001
---

# Divination

<!-- SRD page 125 -->

Level 4 Divination (Cleric, Druid, Wizard)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S, M (incense worth 25+ GP, which the\
spell consumes)\
Duration: Instantaneous\
This spell puts you in contact with a god or a god’s\
servants. You ask one question about a specific goal,\
event, or activity to occur within 7 days. The GM of-\
fers a truthful reply, which might be a short phrase\
or cryptic rhyme. The spell doesn’t account for cir-\
cumstances that might change the answer, such as\
the casting of other spells.\
If you cast the spell more than once before finish-\
ing a Long Rest, there is a cumulative 25 percent\
chance for each casting after the first that you get\
no answer.\

Original layout: [[../../srd-5.2.1/pages/125|page 125]].
