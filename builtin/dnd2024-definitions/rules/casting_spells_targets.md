---
splat_schema: 1
splat_id: d7edfa47-92d7-5e30-9291-861ce3ec209a
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_targets
splat_name: Targets
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_targets
  parameters:
    target_kind_source: spell_description
    clear_path_required: true
    total_cover_blocks_targeting: true
    self_targeting:
      allowed_if_creature_of_choice: true
      excluded_if:
      - hostile_required
      - other_creature_required
    area_shapes:
    - cone
    - cube
    - cylinder
    - emanation
    - line
    - sphere
    area_targets: spell_area_geometry
    target_awareness:
      requires_perceptible_effect: true
      spell_exception_overrides: true
    invalid_target:
      effect: none
      spent_slot_refunded: false
      perception:
        if_successful_save_normally_no_effect: apparent_successful_save
        otherwise: no_effect_observed
      saving_throw_rolled: false
  implementation: dnd2024:rule_casting_spells_targets
splat_contracts:
- id: dnd2024:rule_casting_spells_targets
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - spell_resolution
  events: []
  capabilities:
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 106
  engine_gaps: &id001
  - dnd2024:rule_casting_spells_targets
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 71e4a4e4397e56e4f854e8379ff1bc8e5164bba2d0c935c98d96dec51aec6cea
  parent_name: Casting Spells
  line_span:
    first: 9256
    count: 35
splat_clauses:
- key: source_rules
  text: '# Targets A typical spell requires the caster to pick one or more targets to be affected by the
    spell’s magic. A spell’s description says whether the spell targets creatures, objects, or something
    else. A Clear Path to the Target. To target something with a spell, a caster must have a clear path
    to it, so it can’t be behind Total Cover. Targeting Yourself. If a spell targets a creature of your
    choice, you can choose yourself unless the creature must be Hostile or specifically a creature other
    than you. Areas of Effect. Some spells, such as Thunderwave, cover an area called an area of effect,
    which is defined in “Rules Glossary.” The area determines what the spell targets. The description
    of a spell specifies whether it has an area of effect, which is typically one of these shapes: Cone,
    Cube, Cylinder, Emanation, Line, or Sphere. Awareness of Being Targeted. Unless a spell has a perceptible
    effect, a creature doesn’t know it was targeted by the spell. An effect like lightning is obvious,
    but a more subtle effect, such as an attempt to read thoughts, goes unnoticed unless a spell’s description
    says otherwise. Invalid Targets. If you cast a spell on someone or something that can’t be affected
    by it, nothing happens to that target, but if you used a spell slot to cast the spell, the slot is
    still expended. If the spell normally has no effect on a target that succeeds on a saving throw, the
    invalid target appears to have succeeded on its saving throw, even though it didn’t attempt one (giving
    no hint that the creature is an invalid target). Otherwise, you perceive that the spell did nothing
    to the target.'
  sha256: cf0b65da274c82b2a6cdd74dfbf1bd08f05300b718df89c3a033160ff28362b8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Targets

<!-- SRD page 106 -->

A typical spell requires the caster to pick one or\
more targets to be affected by the spell’s magic. A\
spell’s description says whether the spell targets\
creatures, objects, or something else.\
A Clear Path to the Target. To target something\
with a spell, a caster must have a clear path to it, so\
it can’t be behind Total Cover.\
Targeting Yourself. If a spell targets a creature\
of your choice, you can choose yourself unless the\
creature must be Hostile or specifically a creature\
other than you.\
Areas of Effect. Some spells, such as Thunder-\
wave, cover an area called an area of effect, which\
is defined in “Rules Glossary.” The area determines\
what the spell targets. The description of a spell\
specifies whether it has an area of effect, which is\
typically one of these shapes: Cone, Cube, Cylinder,\
Emanation, Line, or Sphere.\
Awareness of Being Targeted. Unless a spell has\
a perceptible effect, a creature doesn’t know it was\
targeted by the spell. An effect like lightning is ob-\
vious, but a more subtle effect, such as an attempt\
to read thoughts, goes unnoticed unless a spell’s\
description says otherwise.\
Invalid Targets. If you cast a spell on someone\
or something that can’t be affected by it, nothing\
happens to that target, but if you used a spell slot to\
cast the spell, the slot is still expended.\
If the spell normally has no effect on a target that\
succeeds on a saving throw, the invalid target ap-\
pears to have succeeded on its saving throw, even\
though it didn’t attempt one (giving no hint that the\
creature is an invalid target). Otherwise, you per-\
ceive that the spell did nothing to the target.\
