---
splat_schema: 1
splat_id: 354ce3e5-2276-5504-a741-ec46d2f38de5
splat_kind: rule_group
splat_key: dnd2024:species/orc
splat_name: Orc
splat_origin:
  key: dnd2024:reference/pages/086
splat_sources:
- title: SRD 5.2.1
  locator: pp. 86
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: species
splat_rule:
  mechanic: dnd2024:species/orc
  implementation: dnd2024:species_orc
  parameters:
    creature_type: humanoid
    size: medium
    height_feet:
      typical_minimum: 6
      typical_maximum: 7
    speed_feet: 30
    darkvision_feet: 120
    adrenaline_rush:
      action: bonus_action
      take_action: dash
      temporary_hit_points:
        var: actor.proficiency_bonus
      resource:
        id: orc.adrenaline_rush
        maximum:
          var: actor.proficiency_bonus
        recovery:
        - event: rest.short_completed
          amount: all
        - event: rest.long_completed
          amount: all
      cost: 1
    relentless_endurance:
      trigger: actor.reduced_to_zero_hit_points
      requires:
        killed_outright: false
      optional: true
      instead_hit_points: 1
      resource:
        id: orc.relentless_endurance
        maximum: 1
        recovery:
        - event: rest.long_completed
          amount: all
      cost: 1
splat_contracts:
- id: dnd2024:species_orc
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - character.species_selected
  - action.bonus
  - hit_points.reduced_to_zero
  - rest.short_completed
  - rest.long_completed
  capabilities:
  - character_features
  - resources
  - proficiencies
  - movement
splat_conversion:
  status: specified
  category: species
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 86
  engine_gaps: &id001
  - dnd2024:species_orc
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: ecc1c6e854575396124c0fd78fa07a94a6a340933ca152b1079747118bdfc037
  line_span:
    first: 7402
    count: 18
splat_clauses:
- key: source_rules
  text: '# Orc Creature Type: Humanoid Size: Medium (about 6–7 feet tall) Speed: 30 feet As an Orc, you
    have these special traits. Adrenaline Rush. You can take the Dash action as a Bonus Action. When you
    do so, you gain a number of Temporary Hit Points equal to your Proficiency Bonus. You can use this
    trait a number of times equal to your Proficiency Bonus, and you regain all expended uses when you
    finish a Short or Long Rest. Darkvision. You have Darkvision with a range of 120 feet. Relentless
    Endurance. When you are reduced to 0 Hit Points but not killed outright, you can drop to 1 Hit Point
    instead. Once you use this trait, you can’t do so again until you finish a Long Rest.'
  sha256: 427648a0a8fec8152ca254458a124e9bdfac509ed676a410a2a8abc7458e3e2a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Orc

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Medium (about 6–7 feet tall)\
Speed: 30 feet\
As an Orc, you have these special traits.\
Adrenaline Rush. You can take the Dash action as\
a Bonus Action. When you do so, you gain a number\
of Temporary Hit Points equal to your Proficiency\
Bonus.\
You can use this trait a number of times equal\
to your Proficiency Bonus, and you regain all ex-\
pended uses when you finish a Short or Long Rest.\
Darkvision. You have Darkvision with a range of\
120 feet.\
Relentless Endurance. When you are reduced to\
0 Hit Points but not killed outright, you can drop to\
1 Hit Point instead. Once you use this trait, you can’t\
do so again until you finish a Long Rest.\
