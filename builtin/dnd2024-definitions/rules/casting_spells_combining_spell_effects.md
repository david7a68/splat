---
splat_schema: 1
splat_id: 523247e2-233b-5d10-ab0e-df2df3b7968f
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_combining_spell_effects
splat_name: Combining Spell Effects
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_combining_spell_effects
  parameters:
    different_spell_effects: combine
    same_spell_effects:
      stack: false
      selection:
      - most_potent
      - most_recent_if_equal
      scope: overlapping_durations
      independent_durations_continue: true
    identify_ongoing_spell:
      requires:
      - noninstantaneous
      - ongoing_duration
      - observable_effects
      action: study
      ability: intelligence
      skill: arcana
      dc: 15
      success: identify_spell
  implementation: dnd2024:rule_casting_spells_combining_spell_effects
splat_contracts:
- id: dnd2024:rule_casting_spells_combining_spell_effects
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_resolution
  events: []
  capabilities:
  - spellcasting
  - effects
  - checks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 106
  engine_gaps: &id001
  - dnd2024:rule_casting_spells_combining_spell_effects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: f4d0e6ca298168053f83860c0aec35058154f6270bcaefeaf07f3b7e7e89c2be
  parent_name: Casting Spells
  line_span:
    first: 9306
    count: 19
splat_clauses:
- key: source_rules
  text: '# Combining Spell Effects The effects of different spells add together while their durations
    overlap. In contrast, the effects of the same spell cast multiple times don’t combine. Instead, the
    most potent effect—such as the highest bonus—from those castings applies while their durations overlap.
    The most recent effect applies if the castings are equally potent and their durations overlap. For
    example, if two Clerics cast Bless on the same target, that target gains the spell’s benefit only
    once; the target doesn’t receive two bonus dice. But if the durations of the spells overlap, the effect
    continues until the duration of the second Bless ends. idEntiFyinG an onGoinG spEll You can try to
    identify a non-instantaneous spell by its observable effects if its duration is ongoing. To identify
    it, you must take the Study action and succeed on a DC 15 Intelligence (Arcana) check.'
  sha256: df9a909bdc71520fc05335b31e93d7fc97db9874a43ae52e584c843abd94adb2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Combining Spell Effects

<!-- SRD page 106 -->

The effects of different spells add together while\
their durations overlap. In contrast, the effects of\
the same spell cast multiple times don’t combine.\
Instead, the most potent effect—such as the high-\
est bonus—from those castings applies while their\
durations overlap. The most recent effect applies if\
the castings are equally potent and their durations\
overlap. For example, if two Clerics cast Bless on\
the same target, that target gains the spell’s benefit\
only once; the target doesn’t receive two bonus dice.\
But if the durations of the spells overlap, the effect\
continues until the duration of the second Bless\
ends.\
idEntiFyinG an onGoinG spEll\
You can try to identify a non-instantaneous spell by\
its observable effects if its duration is ongoing. To\
identify it, you must take the Study action and suc-\
ceed on a DC 15 Intelligence (Arcana) check.\
