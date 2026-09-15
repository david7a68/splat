---
splat_schema: 1
splat_id: 9851c2a4-c7cb-5db3-b080-a0ac719edce1
splat_kind: spell
splat_key: dnd2024:spells/legend_lore
splat_name: Legend Lore
splat_origin:
  key: dnd2024:reference/spell/legend_lore
splat_sources:
- title: SRD 5.2.1
  locator: pp. 144
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
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
  casting_time: 10 minutes
  ritual: false
  range:
    mode: self
  material:
    description: incense worth 250+ GP, which the spell consumes, and four ivory strips worth 50+ GP each
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/legend_lore
  implementation: dnd2024:spell_legend_lore
  parameters:
    targeting:
      kind: named_or_described_subject
      categories:
      - person
      - place
      - object
      requires: famous
    lore:
      recipient: caster_mind
      responder: gm
      content: significant_lore_summary
      eligible_information:
      - important_details
      - amusing_revelations
      - secret_lore
      accuracy: true
      precision_increases_with: prior_subject_knowledge
      language_style_choice:
      - literal
      - figurative
      - poetry
    not_famous:
      spell_fails: true
      auditory_feedback: sad_trombone_notes
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_legend_lore
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
    subject_description:
      type: text
      required: true
  outputs:
  - effects
  - information
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - campaign_state
splat_conversion:
  status: specified
  category: spell
  source_sha256: 0e8ebf35e96be3d82b3099fbfaf70443a5e3403ec53995bf54b6298770d9180b
  source_path: builtin/srd-5.2.1/spell/legend_lore.md
  pages:
  - 144
  engine_gaps: &id001
  - dnd2024:spell_legend_lore
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 4f87f4882fcfe311546a098af283efe16e0629d6519b5a0bc810adca64cf9dd4
splat_clauses:
- key: source_rules
  text: '# Legend Lore Level 5 Divination (Bard, Cleric, Wizard) Casting Time: 10 minutes Range: Self
    Components: V, S, M (incense worth 250+ GP, which the spell consumes, and four ivory strips worth
    50+ GP each) Duration: Instantaneous Name or describe a famous person, place, or object. The spell
    brings to your mind a brief summary of the significant lore about that famous thing, as described
    by the GM. The lore might consist of important details, amusing revelations, or even secret lore that
    has never been widely known. The more information you already know about the thing, the more precise
    and detailed the information you receive is. That information is accurate but might be couched in
    figurative language or poetry, as determined by the GM. If the famous thing you chose isn’t actually
    famous, you hear sad musical notes played on a trombone, and the spell fails.'
  sha256: 071b3057488426fe5d9e84de24078dc90ab1c24251de37075c838654d3e7118f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Legend Lore

<!-- SRD page 144 -->

Level 5 Divination (Bard, Cleric, Wizard)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (incense worth 250+ GP, which\
the spell consumes, and four ivory strips worth 50+\
GP each)\
Duration: Instantaneous\
Name or describe a famous person, place, or object.\
The spell brings to your mind a brief summary of\
the significant lore about that famous thing, as de-\
scribed by the GM.\
The lore might consist of important details, amus-\
ing revelations, or even secret lore that has never\
been widely known. The more information you\
already know about the thing, the more precise and\
detailed the information you receive is. That infor-\
mation is accurate but might be couched in figura-\
tive language or poetry, as determined by the GM.\
If the famous thing you chose isn’t actually fa-\
mous, you hear sad musical notes played on a trom-\
bone, and the spell fails.\

Original layout: [[../../srd-5.2.1/pages/144|page 144]].
