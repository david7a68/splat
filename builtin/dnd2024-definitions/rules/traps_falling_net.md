---
splat_schema: 1
splat_id: df3ce1cb-9599-538b-8830-8dfe93453ce7
splat_kind: rule
splat_key: dnd2024:rules/traps_falling_net
splat_name: Falling Net
splat_origin:
  key: dnd2024:reference/pages/199
splat_sources:
- title: SRD 5.2.1
  locator: pp. 199
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/traps_falling_net
  parameters:
    trap:
      severity: nuisance
      levels:
      - 1
      - 4
      trigger: first_creature_crosses_tripwire
      duration: instantaneous
    geometry:
      net_square_side_feet: 10
      tripwire_height_inches: 3
      tripwire_between:
      - columns
      - trees
    save:
      ability: dexterity
      dc: 10
      automatic_success_size_minimum: huge
    failure:
      condition: restrained
      until: escape_or_net_destroyed
    escape:
      economy: action
      check:
        ability: strength
        dc: 10
        skill: athletics
      targets:
      - self
      - other_within_reach
    detection:
      economy: search
      check:
        ability: wisdom
        dc: 11
        skill: perception
      reveals:
      - tripwire
      - net
    disarm:
      after_detection:
      - cut_tripwire
      - avoid_tripwire
      check_required: false
    destroy:
      net_hp: 0
      frees_all: true
      statistics_source:
        key: dnd2024:equipment/net_1_gp
    set:
      requires:
      - thieves_tools
      - all_components
      - net
      check:
        ability: dexterity
        dc: 13
        skill: sleight_of_hand
      attempt_duration:
        value: 10
        unit: minutes
    scaling:
    - levels:
      - 1
      - 4
      save_and_escape_dc: 10
    - levels:
      - 5
      - 10
      save_and_escape_dc: 12
    - levels:
      - 11
      - 16
      save_and_escape_dc: 14
    - levels:
      - 17
      - 20
      save_and_escape_dc: 16
  implementation: dnd2024:rules_traps_falling_net
splat_contracts:
- id: dnd2024:rules_traps_falling_net
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    hazard:
      type: object
      required: true
    event:
      type: event
      required: true
    resolution:
      type: resolution
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - effects
  - hazard_updates
  events:
  - movement.completed
  capabilities:
  - hazards
  - effects
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 199
  engine_gaps: &id001
  - dnd2024:rules_traps_falling_net
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 75d40280181cbe97413457524a6f413e7d83654d36c32f7cd9acf37dd4f0fe00
  parent_name: Traps
  line_span:
    first: 9999
    count: 35
splat_clauses:
- key: source_rules
  text: '# Falling Net Nuisance Trap (Levels 1–4) Trigger: A creature crosses a trip wire Duration: Instantaneous
    A falling net trap uses a trip wire to release a weighted, 10-foot-square Net suspended from the ceiling.
    The trip wire is 3 inches off the ground and stretches between two columns or trees. The first creature
    that crosses the trip wire causes the Net to fall on it. The target must succeed on a DC 10 Dexterity
    saving throw or have the Restrained condition until it escapes. The target succeeds automatically
    if it’s Huge or larger. A creature can take an action to make a DC 10 Strength (Athletics) check,
    freeing itself or another creature within its reach from the Net on a successful check. Detect and
    Disarm. As a Search action, a creature can examine the trapped area and make a DC 11 Wisdom (Perception)
    check, detecting the trip wire and the suspended Net on a successful check. Once detected, the trip
    wire can be easily cut or avoided (no ability check required). Destroy the Net. Reducing the Net to
    0 Hit Points frees any creature trapped in it (see “Adventuring Gear” for the Net’s statistics). Set
    the Trap. A creature that has Thieves’ Tools and all the trap’s components (including a Net) can try
    to set a falling net trap, doing so with a successful DC 13 Dexterity (Sleight of Hand) check. Each
    attempt to set this trap takes 10 minutes. At Higher Levels. You can scale the trap for higher levels
    by increasing the weight of the Net, which increases the save DC and the DC of the Strength (Athletics)
    check as follows: DC 12 at levels 5–10, DC 14 at levels 11–16, or DC 16 at levels 17–20.'
  sha256: ee5c46a65c64cb7513497c5c92cd32a4194013d1e2691ee39acb4540ef157a19
  status: contract_specified
splat_requires_mechanics: *id001
---

# Falling Net

<!-- SRD page 199 -->

Nuisance Trap (Levels 1–4)\
Trigger: A creature crosses a trip wire\
Duration: Instantaneous\
A falling net trap uses a trip wire to release a\
weighted, 10-foot-square Net suspended from the\
ceiling. The trip wire is 3 inches off the ground and\
stretches between two columns or trees.\
The first creature that crosses the trip wire\
causes the Net to fall on it. The target must suc-\
ceed on a DC 10 Dexterity saving throw or have the\
Restrained condition until it escapes. The target\
succeeds automatically if it’s Huge or larger. A crea-\
ture can take an action to make a DC 10 Strength\
(Athletics) check, freeing itself or another creature\
within its reach from the Net on a successful check.\
Detect and Disarm. As a Search action, a creature\
can examine the trapped area and make a DC 11\
Wisdom (Perception) check, detecting the trip wire\
and the suspended Net on a successful check. Once\
detected, the trip wire can be easily cut or avoided\
(no ability check required).\
Destroy the Net. Reducing the Net to 0 Hit Points\
frees any creature trapped in it (see “Adventuring\
Gear” for the Net’s statistics).\
Set the Trap. A creature that has Thieves’ Tools\
and all the trap’s components (including a Net) can\
try to set a falling net trap, doing so with a success-\
ful DC 13 Dexterity (Sleight of Hand) check. Each\
attempt to set this trap takes 10 minutes.\
At Higher Levels. You can scale the trap for higher\
levels by increasing the weight of the Net, which\
increases the save DC and the DC of the Strength\
(Athletics) check as follows: DC 12 at levels 5–10, DC\
14 at levels 11–16, or DC 16 at levels 17–20.\
