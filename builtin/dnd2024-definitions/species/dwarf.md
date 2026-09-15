---
splat_schema: 1
splat_id: 91e85153-b0e2-5bcf-8629-5714061ba51c
splat_kind: rule_group
splat_key: dnd2024:species/dwarf
splat_name: Dwarf
splat_origin:
  key: dnd2024:reference/pages/084
splat_sources:
- title: SRD 5.2.1
  locator: pp. 84
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: species
splat_rule:
  mechanic: dnd2024:species/dwarf
  implementation: dnd2024:species_dwarf
  parameters:
    creature_type: humanoid
    size: medium
    height_feet:
      typical_minimum: 4
      typical_maximum: 5
    speed_feet: 30
    darkvision_feet: 120
    dwarven_resilience:
      damage_resistance:
      - poison
      save_advantage:
        condition: poisoned
        purposes:
        - avoid
        - end
    dwarven_toughness:
      hit_point_maximum_bonus:
        var: actor.character_level
      recalculate_on_level_gain: true
    stonecunning:
      action: bonus_action
      resource:
        id: dwarf.stonecunning
        maximum:
          var: actor.proficiency_bonus
        recovery:
        - event: rest.long_completed
          amount: all
      cost: 1
      duration:
        minutes: 10
      tremorsense_feet: 60
      sense_requires:
        stone_surface_contact: true
        stone_types:
        - natural
        - worked
splat_contracts:
- id: dnd2024:species_dwarf
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
  - character.level_gained
  - saving_throw
  - action.bonus
  - rest.long_completed
  - sense.query
  capabilities:
  - character_features
  - resources
  - damage
  - conditions
  - d20_tests
  - proficiencies
  - movement
  - senses
splat_conversion:
  status: specified
  category: species
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 84
  engine_gaps: &id001
  - dnd2024:species_dwarf
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 84fde5ee0b8ca34ad3a89a97d8ed9750d1bd505faf9928901f11aef3987586af
  line_span:
    first: 7215
    count: 22
splat_clauses:
- key: source_rules
  text: '# Dwarf Creature Type: Humanoid Size: Medium (about 4–5 feet tall) Speed: 30 feet As a Dwarf,
    you have these special traits. Darkvision. You have Darkvision with a range of 120 feet. Dwarven Resilience.
    You have Resistance to Poison damage. You also have Advantage on saving throws you make to avoid or
    end the Poisoned condition. Dwarven Toughness. Your Hit Point maximum increases by 1, and it increases
    by 1 again whenever you gain a level. Stonecunning. As a Bonus Action, you gain Tremorsense with a
    range of 60 feet for 10 minutes. You must be on a stone surface or touching a stone surface to use
    this Tremorsense. The stone can be natural or worked. You can use this Bonus Action a number of times
    equal to your Proficiency Bonus, and you regain all expended uses when you finish a Long Rest.'
  sha256: 5d0faed04fc193d7f8a62b839f0030a322211a4000538b8f5c62b7c1fcb4cf5e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dwarf

<!-- SRD page 84 -->

Creature Type: Humanoid\
Size: Medium (about 4–5 feet tall)\
Speed: 30 feet\
As a Dwarf, you have these special traits.\
Darkvision. You have Darkvision with a range of\
120 feet.\
Dwarven Resilience. You have Resistance to\
Poison damage. You also have Advantage on sav-\
ing throws you make to avoid or end the Poisoned\
condition.\
Dwarven Toughness. Your Hit Point maximum\
increases by 1, and it increases by 1 again whenever\
you gain a level.\
Stonecunning. As a Bonus Action, you gain Trem-\
orsense with a range of 60 feet for 10 minutes. You\
must be on a stone surface or touching a stone sur-\
face to use this Tremorsense. The stone can be natu-\
ral or worked.\
You can use this Bonus Action a number of times\
equal to your Proficiency Bonus, and you regain all\
expended uses when you finish a Long Rest.\
