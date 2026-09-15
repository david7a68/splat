---
splat_schema: 1
splat_id: 8efee93e-b3fd-512d-af09-16db9a747c57
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_narrative_curses
splat_name: Narrative Curses
splat_origin:
  key: dnd2024:reference/pages/193
splat_sources:
- title: SRD 5.2.1
  locator: pp. 193
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magical_contagions_narrative_curses
  parameters:
    trigger:
      authority: gm
      kind: taboo_violation
      examples:
      - broken_vow
      - defiled_tomb
      - murdered_innocent
    curse_design:
      authority: gm
      effect_input_type: effect
      may_customize_other_curse: true
    victim_information:
      punishment_reason_known: true
      ending_method_discoverable: true
    remove_curse_response:
      authority: gm
      mode: choice
      allowed_modes:
      - end
      - suppress
      - custom_effect
      suppression_duration_input: duration
  implementation: dnd2024:rules_magical_contagions_narrative_curses
splat_contracts:
- id: dnd2024:rules_magical_contagions_narrative_curses
  inputs:
    victim:
      type: participant
      required: true
    violation:
      type: resolution
      required: true
    curse_effect:
      type: effect
      required: true
    ending_method:
      type: resolution
      required: true
    remove_curse_response:
      type: choice
      required: true
    suppression_duration:
      required: false
      type: duration
  outputs:
  - curse
  - curse_knowledge
  events: []
  capabilities:
  - curses
  - narrative_choices
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 193
  engine_gaps: &id001
  - dnd2024:rules_magical_contagions_narrative_curses
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: f005f91ea13fb816eef478fe2579ca71d25d80c99b8cab9360ed17c1093b85cd
  parent_name: Magical Contagions
  line_span:
    first: 9465
    count: 17
splat_clauses:
- key: source_rules
  text: '# Narrative Curses A curse might manifest during an adventure when a creature’s violation of
    a taboo warrants supernatural punishment, such as breaking a vow, defiling a tomb, or murdering an
    innocent. Such a curse can have any effects you design, or it might be a customized version of another
    type of curse discussed in this section. A creature affected by such a curse should know why they’re
    being punished and be able to learn how to end their curse, likely by symbolically righting the wrong
    they committed. How a spell like Remove Curse affects a curse that’s part of your adventure is up
    to you—the spell might merely suppress the effects of the curse for a time. Regardless, narrative
    curses should feel like rare, potent magic rooted in the lore of your campaign.'
  sha256: ec23a09d68ad1d259b2a9e12446f1827b9ef09534465970c93e93baf5eb63ed7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Narrative Curses

<!-- SRD page 193 -->

A curse might manifest during an adventure when\
a creature’s violation of a taboo warrants supernat-\
ural punishment, such as breaking a vow, defiling a\
tomb, or murdering an innocent. Such a curse can\
have any effects you design, or it might be a custom-\
ized version of another type of curse discussed in\
this section.\
A creature affected by such a curse should know\
why they’re being punished and be able to learn\
how to end their curse, likely by symbolically\
righting the wrong they committed. How a spell\
like Remove Curse affects a curse that’s part of your\
adventure is up to you—the spell might merely sup-\
press the effects of the curse for a time. Regardless,\
narrative curses should feel like rare, potent magic\
rooted in the lore of your campaign.\
