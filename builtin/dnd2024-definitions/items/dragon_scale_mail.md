---
splat_schema: 1
splat_id: e4a7c15f-aadc-550c-bb18-c86442166399
splat_kind: item
splat_key: dnd2024:items/dragon_scale_mail
splat_name: Dragon Scale Mail
splat_origin:
  key: dnd2024:reference/magic_item/dragon_scale_mail
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218, 219
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/dragon_scale_mail
  implementation: dnd2024:item_dragon_scale_mail
  parameters:
    equipment_state: worn
    armor_class_bonus: 1
    saving_throws:
      advantage_against: dragon_breath_weapons
    variants:
      black:
        resistance: acid
      blue:
        resistance: lightning
      brass:
        resistance: fire
      bronze:
        resistance: lightning
      copper:
        resistance: acid
      gold:
        resistance: fire
      green:
        resistance: poison
      red:
        resistance: fire
      silver:
        resistance: cold
      white:
        resistance: cold
    detection:
      action: magic
      resource:
        uses: 1
        recovery: dawn
      target: closest_dragon_matching_armor_type
      range_miles: 30
      outputs:
      - distance
      - direction
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_dragon_scale_mail
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
  source_sha256: a6adb3dac0e893bdad4c631be7b2bad2d16c9c37c521db8191bd87b5110faa28
  source_path: builtin/srd-5.2.1/magic_item/dragon_scale_mail.md
  pages:
  - 218
  - 219
  engine_gaps: &id001
  - dnd2024:item_dragon_scale_mail
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: fbaab62c4e841cc82a370a633764464e0971be9faf8efe70b2282cfb593c01a6
splat_clauses:
- key: source_rules
  text: '# Dragon Scale Mail Armor (Scale Mail), Very Rare (Requires Attunement) Dragon Scale Mail is
    made of the scales of one kind of dragon. Sometimes dragons collect their cast-off scales and gift
    them. Other times, hunters carefully preserve the hide of a dead dragon. In either case, Dragon Scale
    Mail is highly valued. While wearing this armor, you gain a +1 bonus to Armor Class, you have Advantage
    on saving throws against the breath weapons of Dragons, and you have Resistance to one damage type
    determined by the kind of dragon that provided the scales (see the accompanying table). Additionally,
    you can focus your senses as a Magic action to discern the distance and direction to the closest dragon
    within 30 miles of yourself that is of the same type as the armor. This action can’t be used again
    until the next dawn. Dragon Resistance Dragon Resistance Black Acid Gold Fire Blue Lightning Green
    Poison Brass Fire Red Fire Bronze Lightning Silver Cold Copper Acid White Cold'
  sha256: 855099b6a1ac27a9aa5a08a4a0622d85393269a0ff0409f42e6e6054945918de
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dragon Scale Mail

<!-- SRD page 218 -->

Armor (Scale Mail), Very Rare (Requires Attunement)\
Dragon Scale Mail is made of the scales of one kind\
of dragon. Sometimes dragons collect their cast-off\

<!-- SRD page 219 -->

scales and gift them. Other times, hunters carefully\
preserve the hide of a dead dragon. In either case,\
Dragon Scale Mail is highly valued.\
While wearing this armor, you gain a +1 bonus to\
Armor Class, you have Advantage on saving throws\
against the breath weapons of Dragons, and you\
have Resistance to one damage type determined by\
the kind of dragon that provided the scales (see the\
accompanying table).\
Additionally, you can focus your senses as a Magic\
action to discern the distance and direction to the\
closest dragon within 30 miles of yourself that is\
of the same type as the armor. This action can’t be\
used again until the next dawn.\
Dragon Resistance Dragon Resistance\
Black Acid Gold Fire\
Blue Lightning Green Poison\
Brass Fire Red Fire\
Bronze Lightning Silver Cold\
Copper Acid White Cold\

Original layout: [[../../srd-5.2.1/pages/218|page 218]], [[../../srd-5.2.1/pages/219|page 219]].
