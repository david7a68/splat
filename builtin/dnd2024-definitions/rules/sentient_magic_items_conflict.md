---
splat_schema: 1
splat_id: a480fce7-ca46-5781-acf9-63d7ba8234a2
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_conflict
splat_name: Conflict
splat_origin:
  key: dnd2024:reference/pages/208
splat_sources:
- title: SRD 5.2.1
  locator: pp. 208
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sentient_magic_items_conflict
  parameters:
    conflict_trigger:
      bearer_opposes:
      - item_alignment
      - item_purpose
      conflict_occurrence_authority: gm
    initial_save:
      ability: charisma
      dc:
        op: add
        args:
        - 12
        - var: item.charisma_modifier
    failure_demands:
      count_min: 1
      selection: item_controller
      options:
      - pursue_item_goals_exclusively
      - dispose_of_repugnant_possessions
      - transfer_item_to_another_bearer
      - carry_or_wear_item_at_all_times
    noncompliance_responses:
      choice: item_controller
      options:
      - prohibit_bearer_attunement
      - suppress_selected_activated_properties
      - attempt_bearer_control
    control:
      save:
        ability: charisma
        dc:
          op: add
          args:
          - 12
          - var: item.charisma_modifier
      failure:
        condition: charmed
        duration:
          dice:
            count: 1
            sides: 12
          unit: hour
        must_try_obey_item_commands: true
      repeat_save:
        trigger: bearer_damage_taken
        success: end_control
      attempt_recharge: next_dawn
      recharge_required_on_success_or_failure: true
  implementation: dnd2024:rules_sentient_magic_items_conflict
splat_contracts:
- id: dnd2024:rules_sentient_magic_items_conflict
  inputs:
    bearer:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    item_choices:
      type: resolution
      required: true
  outputs:
  - demands
  - suppressed_properties
  - attunement_eligibility
  - control_effect
  events:
  - sentient_item.conflict
  - damage.applied
  - dawn
  capabilities:
  - sentient_items
  - conditions
  - saving_throws
  - attunement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 208
  engine_gaps: &id001
  - dnd2024:rules_sentient_magic_items_conflict
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 715b0e178bdd5c6dd7d3f3c5f97f376e62af5926ea35bb3a293b628ac58fa1be
  parent_name: Sentient Magic Items
  line_span:
    first: 10831
    count: 30
splat_clauses:
- key: source_rules
  text: '# Conflict When the bearer of a sentient item acts in a manner opposed to the item’s alignment
    or purpose, conflict can arise. When such a conflict occurs, the item’s bearer makes a Charisma saving
    throw (DC 12 plus the item’s Charisma modifier). On a failed save, the item makes one or more of the
    following demands: Chase My Dreams. The item demands that its bearer pursue the item’s goals to the
    exclusion of all other goals. Get Rid of It. The item demands that its bearer dispose of anything
    the item finds repugnant. It’s Time for a Change. The item demands to be given to someone else. Keep
    Me Close. The item insists on being carried or worn at all times. If its bearer refuses to comply
    with the item’s demands, the item can do any of the following: • Make it impossible for its bearer
    to attune to it. • Suppress one or more of its activated properties. • Attempt to take control of
    its bearer, whereupon the bearer makes a Charisma saving throw (DC 12 plus the item’s Charisma modifier).
    On a failed save, the bearer has the Charmed condition for 1d12 hours. While Charmed in this way,
    the bearer must try to follow the item’s commands. If the bearer takes damage, it repeats the save,
    ending the effect on a success. Whether or not the attempt to control its bearer succeeds, the item
    can’t use this power again until the next dawn.'
  sha256: fc4bcd8f039ce977d145ec5a663da607e8f12a8695fbf6b1bd3e1105ec1c6d44
  status: contract_specified
splat_requires_mechanics: *id001
---

# Conflict

<!-- SRD page 208 -->

When the bearer of a sentient item acts in a manner\
opposed to the item’s alignment or purpose, conflict\
can arise. When such a conflict occurs, the item’s\
bearer makes a Charisma saving throw (DC 12 plus\
the item’s Charisma modifier). On a failed save, the\
item makes one or more of the following demands:\
Chase My Dreams. The item demands that its\
bearer pursue the item’s goals to the exclusion of\
all other goals.\
Get Rid of It. The item demands that its bearer dis-\
pose of anything the item finds repugnant.\
It’s Time for a Change. The item demands to be\
given to someone else.\
Keep Me Close. The item insists on being carried or\
worn at all times.\
If its bearer refuses to comply with the item’s de-\
mands, the item can do any of the following:\
•  Make it impossible for its bearer to attune to it.\
•  Suppress one or more of its activated properties.\
•  Attempt to take control of its bearer, whereupon\
the bearer makes a Charisma saving throw (DC\
12 plus the item’s Charisma modifier). On a failed\
save, the bearer has the Charmed condition for\
1d12 hours. While Charmed in this way, the\
bearer must try to follow the item’s commands.\
If the bearer takes damage, it repeats the save,\
ending the effect on a success. Whether or not the\
attempt to control its bearer succeeds, the item\
can’t use this power again until the next dawn.\
