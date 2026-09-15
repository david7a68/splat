---
splat_schema: 1
splat_id: 9dfe0dcf-e87a-58f6-9ba7-b10786b48c38
splat_kind: spell
splat_key: dnd2024:spells/fog_cloud
splat_name: Fog Cloud
splat_origin:
  key: dnd2024:reference/spell/fog_cloud
splat_sources:
- title: SRD 5.2.1
  locator: pp. 133
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: conjuration
  classes:
  - druid
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/fog_cloud
  implementation: dnd2024:spell_fog_cloud
  parameters:
    area:
      shape: sphere
      radius_feet: 20
      heavily_obscured: true
    end:
      strong_wind: disperse
    scaling:
      per_slot_above: 1
      radius_feet: 20
splat_contracts:
- id: dnd2024:spell_fog_cloud
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
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
  - fog_area
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - spatial_areas
splat_conversion:
  status: specified
  category: spell
  source_sha256: dbd26817cd76a6deb6edbbd5ae6e9f6a9b4f29ea8d530263fda974d0ddfe934d
  source_path: builtin/srd-5.2.1/spell/fog_cloud.md
  pages:
  - 133
  engine_gaps: &id001
  - dnd2024:spell_fog_cloud
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 3bd25eef97cf28f1be95810d444708e5f648cd88d8a634bb17ae189646ca2636
splat_clauses:
- key: source_rules
  text: '# Fog Cloud Level 1 Conjuration (Druid, Ranger, Sorcerer, Wizard) Casting Time: Action Range:
    120 feet Components: V, S Duration: Concentration, up to 1 hour You create a 20-foot-radius Sphere
    of fog centered on a point within range. The Sphere is Heavily Obscured. It lasts for the duration
    or until a strong wind (such as one created by Gust of Wind) disperses it. Using a Higher-Level Spell
    Slot. The fog’s radius increases by 20 feet for each spell slot level above 1.'
  sha256: 781bf6c4e5ffda0722f14685edfd044fe2db180dc75bfbf7e8f71f3cf099bcc5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fog Cloud

<!-- SRD page 133 -->

Level 1 Conjuration (Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 1 hour\
You create a 20-foot-radius Sphere of fog centered\
on a point within range. The Sphere is Heavily Ob-\
scured. It lasts for the duration or until a strong\
wind (such as one created by Gust of Wind) dis-\
perses it.\
Using a Higher-Level Spell Slot. The fog’s radius\
increases by 20 feet for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/133|page 133]].
