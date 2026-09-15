---
splat_schema: 1
splat_id: 9119be78-3637-5b22-a703-f1ddd9209758
splat_kind: spell
splat_key: dnd2024:spells/illusory_script
splat_name: Illusory Script
splat_origin:
  key: dnd2024:reference/spell/illusory_script
splat_sources:
- title: SRD 5.2.1
  locator: pp. 142
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: illusion
  classes:
  - bard
  - warlock
  - wizard
  components:
  - somatic
  - material
  concentration: false
  duration: 10 days
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    mode: touch
  material:
    description: ink worth 10+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/illusory_script
  implementation: dnd2024:spell_illusory_script
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 864000
    target:
      kind: writing_surface
      suitable_for_writing: true
      touch: true
    message:
      real_meaning: caster_authored
      real_handwriting: caster
    authorized_readers:
      caster: true
      additional_creatures:
        chooser: caster
        timing: cast
    unauthorized_display_choice:
      chooser: caster
      options:
      - unintelligible_script
      - alternative_text
    unintelligible_script:
      appearance_choices:
      - unknown_script
      - magical_script
      readable: false
    alternative_text:
      meaning: caster_choice
      handwriting: caster_choice
      language:
        chooser: caster
        constraint: known_to_caster
    truesight:
      can_read_hidden_message: true
    on_dispel:
      remove:
      - original_script
      - illusion
splat_contracts:
- id: dnd2024:spell_illusory_script
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    surface:
      type: object
      required: true
    authorized_readers:
      type: participant_list
      required: true
    real_message:
      type: text
      required: true
    illusory_display:
      type: choice
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - illusions
  - memory
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: fe364cddefde63760aeec37e92f95faae0d03af8760f826d62e5b4b952d528c5
  source_path: builtin/srd-5.2.1/spell/illusory_script.md
  pages:
  - 142
  engine_gaps: &id001
  - dnd2024:spell_illusory_script
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 3df6891c7a112221c830cf3960004dd8139fb80f2793ebc64c02b538abb6722f
splat_clauses:
- key: source_rules
  text: '# Illusory Script Level 1 Illusion (Bard, Warlock, Wizard) Casting Time: 1 minute or Ritual Range:
    Touch Components: S, M (ink worth 10+ GP, which the spell consumes) Duration: 10 days You write on
    parchment, paper, or another suitable material and imbue it with an illusion that lasts for the duration.
    To you and any creatures you designate when you cast the spell, the writing appears normal, seems
    to be written in your hand, and conveys whatever meaning you intended when you wrote the text. To
    all others, the writing appears as if it were written in an unknown or magical script that is unintelligible.
    Alternatively, the illusion can alter the meaning, handwriting, and language of the text, though the
    language must be one you know. If the spell is dispelled, the original script and the illusion both
    disappear. A creature that has Truesight can read the hidden message.'
  sha256: 53e1da148b1cfd030e17de5f4b8dc2d6a8cb5afb32ebc1832c0531c563bcce04
  status: contract_specified
splat_requires_mechanics: *id001
---

# Illusory Script

<!-- SRD page 142 -->

Level 1 Illusion (Bard, Warlock, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Touch\
Components: S, M (ink worth 10+ GP, which the spell\
consumes)\
Duration: 10 days\
You write on parchment, paper, or another suitable\
material and imbue it with an illusion that lasts for\
the duration. To you and any creatures you desig-\
nate when you cast the spell, the writing appears\
normal, seems to be written in your hand, and\
conveys whatever meaning you intended when you\
wrote the text. To all others, the writing appears as\
if it were written in an unknown or magical script\
that is unintelligible. Alternatively, the illusion can\
alter the meaning, handwriting, and language of the\
text, though the language must be one you know.\
If the spell is dispelled, the original script and the\
illusion both disappear.\
A creature that has Truesight can read the hidden\
message.\

Original layout: [[../../srd-5.2.1/pages/142|page 142]].
