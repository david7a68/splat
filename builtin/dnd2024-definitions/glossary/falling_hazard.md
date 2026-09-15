---
splat_schema: 1
splat_id: 07914b89-3c94-5093-865a-cde91556bbcd
splat_kind: rule
splat_key: dnd2024:glossary/falling_hazard
splat_name: Falling [Hazard]
splat_origin:
  key: dnd2024:reference/glossary/falling_hazard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/falling_hazard
  parameters:
    damage:
      timing: fall_end
      type: bludgeoning
      die_sides: 6
      dice_count:
        op: min
        args:
        - 20
        - op: floor
          args:
          - op: divide
            args:
            - var: fall_distance_feet
            - 10
    landing_condition:
      condition: prone
      unless_no_fall_damage: true
    liquid_landing:
      eligible_liquids: any
      optional: true
      cost: reaction
      check:
        chooser: subject
        options:
        - ability: strength
          skill: athletics
        - ability: dexterity
          skill: acrobatics
        dc: 15
      success:
        entry_orientation:
        - head_first
        - feet_first
        fall_damage_multiplier: 0.5
  implementation: dnd2024:glossary_falling_hazard
splat_contracts:
- id: dnd2024:glossary_falling_hazard
  inputs:
    subject:
      type: participant
      required: true
    fall_distance_feet:
      type: number
      required: true
    landing_in_liquid:
      type: boolean
      required: true
    liquid_reaction:
      required: false
      type: choice
  outputs:
  - damage
  - condition_changes
  - reaction_check
  events:
  - fall.ended
  capabilities:
  - hazards
  - damage
  - checks
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: db7a82a886a1477474d2eb947bb09f72b3b3841c4599aac10fdaad61d93f6937
  source_path: builtin/srd-5.2.1/glossary/falling_hazard.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_falling_hazard
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 0cd232fed55d2eea8d92452525aa58e645403301735f8d20b98d367e06eb8047
splat_clauses:
- key: source_rules
  text: '# Falling [Hazard] A creature that falls takes 1d6 Bludgeoning damage at the end of the fall
    for every 10 feet it fell, to a maximum of 20d6. When the creature lands, it has the Prone condition
    unless it avoids taking any damage from the fall. A creature that falls into water or another liquid
    can use its Reaction to make a DC 15 Strength (Athletics) or Dexterity (Acrobatics) check to hit the
    surface head or feet first. On a successful check, any damage resulting from the fall is halved.'
  sha256: fb563f5e95142a30fe93cffa02ada4476df13a8e4d387a9dbb8f2b7220dbad51
  status: contract_specified
splat_requires_mechanics: *id001
---

# Falling [Hazard]

<!-- SRD page 182 -->

A creature that falls takes 1d6 Bludgeoning dam-\
age at the end of the fall for every 10 feet it fell, to\
a maximum of 20d6. When the creature lands, it\
has the Prone condition unless it avoids taking any\
damage from the fall.\
A creature that falls into water or another liquid\
can use its Reaction to make a DC 15 Strength (Ath-\
letics) or Dexterity (Acrobatics) check to hit the\
surface head or feet first. On a successful check, any\
damage resulting from the fall is halved.\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
