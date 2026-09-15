---
splat_schema: 1
splat_id: 44c00e76-f6b9-5f17-9c2b-03e4c383752b
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_sight_rot
splat_name: Sight Rot
splat_origin:
  key: dnd2024:reference/pages/194
splat_sources:
- title: SRD 5.2.1
  locator: pp. 194
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magical_contagions_sight_rot
  parameters:
    contagion: sight_rot
    water_exposure:
      creature_types:
      - beast
      - humanoid
      trigger: drink_tainted_water
      save:
        ability: constitution
        dc: 15
        failure: infected
    infected_effect:
      condition: blinded
      until: contagion_ends
    immediate_cures:
    - key: dnd2024:spells/heal
    - key: dnd2024:spells/lesser_restoration
    ointment:
      craft_tool:
        key: dnd2024:equipment/other_tools_herbalism_kit_5_gp
      proficiency_required: true
      tool_use_required: true
      hours_per_dose: 1
      doses_per_craft: 1
      application_location: eyes
      suppression_hours_per_dose: 24
      total_suppression_hours_to_cure: 72
      doses_and_applications_to_cure: 3
    skin_contact_spread:
      recipient_type: humanoid
      source: infected_creature
      save:
        ability: constitution
        dc: 15
        failure: infected
        success:
          immunity_to_same_source_hours: 24
  implementation: dnd2024:rule_magical_contagions_sight_rot
splat_contracts:
- id: dnd2024:rule_magical_contagions_sight_rot
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
  - water.drunk
  - creature.skin_contact
  capabilities:
  - contagions
  - conditions
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 194
  engine_gaps: &id001
  - dnd2024:rule_magical_contagions_sight_rot
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: a588b0ed7389ff45198d4d5671b64fe7dfcd06ca37fe80b7ce1811ff75481017
  parent_name: Magical Contagions
  line_span:
    first: 9586
    count: 24
splat_clauses:
- key: source_rules
  text: '# Sight Rot Magical Contagion Any Beast or Humanoid that drinks water tainted by Sight Rot must
    succeed on a DC 15 Constitution saving throw or have the Blinded condition until the contagion ends.
    Fighting the Contagion. Magic such as a Heal or Lesser Restoration spell ends the contagion immediately.
    A character who is proficient with an Herbalism Kit can use it to create one dose of nonmagical ointment,
    which takes 1 hour. When applied to the eyes of a creature suffering from Sight Rot, the ointment
    suppresses the contagion on that creature for 24 hours. If the contagion is suppressed in this way
    for a total of 72 hours (requiring three doses and applications of the ointment), the contagion ends
    on the creature. Spreading the Contagion. Any Humanoid that makes skin contact with a creature infected
    with Sight Rot must succeed on a DC 15 Constitution saving throw or also become infected with the
    contagion. On a successful save, the Humanoid can’t catch the contagion from that particular infected
    creature for the next 24 hours.'
  sha256: c6dcdf1045c17f0f59c5a7deb3327c4fcf7cbfa9ba0c1e26c1113daeac51804b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sight Rot

<!-- SRD page 194 -->

Magical Contagion\
Any Beast or Humanoid that drinks water tainted\
by Sight Rot must succeed on a DC 15 Constitution\
saving throw or have the Blinded condition until the\
contagion ends.\
Fighting the Contagion. Magic such as a Heal or\
Lesser Restoration spell ends the contagion immedi-\
ately. A character who is proficient with an Herbal-\
ism Kit can use it to create one dose of nonmagical\
ointment, which takes 1 hour. When applied to the\
eyes of a creature suffering from Sight Rot, the oint-\
ment suppresses the contagion on that creature for\
24 hours. If the contagion is suppressed in this way\
for a total of 72 hours (requiring three doses and\
applications of the ointment), the contagion ends on\
the creature.\
Spreading the Contagion. Any Humanoid that\
makes skin contact with a creature infected with\
Sight Rot must succeed on a DC 15 Constitution\
saving throw or also become infected with the con-\
tagion. On a successful save, the Humanoid can’t\
catch the contagion from that particular infected\
creature for the next 24 hours.\
