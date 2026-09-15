---
splat_schema: 1
splat_id: 561c2107-8c05-5ef6-96cf-f0c76151a8c0
splat_kind: item
splat_key: dnd2024:items/rod_of_rulership
splat_name: Rod of Rulership
splat_origin:
  key: dnd2024:reference/magic_item/rod_of_rulership
splat_sources:
- title: SRD 5.2.1
  locator: pp. 242
  url: https://www.dndbeyond.com/srd
splat_item:
  category: rod
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/rod_of_rulership
  implementation: dnd2024:item_rod_of_rulership
  parameters:
    action: magic
    activation: present_rod_and_command_obedience
    targets:
      choice: user
      visible: true
      range_ft: 120
    save:
      ability: wisdom
      dc: 15
      failure:
        condition: charmed
        duration:
          amount: 8
          unit: hours
        regards_user_as: trusted_leader
        ends_on:
        - harmed_by_user
        - harmed_by_user_ally
        - command_contrary_to_nature
    uses:
      maximum: 1
      recovery_event: time.dawn
    attunement_required: true
splat_contracts:
- id: dnd2024:item_rod_of_rulership
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - item.activation_requested
  - participant.harmed
  - command.issued
  - time.dawn
  - time.elapsed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - effect_duration
  - spatial_targeting
  - conditions
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 10027fdd2d0164b69d43d340d1c54fa99bcafe4df35704b3695663fb6057a3b2
  source_path: builtin/srd-5.2.1/magic_item/rod_of_rulership.md
  pages:
  - 242
  engine_gaps: &id001
  - dnd2024:item_rod_of_rulership
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: bd17e960f0e8ad764e3e0456f5088190cab0190f05187d80e82348528ebb38ac
splat_clauses:
- key: source_rules
  text: '# Rod of Rulership Rod, Rare (Requires Attunement) You can take a Magic action to present the
    rod and command obedience from each creature of your choice that you can see within 120 feet of yourself.
    Each target must succeed on a DC 15 Wisdom saving throw or have the Charmed condition for 8 hours.
    While Charmed in this way, the creature regards you as its trusted leader. If harmed by you or your
    allies or commanded to do something contrary to its nature, a target ceases to be Charmed in this
    way. Once used, this property can’t be used again until the next dawn.'
  sha256: 12b42673569ced0c3864d9dab49760e3b17aa79a5c4fc93992e2dcc2c296184e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rod of Rulership

<!-- SRD page 242 -->

Rod, Rare (Requires Attunement)\
You can take a Magic action to present the rod and\
command obedience from each creature of your\
choice that you can see within 120 feet of your-\
self. Each target must succeed on a DC 15 Wisdom\
saving throw or have the Charmed condition for 8\
hours. While Charmed in this way, the creature re-\
gards you as its trusted leader. If harmed by you or\
your allies or commanded to do something contrary\
to its nature, a target ceases to be Charmed in this\
way. Once used, this property can’t be used again\
until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/242|page 242]].
