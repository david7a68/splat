---
splat_schema: 1
splat_id: 523ab00e-fd3a-59a1-a53f-1698a5d55d46
splat_kind: item
splat_key: dnd2024:items/rod_of_security
splat_name: Rod of Security
splat_origin:
  key: dnd2024:reference/magic_item/rod_of_security
splat_sources:
- title: SRD 5.2.1
  locator: pp. 242
  url: https://www.dndbeyond.com/srd
splat_item:
  category: rod
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/rod_of_security
  implementation: dnd2024:item_rod_of_security
  parameters:
    equipment_state: held
    activation:
      action: magic
      recovery:
        value: 10
        unit: day
      transport:
        self: true
        other_targets:
          maximum: 199
          willing: true
          seen: true
        destination: created_demiplane
    demiplane:
      form_choice:
        chooser: user
        kind: imagined_environment
        constraints:
        - sufficient_food_and_water
        - environment_cannot_harm_occupants
        - native_objects_cannot_leave
      food_and_water: sufficient_for_all_visitors
      environment_cannot_harm_occupants: true
      native_interactable_objects:
        exist_only_inside: true
        on_removal: disappear
      hourly_healing:
        per_visitor: as_one_hit_point_die_spent
        hit_point_die_cost: 0
      aging: false
      time_passage: normal
      maximum_stay_days:
        operation: floor_divide
        numerator: 200
        denominator: creatures_present
    return_transport:
      triggers:
      - duration_expires
      - user_magic_action_end
      targets: all_visitors
      destination: original_activation_location_per_visitor
      if_occupied: nearest_unoccupied_space
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_rod_of_security
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
  - spatial_queries
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8755b4afec5939b4ba0146e6e91ff6faaf5e86958a28954d504b708999374343
  source_path: builtin/srd-5.2.1/magic_item/rod_of_security.md
  pages:
  - 242
  engine_gaps: &id001
  - dnd2024:item_rod_of_security
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 93a74b0c71599ab1beca8ea5fdeb13c8b1fcfa0e6b816c1e65ed4c9e9c9869a9
splat_clauses:
- key: source_rules
  text: '# Rod of Security Rod, Very Rare While holding this rod, you can take a Magic action to activate
    it. The rod then instantly transports you and up to 199 other willing creatures you can see to a demiplane.
    You choose the form the demiplane takes. It could be a tranquil garden, a cheery tavern, an immense
    palace, a tropical island, a fantastic carnival, or whatever else you can imagine. Regardless of its
    nature, the demiplane contains enough water and food to sustain its visitors, and the demiplane’s
    environment can’t harm its occupants. Everything else that can be interacted with there can exist
    only there. For example, a flower picked from a garden there disappears if it is taken outside the
    demiplane. For each hour spent in the demiplane, a visitor regains Hit Points as if it had spent 1
    Hit Point Die. Also, creatures don’t age while there, although time passes normally. Visitors can
    remain there for up to 200 days divided by the number of creatures present (round down). When the
    time runs out or you take a Magic action to end the effect, all visitors reappear in the location
    they occupied when you activated the rod or an unoccupied space nearest that location. Once used,
    this property can’t be used again until 10 days have passed.'
  sha256: bce243aee1d956117233712fb21a3949713ef8a7ec396ee4a162c0b6fb99f208
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rod of Security

<!-- SRD page 242 -->

Rod, Very Rare\
While holding this rod, you can take a Magic action\
to activate it. The rod then instantly transports you\
and up to 199 other willing creatures you can see\
to a demiplane. You choose the form the demiplane\
takes. It could be a tranquil garden, a cheery tavern,\
an immense palace, a tropical island, a fantastic car-\
nival, or whatever else you can imagine. Regardless\
of its nature, the demiplane contains enough water\
and food to sustain its visitors, and the demiplane’s\
environment can’t harm its occupants. Everything\
else that can be interacted with there can exist\
only there. For example, a flower picked from a\
garden there disappears if it is taken outside the\
demiplane.\
For each hour spent in the demiplane, a visitor\
regains Hit Points as if it had spent 1 Hit Point Die.\
Also, creatures don’t age while there, although time\
passes normally. Visitors can remain there for up to\
200 days divided by the number of creatures pres-\
ent (round down).\
When the time runs out or you take a Magic action\
to end the effect, all visitors reappear in the loca-\
tion they occupied when you activated the rod or an\
unoccupied space nearest that location. Once used,\
this property can’t be used again until 10 days have\
passed.\

Original layout: [[../../srd-5.2.1/pages/242|page 242]].
