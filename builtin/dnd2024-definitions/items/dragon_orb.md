---
splat_schema: 1
splat_id: b30e8d90-84cb-5cf9-a633-b10541f71465
splat_kind: item
splat_key: dnd2024:items/dragon_orb
splat_name: Dragon Orb
splat_origin:
  key: dnd2024:reference/magic_item/dragon_orb
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: artifact
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/dragon_orb
  implementation: dnd2024:item_dragon_orb
  parameters:
    diameter_inches:
      idle: 10
      used: 20
    control:
      action: magic
      peer_into_depths: true
      save:
        ability: charisma
        dc: 15
        failure:
          condition: charmed
          source: orb
          duration_end: attunement_ends
          voluntary_end_attunement: false
          orb_spells:
          - spell:
              key: dnd2024:spells/suggestion
            save_dc: 18
            usage: at_will
            purpose:
              choice: gm
              category: evil_dragon_essence_objective
        success:
          control: true
          duration_end: attunement_ends
    charges:
      initial: 7
      maximum: 7
      recovery:
        event: time.dawn
        amount:
          roll:
            count: 1
            sides: 4
            add: 3
        cap_at_maximum: true
    spells_require_control: true
    spells:
    - spell:
        key: dnd2024:spells/cure_wounds
      level: 9
      charge_cost: 4
    - spell:
        key: dnd2024:spells/daylight
      charge_cost: 1
    - spell:
        key: dnd2024:spells/death_ward
      charge_cost: 2
    - spell:
        key: dnd2024:spells/detect_magic
      charge_cost: 0
    - spell:
        key: dnd2024:spells/scrying
      save_dc: 18
      charge_cost: 3
    call_dragons:
      action: magic
      requires_control: true
      telepathic_range_miles: 40
      targets: chromatic_dragons
      excluded: dragon_deities
      movement: as_soon_as_possible_most_direct_route
      attitude:
        choice: gm
        hostile_permitted: true
      cooldown:
        amount: 1
        unit: hours
    object_defense:
      ac: 20
      damage_destroys_from:
      - weapon_magic_bonus: 3
      - spell:
          key: dnd2024:spells/disintegrate
      immune_to_all_other_harm: true
    attunement_required: true
splat_contracts:
- id: dnd2024:item_dragon_orb
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
  - item.attunement_ended
  - spell.cast_pending
  - time.dawn
  - time.elapsed
  - item.damage_pending
  - item.attunement_completed
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - item_charges
  - effect_duration
  - item_recharge
  - damage
  - conditions
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1d98b13469052588e91c5c485f2294c75fe9c1ebdbf5fe562a4cb4c02fb2ea58
  source_path: builtin/srd-5.2.1/magic_item/dragon_orb.md
  pages:
  - 218
  engine_gaps: &id001
  - dnd2024:item_dragon_orb
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 69b17663d044fea8605a76145d1182164e3aae335bcf82b8a598f10067473f9b
splat_clauses:
- key: source_rules
  text: '# Dragon Orb Wondrous Item, Artifact (Requires Attunement) An orb is an etched crystal globe
    about 10 inches in diameter. When used, it grows to about 20 inches in diameter, and mist swirls inside
    it. While attuned to an orb, you can take a Magic action to peer into the orb’s depths. You must then
    make a DC 15 Charisma saving throw. On a successful save, you control the orb for as long as you remain
    attuned to it. On a failed save, the orb imposes the Charmed condition on you for as long as you remain
    attuned to it. While you are Charmed by the orb, you can’t voluntarily end your Attunement to it,
    and the orb casts Suggestion on you at will (save DC 18), urging you to work toward the evil ends
    it desires. The dragon essence within the orb might want many things: the annihilation of a particular
    society or organization, freedom from the orb, to spread suffering in the world, to advance the worship
    of Tiamat, or something else the GM decides. Spells. The orb has 7 charges and regains 1d4 + 3 expended
    charges daily at dawn. If you control the orb, you can cast one of the spells on the following table
    from it. The table indicates how many charges you must expend to cast the spell. Spell Charge Cost
    Cure Wounds (level 9 version) 4 Daylight 1 Death Ward 2 Detect Magic 0 Scrying (save DC 18) 3 Call
    Dragons. While you control the orb, you can take a Magic action to cause the orb to issue a telepathic
    call that extends in all directions for 40 miles. Chromatic dragons in range feel compelled to come
    to the orb as soon as possible by the most direct route. Dragon deities such as Tiamat are unaffected
    by this call. Chromatic dragons drawn to the orb might be Hostile toward you for compelling them against
    their will. Once you have used this property, it can’t be used again for 1 hour. Destroying an Orb.
    A Dragon Orb has AC 20 and is destroyed if it takes damage from a +3 Weapon or a Disintegrate spell.
    Nothing else can harm it.'
  sha256: 9e47217efb5f325746bedb5a985c30670740ae4946743037e52a08a83f6488be
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dragon Orb

<!-- SRD page 218 -->

Wondrous Item, Artifact (Requires Attunement)\
An orb is an etched crystal globe about 10 inches in\
diameter. When used, it grows to about 20 inches in\
diameter, and mist swirls inside it.\
While attuned to an orb, you can take a Magic\
action to peer into the orb’s depths. You must then\
make a DC 15 Charisma saving throw. On a success-\
ful save, you control the orb for as long as you re-\
main attuned to it. On a failed save, the orb imposes\
the Charmed condition on you for as long as you\
remain attuned to it.\
While you are Charmed by the orb, you can’t\
voluntarily end your Attunement to it, and the orb\
casts Suggestion on you at will (save DC 18), urging\
you to work toward the evil ends it desires. The\
dragon essence within the orb might want many\
things: the annihilation of a particular society or or-\
ganization, freedom from the orb, to spread suffer-\
ing in the world, to advance the worship of Tiamat,\
or something else the GM decides.\
Spells. The orb has 7 charges and regains 1d4 + 3\
expended charges daily at dawn. If you control the\
orb, you can cast one of the spells on the following\
table from it. The table indicates how many charges\
you must expend to cast the spell.\
Spell Charge\
Cost\
Cure Wounds (level 9 version) 4\
Daylight 1\
Death Ward 2\
Detect Magic 0\
Scrying (save DC 18) 3\
Call Dragons. While you control the orb, you\
can take a Magic action to cause the orb to issue a\
telepathic call that extends in all directions for 40\
miles. Chromatic dragons in range feel compelled\
to come to the orb as soon as possible by the most\
direct route. Dragon deities such as Tiamat are un-\
affected by this call. Chromatic dragons drawn to\
the orb might be Hostile toward you for compelling\
them against their will. Once you have used this\
property, it can’t be used again for 1 hour.\
Destroying an Orb. A Dragon Orb has AC 20 and is\
destroyed if it takes damage from a +3 Weapon or a\
Disintegrate spell. Nothing else can harm it.\

Original layout: [[../../srd-5.2.1/pages/218|page 218]].
