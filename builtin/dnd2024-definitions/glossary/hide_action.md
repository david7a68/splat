---
splat_schema: 1
splat_id: 683841dc-a0ff-5b2e-a884-d671e301c0c8
splat_kind: rule
splat_key: dnd2024:glossary/hide_action
splat_name: Hide [Action]
splat_origin:
  key: dnd2024:reference/glossary/hide_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/hide_action
  parameters:
    cost: action
    prerequisites:
      any:
      - heavily_obscured
      - three_quarters_cover
      - total_cover
      outside_all_enemy_lines_of_sight: true
    visibility_information:
      can_see_creature: true
      can_discern_if_creature_sees_subject: true
    check:
      ability: dexterity
      skill: stealth
      dc: 15
    success:
      hidden: true
      condition: invisible
      find_check:
        ability: wisdom
        skill: perception
        dc:
          var: hide_check_total
    ends_immediately_after_any:
    - sound_louder_than_whisper
    - enemy_finds_subject
    - subject_makes_attack_roll
    - subject_casts_verbal_component_spell
  implementation: dnd2024:glossary_hide_action
splat_contracts:
- id: dnd2024:glossary_hide_action
  inputs:
    subject:
      type: participant
      required: true
    geometry:
      type: resolution
      required: true
  outputs:
  - hide_check
  - hidden_effect
  - find_dc
  events:
  - action.requested
  - sound.made
  - hidden_creature.found
  - attack.rolled
  - spell.cast
  capabilities:
  - action_economy
  - stealth
  - visibility
  - checks
  - conditions
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 40aeed1f6047f561271793954025ad10881022ae3d3ccfb14bd2eb7b36294260
  source_path: builtin/srd-5.2.1/glossary/hide_action.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_hide_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: cb0fedc8bfcf4ab4f6797045d6a69712c1e9efb15586831678a9844935acb736
splat_clauses:
- key: source_rules
  text: '# Hide [Action] With the Hide action, you try to hide yourself. To do so, you must succeed on
    a DC 15 Dexterity (Stealth) check while you’re Heavily Obscured or behind Three-Quarters Cover or
    Total Cover, and you must be out of any enemy’s line of sight; if you can see a creature, you can
    discern whether it can see you. On a successful check, you have the Invisible condition while hidden.
    Make note of your check’s total, which is the DC for a creature to find you with a Wisdom (Perception)
    check. You stop being hidden immediately after any of the following occurs: you make a sound louder
    than a whisper, an enemy finds you, you make an attack roll, or you cast a spell with a Verbal component.'
  sha256: ffab6f6a0b1f6b887fd0221b78b6a5545159ad1bfad7ac0b062b9b45f32c31b6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hide [Action]

<!-- SRD page 183 -->

With the Hide action, you try to hide yourself. To do\
so, you must succeed on a DC 15 Dexterity (Stealth)\
check while you’re Heavily Obscured or behind\
Three-Quarters Cover or Total Cover, and you must\
be out of any enemy’s line of sight; if you can see a\
creature, you can discern whether it can see you.\
On a successful check, you have the Invisible con-\
dition while hidden. Make note of your check’s total,\
which is the DC for a creature to find you with a\
Wisdom (Perception) check.\
You stop being hidden immediately after any of\
the following occurs: you make a sound louder than\
a whisper, an enemy finds you, you make an attack\
roll, or you cast a spell with a Verbal component.\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].
