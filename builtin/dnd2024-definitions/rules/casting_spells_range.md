---
splat_schema: 1
splat_id: 5d677f85-ca06-54d5-8562-f93436218c4f
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_range
splat_name: Range
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_range
  parameters:
    range_modes:
      distance:
        unit: feet
        limits: spell_specified_effect_origin
      touch:
        requires_touch: true
        within_caster_reach: true
      self:
        origin: caster
        mode_from_spell:
        - on_caster
        - emanates_from_caster
    movable_effects_range_limited_after_cast_default: false
    specific_spell_overrides: true
  implementation: dnd2024:rules_casting_spells_range
splat_contracts:
- id: dnd2024:rules_casting_spells_range
  inputs:
    caster:
      type: participant
      required: true
    spell:
      type: definition
      required: true
    target:
      type: resolution
      required: true
  outputs:
  - range_eligibility
  events: []
  capabilities:
  - spellcasting
  - geometry
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 105
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_range
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: c6153ced4722ce4e69c4901a051aac81a7b4c5e1f1907defc0e1217c82af917d
  parent_name: Casting Spells
  line_span:
    first: 9175
    count: 15
splat_clauses:
- key: source_rules
  text: '# Range A spell’s range indicates how far from the spellcaster the spell’s effect can originate,
    and the spell’s description specifies which part of the effect is limited by the range. A range usually
    takes one of the following forms: Distance. The range is expressed in feet. Touch. The spell’s effect
    originates on something, as defined by the spell, that the spellcaster must touch within their reach.
    Self. The spell is cast on the spellcaster or emanates from them, as specified in the spell. If a
    spell has movable effects, they aren’t restricted by its range unless the spell’s description says
    otherwise.'
  sha256: 45a8ee0000d54fcf50d98cf0cddf9223e75925af11cfe9a89d5e92f2df7fa5ff
  status: contract_specified
splat_requires_mechanics: *id001
---

# Range

<!-- SRD page 105 -->

A spell’s range indicates how far from the spell-\
caster the spell’s effect can originate, and the spell’s\
description specifies which part of the effect is lim-\
ited by the range.\
A range usually takes one of the following forms:\
Distance. The range is expressed in feet.\
Touch. The spell’s effect originates on something,\
as defined by the spell, that the spellcaster must\
touch within their reach.\
Self. The spell is cast on the spellcaster or emanates\
from them, as specified in the spell.\
If a spell has movable effects, they aren’t restricted\
by its range unless the spell’s description says\
otherwise.\
