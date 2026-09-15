---
splat_schema: 1
splat_id: 23c2a7d0-f633-56b9-b23d-c4008e65c4af
splat_kind: rule
splat_key: dnd2024:glossary/knocking_out_a_creature
splat_name: Knocking Out a Creature
splat_origin:
  key: dnd2024:reference/glossary/knocking_out_a_creature
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/knocking_out_a_creature
  parameters:
    trigger:
      attack_type: melee
      would_reduce_target_to_hit_points: 0
    optional_attacker_choice: true
    replacement_hit_points: 1
    apply_condition: unconscious
    start_rest: short_rest
    ends_unconscious_when_any:
    - regains_any_hit_points
    - successful_first_aid
    first_aid:
      cost: action
      ability: wisdom
      skill: medicine
      dc: 10
  implementation: dnd2024:glossary_knocking_out_a_creature
splat_contracts:
- id: dnd2024:glossary_knocking_out_a_creature
  inputs:
    attacker:
      type: participant
      required: true
    target:
      type: participant
      required: true
    knock_out:
      type: boolean
      required: true
    first_aid_actor:
      required: false
      type: participant
  outputs:
  - hit_points
  - condition_changes
  - short_rest_started
  - first_aid_check
  events:
  - damage.pending
  - healing.applied
  - action.requested
  capabilities:
  - attacks
  - hit_points
  - conditions
  - rests
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 9aeaf09f1854f4f72d2c1e7e7dbd40379c721e1a8e2bf79f8ca6037a3e3a00ea
  source_path: builtin/srd-5.2.1/glossary/knocking_out_a_creature.md
  pages:
  - 184
  engine_gaps: &id001
  - dnd2024:glossary_knocking_out_a_creature
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: d1ed84703ee72e92f3e0f8272a548d58e266d8c0529e7262ac884aa4f408c3b3
splat_clauses:
- key: source_rules
  text: '# Knocking Out a Creature When you would reduce a creature to 0 Hit Points with a melee attack,
    you can instead reduce the creature to 1 Hit Point. The creature then has the Unconscious condition
    and starts a Short Rest. The creature remains Unconscious until it regains any Hit Points or until
    someone uses an action to administer first aid to it, which requires a successful DC 10 Wisdom (Medicine)
    check.'
  sha256: e5e7be70bca92adaa5cabdb140ce990c72fa288afb7aa325591e54c6724060f2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Knocking Out a Creature

<!-- SRD page 184 -->

When you would reduce a creature to 0 Hit Points\
with a melee attack, you can instead reduce the\
creature to 1 Hit Point. The creature then has the\
Unconscious condition and starts a Short Rest.\
The creature remains Unconscious until it regains\
any Hit Points or until someone uses an action to\
administer first aid to it, which requires a success-\
ful DC 10 Wisdom (Medicine) check.\

Original layout: [[../../srd-5.2.1/pages/184|page 184]].
