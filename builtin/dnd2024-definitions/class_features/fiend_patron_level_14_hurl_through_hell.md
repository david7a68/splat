---
splat_schema: 1
splat_id: b1148ee6-a4c1-5ce0-b1df-5b1068550c28
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_14_hurl_through_hell
splat_name: 'Level 14: Hurl Through Hell'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fiend_patron_level_14_hurl_through_hell
  parameters:
    trigger: attack_roll.hit_creature
    limit:
      count: 1
      period: turn
    save:
      ability: charisma
      dc:
        stat: spell_save_dc
      success: no_effect
      failure:
        banish_to: lower_planes
        damage:
          roll:
            dice:
              count: 8
              sides: 10
          type: psychic
          unless_creature_type: fiend
        condition: incapacitated
        until: source.next_turn_end
        return:
          destination: previous_space
          if_occupied: nearest_unoccupied_space
    uses:
      maximum: 1
      recover:
        event: rest.long_rest_completed
        amount: all
    restore_use:
      resource: pact_magic_spell_slot
      amount: 1
      action: none
  implementation: dnd2024:class_features_fiend_patron_level_14_hurl_through_hell
splat_contracts:
- id: dnd2024:class_features_fiend_patron_level_14_hurl_through_hell
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - resource_changes
  - damage
  - conditions
  events:
  - attack.hit
  - turn.ended
  capabilities:
  - planar_travel
  - conditions
  - damage
  - saving_throws
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 76
  engine_gaps: &id001
  - dnd2024:class_features_fiend_patron_level_14_hurl_through_hell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 86e82fa377432c76ccc095246aaf70c92c3dd13d65009d8bdc27c723a5122ff4
  parent_name: Fiend Patron
  line_span:
    first: 6514
    count: 15
splat_clauses:
- key: source_rules
  text: '# Level 14: Hurl Through Hell Once per turn when you hit a creature with an attack roll, you
    can try to instantly transport the target through the Lower Planes. The target must succeed on a Charisma
    saving throw against your spell save DC, or the target disappears and hurtles through a nightmare
    landscape. The target takes 8d10 Psychic damage if it isn’t a Fiend, and it has the Incapacitated
    condition until the end of your next turn, when it returns to the space it previously occupied or
    the nearest unoccupied space. Once you use this feature, you can’t use it again until you finish a
    Long Rest unless you expend a Pact Magic spell slot (no action required) to restore your use of it.'
  sha256: 2c6919f41d448ac936cbd51a99c380525dbd957b133966c50ef88cd96871e7c3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Hurl Through Hell

<!-- SRD page 76 -->

Once per turn when you hit a creature with an\
attack roll, you can try to instantly transport the\
target through the Lower Planes. The target must\
succeed on a Charisma saving throw against your\
spell save DC, or the target disappears and hurtles\
through a nightmare landscape. The target takes\
8d10 Psychic damage if it isn’t a Fiend, and it has the\
Incapacitated condition until the end of your next\
turn, when it returns to the space it previously oc-\
cupied or the nearest unoccupied space.\
Once you use this feature, you can’t use it again\
until you finish a Long Rest unless you expend a\
Pact Magic spell slot (no action required) to restore\
your use of it.\
