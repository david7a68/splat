---
splat_schema: 1
splat_id: 70a6f59d-157f-51c2-9360-a2c43c96cd11
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_environmental_curses
splat_name: Environmental Curses
splat_origin:
  key: dnd2024:reference/pages/193
splat_sources:
- title: SRD 5.2.1
  locator: pp. 193
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magical_contagions_environmental_curses
  parameters:
    curse: demonic_possession
    exposure_contexts:
    - demonic_object_interaction
    - desecrated_location_lingering
    target_selection: gm
    initial_save:
      ability: charisma
      dc: 15
      failure: possessed_by_bodiless_demonic_entity
    while_possessed:
      trigger:
        d20_test_die_result: 1
      effect: entity_controls_creature_behavior
      regain_control_save:
        timing: end_of_each_later_turn
        ability: charisma
        dc: 15
        success: creature_regains_control
        curse_persists: true
    end_curse:
      long_rest_end_save:
        ability: charisma
        dc: 15
        success: curse_ends
      magic:
      - key: dnd2024:spells/dispel_evil_and_good
      - any_curse_removal_magic
  implementation: dnd2024:rule_magical_contagions_environmental_curses
splat_contracts:
- id: dnd2024:rule_magical_contagions_environmental_curses
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - effects
  events:
  - d20_test.rolled
  - turn.ended
  - rest.long_completed
  capabilities:
  - curses
  - possession
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 193
  engine_gaps: &id001
  - dnd2024:rule_magical_contagions_environmental_curses
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 0729ab013af633bba463f94dca87e2f0ad396a49ed044512002cabf799f31915
  parent_name: Magical Contagions
  line_span:
    first: 9482
    count: 25
splat_clauses:
- key: source_rules
  text: '# Environmental Curses Some locations are so suffused with evil that anyone who lingers there
    is burdened with a curse. Demonic Possession is one example of an environmental curse. Demonic Possession.
    Demonic Possession arises from the chaos and evil of the Abyss and commonly besets creatures that
    interact with demonic objects or linger in desecrated locations, where demonic spirits await victims.
    A creature that becomes the target of Demonic Possession must succeed on a DC 15 Charisma saving throw
    or be possessed by a bodiless demonic entity. Whenever the possessed creature rolls a 1 on a D20 Test,
    the demonic entity takes control of the creature and determines the creature’s behavior thereafter.
    At the end of each of the possessed creature’s later turns, the creature makes a DC 15 Charisma saving
    throw, regaining control of itself on a success. After finishing a Long Rest, a creature with Demonic
    Possession makes a DC 15 Charisma saving throw. On a successful save, the effect ends on the creature.
    A Dispel Evil and Good spell or any magic that removes a curse also ends the effect on it.'
  sha256: 6277c377517b95aac591a53bdf8c8723c6089a24df4a6c7b8f2591c02269d7bd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Environmental Curses

<!-- SRD page 193 -->

Some locations are so suffused with evil that any-\
one who lingers there is burdened with a curse.\
Demonic Possession is one example of an environ-\
mental curse.\
Demonic Possession. Demonic Possession arises\
from the chaos and evil of the Abyss and commonly\
besets creatures that interact with demonic objects\
or linger in desecrated locations, where demonic\
spirits await victims.\
A creature that becomes the target of Demonic\
Possession must succeed on a DC 15 Charisma sav-\
ing throw or be possessed by a bodiless demonic\
entity. Whenever the possessed creature rolls a 1\
on a D20 Test, the demonic entity takes control of\
the creature and determines the creature’s behav-\
ior thereafter. At the end of each of the possessed\
creature’s later turns, the creature makes a DC 15\
Charisma saving throw, regaining control of itself\
on a success.\
After finishing a Long Rest, a creature with De-\
monic Possession makes a DC 15 Charisma saving\
throw. On a successful save, the effect ends on the\
creature. A Dispel Evil and Good spell or any magic\
that removes a curse also ends the effect on it.\
