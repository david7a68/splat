---
splat_schema: 1
splat_id: 129a5122-8826-5942-890f-0418c4dad6e7
splat_kind: spell
splat_key: dnd2024:spells/call_lightning
splat_name: Call Lightning
splat_origin:
  key: dnd2024:reference/spell/call_lightning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 114
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: conjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/call_lightning
  implementation: dnd2024:spell_call_lightning
  parameters:
    cloud:
      shape: cylinder
      height_feet: 10
      radius_feet: 60
      origin_visible: true
      above_caster: true
    bolt:
      target: visible_point_under_cloud
      creatures_within_feet: 5
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 3
                sides: 10
                add: 0
            type: lightning
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
    triggers:
    - spell.cast_completed
    repeat:
      action: magic
      same_or_new_point: true
    existing_storm:
      requires:
      - outdoors
      - storm_at_cast
      replace_cloud_creation_with_storm_control: true
      bonus_damage:
        dice:
          count: 1
          sides: 10
          add: 0
    scaling:
      per_slot_above: 3
      damage_dice: 1
      sides: 10
splat_contracts:
- id: dnd2024:spell_call_lightning
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - cloud_effect
  - saves
  - damage
  - granted_action
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - spatial_areas
  - perception
  - relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: b29b5e740e30daa7fe051974226d84d11deaefd0aa56bb8c68db0fce4862e11d
  source_path: builtin/srd-5.2.1/spell/call_lightning.md
  pages:
  - 114
  engine_gaps: &id001
  - dnd2024:spell_call_lightning
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 58272aabc7bdf985bf71908078097b6f53747e53e4eedaec5bf97451f00071e8
splat_clauses:
- key: source_rules
  text: '# Call Lightning Level 3 Conjuration (Druid) Casting Time: Action Range: 120 feet Components:
    V, S Duration: Concentration, up to 10 minutes A storm cloud appears at a point within range that
    you can see above yourself. It takes the shape of a Cylinder that is 10 feet tall with a 60-foot radius.
    When you cast the spell, choose a point you can see under the cloud. A lightning bolt shoots from
    the cloud to that point. Each creature within 5 feet of that point makes a Dexterity saving throw,
    taking 3d10 Lightning damage on a failed save or half as much damage on a successful one. Until the
    spell ends, you can take a Magic action to call down lightning in that way again, targeting the same
    point or a different one. If you’re outdoors in a storm when you cast this spell, the spell gives
    you control over that storm instead of creating a new one. Under such conditions, the spell’s damage
    increases by 1d10. Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot
    level above 3.'
  sha256: e2aabe0f3aca38dd4cb529f698ca9f30885c0cbcb2e49ffb3a9db316c7e97e82
  status: contract_specified
splat_requires_mechanics: *id001
---

# Call Lightning

<!-- SRD page 114 -->

Level 3 Conjuration (Druid)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
A storm cloud appears at a point within range that\
you can see above yourself. It takes the shape of a\
Cylinder that is 10 feet tall with a 60-foot radius.\
When you cast the spell, choose a point you can\
see under the cloud. A lightning bolt shoots from the\
cloud to that point. Each creature within 5 feet of\
that point makes a Dexterity saving throw, taking\
3d10 Lightning damage on a failed save or half as\
much damage on a successful one.\
Until the spell ends, you can take a Magic action to\
call down lightning in that way again, targeting the\
same point or a different one.\
If you’re outdoors in a storm when you cast this\
spell, the spell gives you control over that storm in-\
stead of creating a new one. Under such conditions,\
the spell’s damage increases by 1d10.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d10 for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/114|page 114]].
