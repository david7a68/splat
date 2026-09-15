---
splat_schema: 1
splat_id: 07492c50-9b13-5c0e-914d-7987f83a9f17
splat_kind: item
splat_key: dnd2024:items/sword_of_wounding
splat_name: Sword of Wounding
splat_origin:
  key: dnd2024:reference/magic_item/sword_of_wounding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/sword_of_wounding
  implementation: dnd2024:item_sword_of_wounding
  parameters:
    on_creature_hit:
      damage:
        amount:
          roll:
            count: 2
            sides: 6
        type: necrotic
      save:
        ability: constitution
        dc: 15
        failure:
          can_regain_hp: false
          duration:
            amount: 1
            unit: hours
        repeat:
          event: target.turn_end
          success: end_effect
    attunement_required: true
splat_contracts:
- id: dnd2024:item_sword_of_wounding
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
  - attack.hit
  - turn.ended
  - time.elapsed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - effect_duration
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 4a991fa8df00969dc4da640b2f9f2555a15b8fd20007b2a28a08f1d3d04b6c89
  source_path: builtin/srd-5.2.1/magic_item/sword_of_wounding.md
  pages:
  - 248
  engine_gaps: &id001
  - dnd2024:item_sword_of_wounding
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 251beada3748c06d0c5624046a2ac78a90029d1f25a7121255838a08a28df4ef
splat_clauses:
- key: source_rules
  text: '# Sword of Wounding Weapon (Glaive, Greatsword, Longsword, Rapier, Scimitar, or Shortsword),
    Rare (Requires Attunement) When you hit a creature with an attack using this magic weapon, the target
    takes an extra 2d6 Necrotic damage and must succeed on a DC 15 Constitution saving throw or be unable
    to regain Hit Points for 1 hour. The target repeats the save at the end of each of its turns, ending
    the effect on itself on a success.'
  sha256: 3f190153a5b5f7e9a349bf99fa218129bbda1bd99a13f9cf89e085332b63b1a4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sword of Wounding

<!-- SRD page 248 -->

Weapon (Glaive, Greatsword, Longsword, Rapier,\
Scimitar, or Shortsword), Rare (Requires Attunement)\
When you hit a creature with an attack using\
this magic weapon, the target takes an extra 2d6\
Necrotic damage and must succeed on a DC 15\
Constitution saving throw or be unable to regain Hit\
Points for 1 hour. The target repeats the save at the\
end of each of its turns, ending the effect on itself on\
a success.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]].
