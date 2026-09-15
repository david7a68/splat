---
splat_schema: 1
splat_id: aaa8e706-f954-5859-ab92-5180b3438853
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_the_nine_alignments
splat_name: The Nine Alignments
splat_origin:
  key: dnd2024:reference/pages/021
splat_sources:
- title: SRD 5.2.1
  locator: pp. 21, 22
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_the_nine_alignments
  parameters:
    alignment_axes:
      morality:
      - good
      - neutral
      - evil
      order:
      - lawful
      - neutral
      - chaotic
    options:
    - lawful_good
    - neutral_good
    - chaotic_good
    - lawful_neutral
    - neutral
    - chaotic_neutral
    - lawful_evil
    - neutral_evil
    - chaotic_evil
    rational_thought_absent_default: unaligned
    behavior_binding: false
  implementation: dnd2024:rule_create_your_character_the_nine_alignments
splat_contracts:
- id: dnd2024:rule_create_your_character_the_nine_alignments
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 21
  - 22
  engine_gaps: &id001
  - dnd2024:rule_create_your_character_the_nine_alignments
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 7b0231f7c236656f7e9f53db6af2cd9c0d3f7aec5d42998ef79147b85b468255
  parent_name: Create Your Character
  line_span:
    first: 1583
    count: 54
splat_clauses:
- key: source_rules
  text: '# The Nine Alignments A creature’s alignment broadly describes its ethical attitudes and ideals.
    Alignment is a combination of two factors: one identifies morality (good, evil, or neutral), and the
    other describes attitudes toward order (lawful, chaotic, or neutral). The summaries of the alignments
    below describe the typical behavior of a creature with that alignment; individuals can vary from that
    behavior. Lawful Good (LG). Lawful Good creatures endeavor to do the right thing as expected by society.
    Someone who fights injustice and protects the innocent without hesitation is probably Lawful Good.
    Neutral Good (NG). Neutral Good creatures do the best they can, working within rules but not feeling
    bound by them. A kindly person who helps others according to their needs is probably Neutral Good.
    Chaotic Good (CG). Chaotic Good creatures act as their conscience directs with little regard for what
    others expect. A rebel who waylays a cruel baron’s tax collectors and uses the stolen money to help
    the poor is probably Chaotic Good. Lawful Neutral (LN). Lawful Neutral individuals act in accordance
    with law, tradition, or personal codes. Someone who follows a disciplined rule of life—and isn’t swayed
    either by the demands of those in need or by the temptations of evil—is probably Lawful Neutral. Neutral
    (N). Neutral is the alignment of those who prefer to avoid moral questions and don’t take sides, unaliGnEd
    crEaturEs Most creatures that lack the capacity for rational thought don’t have alignments; they are
    unaligned. Sharks are savage predators, for example, but they aren’t evil; they are unaligned. doing
    what seems best at the time. Someone who’s bored by moral debate is probably Neutral. Chaotic Neutral
    (CN). Chaotic Neutral creatures follow their whims, valuing their personal freedom above all else.
    A scoundrel who wanders the land living by their wits is probably Chaotic Neutral. Lawful Evil (LE).
    Lawful Evil creatures methodically take what they want within the limits of a code of tradition, loyalty,
    or order. An aristocrat exploiting citizens while scheming for power is probably Lawful Evil. Neutral
    Evil (NE). Neutral Evil is the alignment of those who are untroubled by the harm they cause as they
    pursue their desires. A criminal who robs and murders as they please is probably Neutral Evil. Chaotic
    Evil (CE). Chaotic Evil creatures act with arbitrary violence, spurred by their hatred or bloodlust.
    A villain pursuing schemes of vengeance and havoc is probably Chaotic Evil.'
  sha256: 0c3bc62a92fdf1a753acb7161353374e3ff914afac1f62d73c7d5eef1dc95aa4
  status: contract_specified
splat_requires_mechanics: *id001
---

# The Nine Alignments

<!-- SRD page 21 -->

A creature’s alignment broadly describes its ethical\
attitudes and ideals. Alignment is a combination of\
two factors: one identifies morality (good, evil, or\
neutral), and the other describes attitudes toward\
order (lawful, chaotic, or neutral).\
The summaries of the alignments below describe\
the typical behavior of a creature with that align-\
ment; individuals can vary from that behavior.\
Lawful Good (LG). Lawful Good creatures en-\
deavor to do the right thing as expected by society.\
Someone who fights injustice and protects the inno-\
cent without hesitation is probably Lawful Good.\
Neutral Good (NG). Neutral Good creatures do the\
best they can, working within rules but not feeling\
bound by them. A kindly person who helps others\
according to their needs is probably Neutral Good.\
Chaotic Good (CG). Chaotic Good creatures act as\
their conscience directs with little regard for what\
others expect. A rebel who waylays a cruel baron’s\
tax collectors and uses the stolen money to help the\
poor is probably Chaotic Good.\
Lawful Neutral (LN). Lawful Neutral individuals\
act in accordance with law, tradition, or personal\
codes. Someone who follows a disciplined rule of\
life—and isn’t swayed either by the demands of\
those in need or by the temptations of evil—is prob-\
ably Lawful Neutral.\
Neutral (N). Neutral is the alignment of those who\
prefer to avoid moral questions and don’t take sides,\

<!-- SRD page 22 -->

unaliGnEd crEaturEs\
Most creatures that lack the capacity for rational\
thought don’t have alignments; they are unaligned.\
Sharks are savage predators, for example, but they\
aren’t evil; they are unaligned.\
doing what seems best at the time. Someone who’s\
bored by moral debate is probably Neutral.\
Chaotic Neutral (CN). Chaotic Neutral creatures\
follow their whims, valuing their personal freedom\
above all else. A scoundrel who wanders the land\
living by their wits is probably Chaotic Neutral.\
Lawful Evil (LE). Lawful Evil creatures methodi-\
cally take what they want within the limits of a code\
of tradition, loyalty, or order. An aristocrat exploit-\
ing citizens while scheming for power is probably\
Lawful Evil.\
Neutral Evil (NE). Neutral Evil is the alignment of\
those who are untroubled by the harm they cause as\
they pursue their desires. A criminal who robs and\
murders as they please is probably Neutral Evil.\
Chaotic Evil (CE). Chaotic Evil creatures act\
with arbitrary violence, spurred by their hatred or\
bloodlust. A villain pursuing schemes of vengeance\
and havoc is probably Chaotic Evil.\
