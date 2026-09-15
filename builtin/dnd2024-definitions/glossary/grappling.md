---
splat_schema: 1
splat_id: 470f050d-30bd-597d-ad11-c8d557a4463a
splat_kind: rule
splat_key: dnd2024:glossary/grappling
splat_name: Grappling
splat_origin:
  key: dnd2024:reference/glossary/grappling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/grappling
  parameters:
    successful_grapple_condition: grappled
    body_part:
      default: free_hand
      alternatives_require_explicit_rule: true
      targets_per_part: 1
      cannot_target_other_creatures_while_grappling: true
    escape:
      cost: action
      check_choice:
        chooser: subject
        options:
        - ability: strength
          skill: athletics
        - ability: dexterity
          skill: acrobatics
      dc: grapple_escape_dc
      success: end_grappled
    also_ends_when_any:
    - grappler_incapacitated
    - distance_exceeds_grapple_range
    release:
      chooser: grappler
      timing: any_time
      cost: none
  implementation: dnd2024:glossary_grappling
splat_contracts:
- id: dnd2024:glossary_grappling
  inputs:
    subject:
      type: participant
      required: true
    grappler:
      type: participant
      required: true
    grapple:
      type: effect
      required: true
    escape_choice:
      required: false
      type: choice
  outputs:
  - grapple_state
  - escape_check
  - body_part_reservation
  events:
  - grapple.started
  - action.requested
  - condition.changed
  - movement.completed
  - grapple.released
  capabilities:
  - grappling
  - conditions
  - checks
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: a410ea077d9df57019d795c0ae09dc625944e2d4cbb7d9b9b93c803d71ed7439
  source_path: builtin/srd-5.2.1/glossary/grappling.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_grappling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: ea10f659eb8b541e3ff3357d23a02110e241466b4eb0b7ad315f425b8585bca9
splat_clauses:
- key: source_rules
  text: '# Grappling A creature can grapple another creature. Characters typically grapple by using an
    Unarmed Strike. Many monsters have special attacks that allow them to quickly grapple prey. However
    a grapple is initiated, it follows these rules. See also “Unarmed Strike” and “Grappled.” Grappled
    Condition. Successfully grappling a creature gives it the Grappled condition. One Grapple per Hand.
    A creature must have a hand free to grapple another creature. Some stat blocks and game effects allow
    a creature to grapple using a tentacle, a maw, or another body part. Whatever part a grappler uses,
    it can grapple only one creature at a time with that part, and the grappler can’t use that part to
    target another creature unless it ends the grapple. Ending a Grapple. A Grappled creature can use
    its action to make a Strength (Athletics) or Dexterity (Acrobatics) check against the grapple’s escape
    DC, ending the condition on itself on a success. The condition also ends if the grappler has the Incapacitated
    condition or if the distance between the Grappled target and the grappler exceeds the grapple’s range.
    In addition, the grappler can release the target at any time (no action required).'
  sha256: 354a1b5daf1de8824219edef812874164253fabfc8ab22102c6b507c728df4c2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Grappling

<!-- SRD page 182 -->

A creature can grapple another creature. Charac-\
ters typically grapple by using an Unarmed Strike.\
Many monsters have special attacks that allow\
them to quickly grapple prey. However a grapple is\
initiated, it follows these rules. See also “Unarmed\
Strike” and “Grappled.”\
Grappled Condition. Successfully grappling a\
creature gives it the Grappled condition.\
One Grapple per Hand. A creature must have a\
hand free to grapple another creature. Some stat\
blocks and game effects allow a creature to grapple\
using a tentacle, a maw, or another body part. What-\
ever part a grappler uses, it can grapple only one\
creature at a time with that part, and the grappler\
can’t use that part to target another creature unless\
it ends the grapple.\
Ending a Grapple. A Grappled creature can use\
its action to make a Strength (Athletics) or Dexter-\
ity (Acrobatics) check against the grapple’s escape\
DC, ending the condition on itself on a success. The\
condition also ends if the grappler has the Inca-\
pacitated condition or if the distance between the\
Grappled target and the grappler exceeds the grap-\
ple’s range. In addition, the grappler can release the\
target at any time (no action required).\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
