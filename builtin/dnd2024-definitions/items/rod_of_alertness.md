---
splat_schema: 1
splat_id: c112e39d-daba-5b50-88dc-43ce13020c4a
splat_kind: item
splat_key: dnd2024:items/rod_of_alertness
splat_name: Rod of Alertness
splat_origin:
  key: dnd2024:reference/magic_item/rod_of_alertness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 241
  url: https://www.dndbeyond.com/srd
splat_item:
  category: rod
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/rod_of_alertness
  implementation: dnd2024:item_rod_of_alertness
  parameters:
    while_held:
      advantage:
      - wisdom_perception
      - initiative
      spells:
      - spell:
          key: dnd2024:spells/detect_evil_and_good
        usage: at_will
      - spell:
          key: dnd2024:spells/detect_magic
        usage: at_will
      - spell:
          key: dnd2024:spells/detect_poison_and_disease
        usage: at_will
      - spell:
          key: dnd2024:spells/see_invisibility
        usage: at_will
    protective_aura:
      action: magic
      plant_haft_in_ground: true
      light:
        bright_radius_ft: 60
        additional_dim_ft: 60
      beneficiaries:
        relation:
        - self
        - ally
        within: rod_bright_light
      ac_bonus: 1
      save_bonus: 1
      sense_invisible_creature_location:
        creature_within: rod_bright_light
      duration:
        amount: 10
        unit: minutes
      ends_on:
        action: magic
        actor: any_creature
        operation: pull_rod_from_ground
      uses:
        maximum: 1
        recovery_event: time.dawn
    attunement_required: true
splat_contracts:
- id: dnd2024:item_rod_of_alertness
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
  - spell.cast_pending
  - time.dawn
  - time.elapsed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
  - effect_duration
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e6ee998ee2f365e7927cf9fc07f216aff2cee473b76031be251bf9736b425123
  source_path: builtin/srd-5.2.1/magic_item/rod_of_alertness.md
  pages:
  - 241
  engine_gaps: &id001
  - dnd2024:item_rod_of_alertness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 19c4011a0b8cf6d5d24e4ce0a32f31729f74da69596eca27590a8f9ebfd5c73e
splat_clauses:
- key: source_rules
  text: '# Rod of Alertness Rod, Very Rare (Requires Attunement) This rod has the following properties.
    Alertness. While holding the rod, you have Advantage on Wisdom (Perception) checks and on Initiative
    rolls. Spells. While holding the rod, you can cast the following spells from it: • Detect Evil and
    Good • Detect Magic • Detect Poison and Disease • See Invisibility Protective Aura. As a Magic action,
    you can plant the haft end of the rod in the ground, whereupon the rod’s head sheds Bright Light in
    a 60-foot radius and Dim Light for an additional 60 feet. While in that Bright Light, you and your
    allies gain a +1 bonus to Armor Class and saving throws and can sense the location of any Invisible
    creature that is also in the Bright Light. The rod’s head stops glowing and the effect ends after
    10 minutes or when a creature takes a Magic action to pull the rod from the ground. Once used, this
    property can’t be used again until the next dawn.'
  sha256: e299779d07d472e4459f5fa9fa54c21b3be7932e8a983c3c4968327527dc6f02
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rod of Alertness

<!-- SRD page 241 -->

Rod, Very Rare (Requires Attunement)\
This rod has the following properties.\
Alertness. While holding the rod, you have\
Advantage on Wisdom (Perception) checks and on\
Initiative rolls.\
Spells. While holding the rod, you can cast the fol-\
lowing spells from it:\
•  Detect Evil and Good\
•  Detect Magic\
•  Detect Poison and Disease\
•  See Invisibility\
Protective Aura. As a Magic action, you can plant\
the haft end of the rod in the ground, whereupon\
the rod’s head sheds Bright Light in a 60-foot radius\
and Dim Light for an additional 60 feet. While in\
that Bright Light, you and your allies gain a +1\
bonus to Armor Class and saving throws and can\
sense the location of any Invisible creature that is\
also in the Bright Light.\
The rod’s head stops glowing and the effect ends\
after 10 minutes or when a creature takes a Magic\
action to pull the rod from the ground. Once used,\
this property can’t be used again until the next\
dawn.\

Original layout: [[../../srd-5.2.1/pages/241|page 241]].
