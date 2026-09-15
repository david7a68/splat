---
splat_schema: 1
splat_id: 6082eb9f-8f75-5b5e-9a57-c69b64e83afc
splat_kind: item
splat_key: dnd2024:items/candle_of_invocation
splat_name: Candle of Invocation
splat_origin:
  key: dnd2024:reference/magic_item/candle_of_invocation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214, 215
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/candle_of_invocation
  implementation: dnd2024:item_candle_of_invocation
  parameters:
    activation:
      action: magic
      operation: light
    burn:
      total:
        value: 4
        unit: hour
      accounting_increment:
        value: 1
        unit: minute
      snuff_and_resume: true
      on_exhaustion: destroy_item
    lit_effects:
      light:
        kind: dim
        radius_ft: 30
      attuned_user_in_light:
        d20_tests: advantage
      cleric_or_druid_in_light:
        prepared_spells_level: 1
        spell_slot_cost: 0
    first_lighting_alternative:
      cast:
        spell:
          key: dnd2024:spells/gate
      destroys_item: true
      portal_destination:
        plane_selection:
        - gm_choice
        - d100
        table:
        - min: 1
          max: 5
          plane: abyss
        - min: 6
          max: 10
          plane: acheron
        - min: 11
          max: 17
          plane: arborea
        - min: 18
          max: 25
          plane: arcadia
        - min: 26
          max: 33
          plane: beastlands
        - min: 34
          max: 41
          plane: bytopia
        - min: 42
          max: 46
          plane: carceri
        - min: 47
          max: 54
          plane: elysium
        - min: 55
          max: 59
          plane: gehenna
        - min: 60
          max: 64
          plane: hades
        - min: 65
          max: 69
          plane: limbo
        - min: 70
          max: 77
          plane: mechanus
        - min: 78
          max: 85
          plane: mount_celestia
        - min: 86
          max: 90
          plane: nine_hells
        - min: 91
          max: 95
          plane: pandemonium
        - min: 96
          max: 100
          plane: ysgard
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_candle_of_invocation
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
  - spell.cast_completed
  capabilities:
  - equipment
  - item_resources
  - effects
  - item_spellcasting
  - spatial_queries
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 754265ee3a22be6c7a5e81d8ad0af776dd4ac9e1b681df7cb0f5e1908dcf35e2
  source_path: builtin/srd-5.2.1/magic_item/candle_of_invocation.md
  pages:
  - 214
  - 215
  engine_gaps: &id001
  - dnd2024:item_candle_of_invocation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: ef0feade2e1f64c1d1bf701e9b7b19b8cc540acc52a76bbccd464f74e8a1e411
splat_clauses:
- key: source_rules
  text: '# Candle of Invocation Wondrous Item, Very Rare (Requires Attunement) This candle’s magic is
    activated when the candle is lit, which requires a Magic action. After burning for 4 hours, the candle
    is destroyed. You can snuff it out early for use at a later time. Deduct the time it burned in increments
    of 1 minute from its total burn time. While lit, the candle sheds Dim Light in a 30-foot radius. While
    you are within that light, you have Advantage on D20 Tests. In addition, a Cleric or Druid in the
    light can cast level 1 spells they have prepared without expending spell slots. Alternatively, when
    you light the candle for the first time, you can cast Gate with it. Doing so destroys the candle.
    The portal created by the spell links to a particular Outer Plane chosen by the GM or determined by
    rolling on the following table. 1d100 Outer Plane 1d100 Outer Plane 01–05 Abyss 55–59 Gehenna 06–10
    Acheron 60–64 Hades 11–17 Arborea 65–69 Limbo 18–25 Arcadia 70–77 Mechanus 26–33 Beastlands 78–85
    Mount Celestia 34–41 Bytopia 86–90 Nine Hells 42–46 Carceri 91–95 Pandemonium 47–54 Elysium 96–00
    Ysgard'
  sha256: 2c2c3b18a0ddc8ff5865f5b39392b02435a9d45837126f5d6113326f2df17022
  status: contract_specified
splat_requires_mechanics: *id001
---

# Candle of Invocation

<!-- SRD page 214 -->

Wondrous Item, Very Rare (Requires Attunement)\
This candle’s magic is activated when the candle\
is lit, which requires a Magic action. After burning\
for 4 hours, the candle is destroyed. You can snuff\
it out early for use at a later time. Deduct the time\
it burned in increments of 1 minute from its total\
burn time.\
While lit, the candle sheds Dim Light in a 30-foot\
radius. While you are within that light, you have\
Advantage on D20 Tests. In addition, a Cleric or\
Druid in the light can cast level 1 spells they have\
prepared without expending spell slots.\
Alternatively, when you light the candle for the\
first time, you can cast Gate with it. Doing so de-\
stroys the candle. The portal created by the spell\

<!-- SRD page 215 -->

links to a particular Outer Plane chosen by the GM\
or determined by rolling on the following table.\
1d100 Outer Plane 1d100 Outer Plane\
01–05 Abyss 55–59 Gehenna\
06–10 Acheron 60–64 Hades\
11–17 Arborea 65–69 Limbo\
18–25 Arcadia 70–77 Mechanus\
26–33 Beastlands 78–85 Mount Celestia\
34–41 Bytopia 86–90 Nine Hells\
42–46 Carceri 91–95 Pandemonium\
47–54 Elysium 96–00 Ysgard\

Original layout: [[../../srd-5.2.1/pages/214|page 214]], [[../../srd-5.2.1/pages/215|page 215]].
